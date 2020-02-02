use jmespath::JmespathError;
use serde_json::Error as SerdeJsonError;


#[derive(Debug)]
pub enum ErrorType {
    CliError,
    JmespathError,
    SerdeJsonError
}

#[derive(Debug)]
pub struct CustomError {
    pub message: String,
    pub kind: ErrorType,
}

impl CustomError {
    pub fn cli_error(message: &str) -> Self {
        Self {
            message: message.to_owned(),
            kind: ErrorType::CliError
        }
    }
}

impl From<SerdeJsonError> for CustomError {
    fn from(err: SerdeJsonError) -> Self {
        Self {
            message: format!("{}", err),
            kind: ErrorType::SerdeJsonError,
        }
    }
}

impl From<JmespathError> for CustomError {
    fn from(err: JmespathError) -> Self {
        Self {
            message: format!("{}", err),
            kind: ErrorType::JmespathError,
        }
    }
}
