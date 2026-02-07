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
