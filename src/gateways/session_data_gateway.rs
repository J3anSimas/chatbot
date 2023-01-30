use crate::entities::session;
pub trait SessionDataGateway {
    fn get_active_session_by_user_id(&self, user_id: &str) -> Option<session::Session>;
    fn get_session_by_id(&self, id: &str) -> Option<session::Session>;
    fn create_session(&mut self, user_id: &str) -> session::Session;
    fn update_session(&self, session: session::Session, option: usize) -> session::Session;
    fn inactive_session(&mut self, session: session::Session) -> session::Session;
}
