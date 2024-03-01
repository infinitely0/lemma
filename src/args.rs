use std::{
    error::Error,
    fmt::{self, Display, Formatter},
    fs,
};

#[derive(Debug)]
pub struct Args {
    pub file_path: String,
}

impl Args {
    pub fn build(args: &mut impl Iterator<Item = String>) -> Result<Args, ApplicationError> {
        args.next(); // Program name

        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err(ApplicationError::Args("no file provided".to_string())),
        };

        Ok(Args { file_path })
    }

    pub fn source(&self) -> Result<String, ApplicationError> {
        match fs::read_to_string(&self.file_path) {
            Ok(source) => Ok(source),
            Err(error) => {
                let message = format!(
                    "could not read file `{}` ({})",
                    &self.file_path,
                    error.kind()
                );
                Err(ApplicationError::Args(message))
            }
        }
    }
}

#[derive(PartialEq, Debug)]
pub enum ApplicationError {
    Args(String),
}

impl Error for ApplicationError {}

impl Display for ApplicationError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            ApplicationError::Args(e) => write!(f, "Invalid program arguments: {}", e),
        }
    }
}
