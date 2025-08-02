pub fn validate_text_input(text: &str) -> Result<(), String> {
    let text = text.trim();
    
    if text.is_empty() {
        return Err("กรุณาใส่ข้อความ".to_string());
    }
    
    if text.len() > 5000 {
        return Err("ข้อความยาวเกินไป (สูงสุด 5000 ตัวอักษร)".to_string());
    }
    
    // Basic XSS protection - check for dangerous patterns
    if contains_suspicious_content(text) {
        return Err("พบเนื้อหาที่ไม่เหมาะสม".to_string());
    }
    
    Ok(())
}

fn contains_suspicious_content(text: &str) -> bool {
    let text_lower = text.to_lowercase();
    
    // Check for basic XSS patterns
    let suspicious_patterns = [
        "<script",
        "javascript:",
        "onclick=",
        "onload=",
        "<iframe",
        "<object",
        "<embed",
    ];
    
    for pattern in &suspicious_patterns {
        if text_lower.contains(pattern) {
            return true;
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