use crate::{
    entities::{
        operation::{Operation, OperationTypes},
        session::Session,
    },
    gateways::session_data_gateway::SessionDataGateway,
};

pub struct UserSendsMessageToBot {
    session_data_gateway: Box<dyn SessionDataGateway>,
}

impl UserSendsMessageToBot {
    pub fn new(session_data_gateway: Box<dyn SessionDataGateway>) -> Self {
        Self {
            session_data_gateway: session_data_gateway,
        }
    }

    fn handle(&self, user_id: &str, message: &str) -> Session {
        let mut session = match self
            .session_data_gateway
            .get_active_session_by_user_id(user_id)
        {
            Option::Some(session) => {
                let option: usize = message.parse().expect("Failed to parse message");
                self.session_data_gateway.update_session(session, option)
            }
            Option::None => return self.session_data_gateway.create_session(user_id),
        };

        return session;
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        entities::{
            operation::{Operation, OperationTypes},
            session::Session,
        },
        gateways::session_data_gateway::SessionDataGateway,
    };

    use super::UserSendsMessageToBot;

    struct SessionDataGatewaySpy {}

    impl SessionDataGatewaySpy {
        fn new() -> Self {
            Self {}
        }
    }
    impl SessionDataGateway for SessionDataGatewaySpy {
        fn get_active_session_by_user_id(
            &self,
            user_id: &str,
        ) -> Option<crate::entities::session::Session> {
            return None;
        }
        fn create_session(&self, user_id: &str) -> crate::entities::session::Session {
            let operation = Operation::new(
                "1",
                "Titulo",
                OperationTypes::SendAndWaitForReply("test".to_string()),
                vec![],
            );
            return Session::new("new_id", operation, user_id, true);
        }
        fn update_session(
            &self,
            mut session: Session,
            option: usize,
        ) -> crate::entities::session::Session {
            let mut operation = session.current_operation().get_selected_option(option);
            session.set_current_operation(operation);
            return session;
        }
    }
    #[test]
    fn it_should_create_a_session_if_user_have_no_active_session() {
        let session_data_gateway_spy = SessionDataGatewaySpy::new();
        let usecase = UserSendsMessageToBot::new(Box::new(session_data_gateway_spy));
        let response = usecase.handle("teste", "teste");
        assert_eq!(response.id(), "new_id");
    }

    #[test]
    fn it_should_advance_to_next_operation_based_on_user_message() {
        let session_data_gateway_spy = SessionDataGatewaySpy::new();
        let usecase = UserSendsMessageToBot::new(Box::new(session_data_gateway_spy));
        let response_session = usecase.handle("teste", "teste");
        let next_oper = &response_session.current_operation().operation_options()[0];
        let response_session = usecase.handle("teste", "1");
        assert_eq!(response_session.current_operation().id(), next_oper.id());
    }
}
