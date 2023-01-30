#[derive(Clone)]
pub enum OperationTypes {
    Send(String),
    SendAndWaitForReply(String),
    MakeHTTPRequest(String),
}
#[derive(Clone)]
pub struct Operation {
    id: String,
    title: String,
    operation_type: OperationTypes,
    operation_options: Vec<Operation>,
}

impl Operation {
    pub fn new(
        id: &str,
        title: &str,
        operation_type: OperationTypes,
        operation_options: Vec<Operation>,
    ) -> Self {
        Self {
            id: id.to_string(),
            title: title.to_string(),
            operation_type,
            operation_options,
        }
    }

    pub fn operation_options(&self) -> &[Operation] {
        self.operation_options.as_ref()
    }

    pub fn id(&self) -> &str {
        self.id.as_ref()
    }
    pub fn get_selected_option(&self, option_index: usize) -> Operation {
        let operation = self.operation_options[option_index].clone();
        return operation;
    }
    pub fn execute(&self) -> (String, bool) {
        match &self.operation_type {
            OperationTypes::Send(message) => (message.clone(), true),
            OperationTypes::SendAndWaitForReply(message) => (message.clone(), false),
            OperationTypes::MakeHTTPRequest(url) => (url.clone(), true),
        }
    }
}
