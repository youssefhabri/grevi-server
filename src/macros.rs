#[macro_export]
macro_rules! map_to_field_result {
    ($result:expr) => {
        ($result).map_err(|err| FieldError::from(err.to_string()))
    };
}
