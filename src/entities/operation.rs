pub enum OperationTypes {
    Send(String),
    SendAndWaitForReply(String),
    MakeHTTPRequest(String),
}
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
}