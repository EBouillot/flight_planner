#[derive(thiserror::Error, Debug)]
pub enum ParseError {
    #[error("File not found")]
    file_not_found,
    #[error("Wrong file format")]
    wrong_format,
    #[error("Field not found")]
    field_not_found,
    #[error("Unknown error")]
    unknown_error,
}