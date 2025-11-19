use regex::Regex;
use validator::ValidationError;
use std::borrow::Cow;

pub fn validate_password(pwd: &str) -> Result<(), ValidationError> {
    if pwd.len() < 8 {
        let mut err = ValidationError::new("password_too_short");
        err.message = Some(Cow::from("Password must be at least 8 characters"));
        return Err(err);
    }

    let has_lowercase = Regex::new(r"[a-z]").unwrap();
    if !has_lowercase.is_match(pwd) {
        let mut err = ValidationError::new("password_weak");
        err.message = Some(Cow::from("Password must contain at least one lowercase letter"));
        return Err(err);
    }

    let has_uppercase = Regex::new(r"[A-Z]").unwrap();
    if !has_uppercase.is_match(pwd) {
        let mut err = ValidationError::new("password_weak");
        err.message = Some(Cow::from("Password must contain at least one uppercase letter"));
        return Err(err);
    }

    let has_digit = Regex::new(r"\d").unwrap();
    if !has_digit.is_match(pwd) {
        let mut err = ValidationError::new("password_weak");
        err.message = Some(Cow::from("Password must contain at least one number"));
        return Err(err);
    }

    let has_symbol = Regex::new(r#"[!@#$%^&*()_\-=\[\]{};':"\\|,.<>/?]"#).unwrap();
    if !has_symbol.is_match(pwd) {
        let mut err = ValidationError::new("password_weak");
        err.message = Some(Cow::from("Password must contain at least one symbol character"));
        return Err(err);
    }
    
    Ok(())
}