use super::operation;

pub struct Session {
    id: String,
    current_operation: operation::Operation,
    user_id: String,
    active: bool,
}

impl Session {
    pub fn new(
        id: &str,
        current_operation: operation::Operation,
        user_id: &str,
        active: bool,
    ) -> Self {
        Self {
            id: id.to_string(),
            current_operation,
            user_id: user_id.to_string(),
            active,
        }
    }

    pub fn id(&self) -> &str {
        self.id.as_ref()
    }

    pub fn current_operation(&self) -> &operation::Operation {
        &self.current_operation
    }

    pub fn set_current_operation(&mut self, current_operation: operation::Operation) {
        self.current_operation = current_operation;
    }
}
