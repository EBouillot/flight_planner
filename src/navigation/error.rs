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

#[derive(thiserror::Error, Debug)]
pub enum BalanceError {
    #[error("Element is not in balance sheet")]
    not_in_balance,
    #[error("Tank capacity exceeded (max {0})")]
    tank_capacity_exceeded(i32),
    #[error("Unknown error")]
    unknown_error,
}