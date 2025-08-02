use crate::AppError;

/// Validate email format
pub fn validate_email(email: &str) -> Result<(), AppError> {
    if email.is_empty() {
        return Err(AppError::Validation("Email cannot be empty".to_string()));
    }

    if !email.contains('@') {
        return Err(AppError::Validation("Invalid email format".to_string()));
    }

    Ok(())
}

/// Validate password strength
pub fn validate_password(password: &str) -> Result<(), AppError> {
    if password.len() < 8 {
        return Err(AppError::Validation(
            "Password must be at least 8 characters".to_string(),
        ));
    }

    Ok(())
}

/// Validate text length
pub fn validate_text_length(
    text: &str,
    min: usize,
    max: usize,
    field_name: &str,
) -> Result<(), AppError> {
    if text.len() < min {
        return Err(AppError::Validation(format!(
            "{field_name} must be at least {min} characters"
        )));
    }

    if text.len() > max {
        return Err(AppError::Validation(format!(
            "{field_name} must be at most {max} characters"
        )));
    }

    Ok(())
}
