// Copyright (c) 2025 AI4Thai Crop Guardian
// Licensed under the MIT License

//! Validation utility functions for user input

/// Validate text input
pub fn validate_text_input(text: &str) -> Result<(), String> {
    if text.trim().is_empty() {
        return Err("Text cannot be empty".to_string());
    }
    
    if text.len() > 1000 {
        return Err("Text too long".to_string());
    }
    
    Ok(())
}

/// Validate crop type
pub fn validate_crop_type(crop_type: &str) -> bool {
    matches!(crop_type, "rice" | "cassava" | "durian" | "mango" | "rubber")
}

/// Validate language
pub fn validate_language(language: &str) -> bool {
    matches!(language, "thai" | "english")
}

/// Sanitize text input by removing HTML tags
pub fn sanitize_text_input(text: &str) -> String {
    // Simple HTML tag removal - in production use a proper sanitizer
    text.chars()
        .filter(|&c| c != '<' && c != '>')
        .collect::<String>()
        .replace("script", "")
        .replace("div", "")
}
