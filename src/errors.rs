use std::fmt::Display;

///
/// Common Errors
///
#[derive(Debug)]
pub enum TaskError {
    Common(String),
    Parse,
}

impl Display for TaskError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Common(e) => write!(f, "{e}"),
            Self::Parse => write!(f, "Invalid parameters combination."),
        }
    }
}
