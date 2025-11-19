use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::{Validate, ValidationError};
use crate::validators::validate_password;

#[derive(Deserialize, Serialize, Validate, ToSchema)]
#[validate(schema(function = "validate_passwords_match"))]
pub struct UserRegistration {
    #[validate(length(min = 1, max = 50, message = "First Name must be between 3 and 50 characters"))]
    pub first_name: String,
    #[validate(length(min = 1, max = 50, message = "Last Name must be between 3 and 50 characters"))]
    pub last_name: Option<String>,
    #[validate(email(message = "Invalid email format"))]
    pub email: String,

    #[validate(length(min = 1, max = 50, message = "Username must be between 3 and 50 characters"))]
    pub username: String,

    #[validate(custom(function = "validate_password"))]
    pub password: String,
    #[validate(custom(function = "validate_password"))]
    pub confirm_password: String,
}

fn validate_passwords_match(user: &UserRegistration) -> Result<(), ValidationError> {
    if user.password != user.confirm_password {
        let mut err = ValidationError::new("password_mismatch");
        err.message = Some("Password and confirmation password do not match".into());
        err.add_param("field".into(), &"confirm_password");
        return Err(err);
    }
    Ok(())
}