use validator::Validate;

use crate::error::AppError;

pub fn validate<T: Validate>(value: T) -> Result<T, AppError> {
    value.validate()?;
    Ok(value)
}
