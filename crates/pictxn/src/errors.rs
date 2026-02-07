pub type CoreResult<T> = Result<T, CoreError>;

#[derive(Debug, derive_more::From)]
pub enum CoreError {
    User(UserError),
    Card(CardError),
    File(FileError),

    Io(std::io::Error),
}

#[derive(Debug)]
pub enum UserError {
    NotFound,

    UsernameTaken,
    InvalidUsername,
}

#[derive(Debug)]
pub enum CardError {
    NotFound,
}

#[derive(Debug)]
pub enum FileError {
    NotFound,

    InvalidResolution,
    InvalidMimetype,
    FileTooLarge,
}
