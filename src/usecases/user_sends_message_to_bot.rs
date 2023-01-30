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

    fn handle(&mut self, user_id: &str, message: &str) -> String {
        let mut session = match self
            .session_data_gateway
            .get_active_session_by_user_id(user_id)
        {
            Option::Some(session) => {
                let option: usize = message.parse().expect("Failed to parse message");
                let session = self
                    .session_data_gateway
                    .update_session(session, option - 1);
                let (message, should_inactivate_session) = session.current_operation().execute();
                if should_inactivate_session {
                    self.session_data_gateway.inactive_session(session);
                }
                return message;
            }
            Option::None => {
                let session = self.session_data_gateway.create_session(user_id).clone();
                let (message, should_inactivate_session) = session.current_operation().execute();
                if should_inactivate_session {
                    self.session_data_gateway.inactive_session(session);
                }
                return message;
            }
        };
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

    #[derive(Clone)]
    struct SessionDataGatewaySpy {
        session_array: Vec<Session>,
    }

    impl SessionDataGatewaySpy {
        fn new(session_array: Vec<Session>) -> Self {
            Self { session_array }
        }

        fn set_session_array(&mut self, session_array: Vec<Session>) {
            self.session_array = session_array;
        }
    }
    impl SessionDataGateway for SessionDataGatewaySpy {
        fn get_active_session_by_user_id(
            &self,
            user_id: &str,
        ) -> Option<crate::entities::session::Session> {
            for session in self.session_array.iter() {
                if session.user_id() == user_id && session.active() {
                    return Some(session.clone());
                }
            }
            return None;
        }
        fn get_session_by_id(&self, id: &str) -> Option<crate::entities::session::Session> {
            for session in &self.session_array {
                println!("current id: {}", session.id());
                if session.id() == id {
                    return Some(session.clone());
                }
            }
            return None;
        }
        fn create_session(&mut self, user_id: &str) -> crate::entities::session::Session {
            let operation = Operation::new(
                "1",
                "Titulo",
                OperationTypes::SendAndWaitForReply("test".to_string()),
                vec![Operation::new(
                    "next_id",
                    "titulo teste",
                    OperationTypes::Send("test response".to_string()),
                    vec![],
                )],
            );
            let new_session = Session::new("1", operation, user_id, true);
            self.session_array.push(new_session.clone());

            return new_session;
        }
        fn update_session(
            &self,
            mut session: Session,
            option: usize,
        ) -> crate::entities::session::Session {
            let operation = session.current_operation().get_selected_option(option);
            session.set_current_operation(operation);
            return session;
        }
        fn inactive_session(
            &mut self,
            mut session: crate::entities::session::Session,
        ) -> crate::entities::session::Session {
            for current_session in &mut self.session_array {
                if current_session.id() == session.id() {
                    current_session.set_active(false);
                }
            }
            session.set_active(false);
            return session;
        }
    }
    #[test]
    fn it_should_create_a_session_if_user_have_no_active_session() {
        let sessions_array: Vec<Session> = Vec::new();
        let session_data_gateway_spy = SessionDataGatewaySpy::new(sessions_array);
        let mut usecase = UserSendsMessageToBot::new(Box::new(session_data_gateway_spy));
        usecase.handle("id_create", "teste");
        let session_data_gateway_spy = SessionDataGatewaySpy::new(sessions_array);
        let session = match session_data_gateway_spy.get_session_by_id("1") {
            Some(session) => session,
            None => {
                panic!("Não foi possivel encontrar sessao");
            }
        };
        println!("sessao {}", session.id());
        assert_eq!(session.id(), "1");
    }

    // #[test]
    // fn it_should_advance_to_next_operation_based_on_user_message() {
    //     let sessions_array: Vec<Session> = Vec::new();
    //     let session_data_gateway_spy = SessionDataGatewaySpy::new(sessions_array.clone());
    //     let mut usecase = UserSendsMessageToBot::new(Box::new(session_data_gateway_spy));
    //     usecase.handle("teste", "0");
    //     let session_data_gateway_spy = SessionDataGatewaySpy::new(sessions_array);
    //     let session = session_data_gateway_spy.get_session_by_id("1").unwrap();
    //     let next_oper = &session.current_operation().operation_options()[0];
    //     usecase.handle("teste", "1");
    //     let session = session_data_gateway_spy.get_session_by_id("1").unwrap();
    //     assert_eq!(session.current_operation().id(), next_oper.id());
    // }
    // #[test]
    // fn it_should_execute_operation_selected() {
    //     let sessions_array: Vec<Session> = Vec::new();
    //     let session_data_gateway_spy = SessionDataGatewaySpy::new(sessions_array);
    //     let mut usecase = UserSendsMessageToBot::new(Box::new(session_data_gateway_spy));
    //     let response = usecase.handle("teste", "0");
    //     assert_eq!(response, "test response".to_string());
    // }
}
