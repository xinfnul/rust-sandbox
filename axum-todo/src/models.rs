use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationError};

fn validate_title(title: &str) -> Result<(), ValidationError> {
    let title = title.trim();

    if title.is_empty() {
        return Err(ValidationError::new("title_empty"));
    }

    if title.chars().count() > 120 {
        return Err(ValidationError::new("title_too_long"));
    }

    Ok(())
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Todo {
    pub id: u64,
    pub title: String,
    pub description: String,
    pub completed: bool,
}

#[derive(Debug, Deserialize, Validate)]
#[serde(deny_unknown_fields)]
pub struct CreateTodo {
    #[validate(custom(function = "validate_title"))]
    pub title: String,

    #[validate(length(max = 2000))]
    pub description: String,
}

#[derive(Debug, Deserialize, Validate)]
#[serde(deny_unknown_fields)]
pub struct UpdateTodo {
    #[validate(custom(function = "validate_title"))]
    pub title: String,

    #[validate(length(max = 2000))]
    pub description: String,

    pub completed: bool,
}
