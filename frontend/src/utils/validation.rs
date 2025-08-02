use regex::Regex;

pub fn validate_text_input(text: &str) -> Result<(), String> {
    let text = text.trim();
    
    if text.is_empty() {
        return Err("กรุณาใส่ข้อความ".to_string());
    }
    
    if text.len() > 5000 {
        return Err("ข้อความยาวเกินไป (สูงสุด 5000 ตัวอักษร)".to_string());
    }
    
    // Check for potentially harmful content
    if contains_suspicious_content(text) {
        return Err("พบเนื้อหาที่ไม่เหมาะสม".to_string());
    }
    
    Ok(())
}

fn contains_suspicious_content(text: &str) -> bool {
    let suspicious_patterns = [
        r"<script[^>]*>.*?</script>",
        r"javascript:",
        r"on\w+\s*=",
        r"<iframe[^>]*>",
        r"<object[^>]*>",
        r"<embed[^>]*>",
    ];
    
    let text_lower = text.to_lowercase();
    
    for pattern in &suspicious_patterns {
        if let Ok(regex) = Regex::new(pattern) {
            if regex.is_match(&text_lower) {
                return true;
            }
        }
    }
    
    false
}

pub fn sanitize_text_input(text: &str) -> String {
    text.trim()
        .replace("<", "&lt;")
        .replace(">", "&gt;")
        .replace("\"", "&quot;")
        .replace("'", "&#x27;")
        .replace("/", "&#x2F;")
}

pub fn validate_crop_type(crop_type: &str) -> bool {
    matches!(crop_type, "rice" | "cassava" | "durian" | "mango" | "rubber")
}

pub fn validate_language(language: &str) -> bool {
    matches!(language, "thai" | "english")
}