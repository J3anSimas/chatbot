use crate::entities::session;
pub trait SessionDataGateway {
    fn get_active_session_by_user_id(&self, user_id: &str) -> Option<session::Session>;
    fn create_session(&self, user_id: &str) -> session::Session;
}
