use std::fmt;

#[derive(Debug)]
pub enum AppError {
    LocalDireNotResolved,
    DatabaseNotLoaded,
    AppDirectoryError,
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            AppError::LocalDireNotResolved => write!(f, "User Local Directory Path Not Resolved"),
            AppError::DatabaseNotLoaded => write!(f, "Database File Not Loaded"),
            AppError::AppDirectoryError => write!(f, "Application Directory creation error"),            
        }
    }
}