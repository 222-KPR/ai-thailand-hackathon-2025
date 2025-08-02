use web_sys::File;

pub const MAX_FILE_SIZE: u64 = 5 * 1024 * 1024; // 5MB
pub const SUPPORTED_FORMATS: &[&str] = &["jpg", "jpeg", "png", "webp"];

pub fn validate_image_file(file: &File) -> Result<(), String> {
    // Check file size
    if file.size() as u64 > MAX_FILE_SIZE {
        return Err("รูปภาพใหญ่เกินไป (สูงสุด 5MB)".to_string());
    }
    
    // Check file type
    let file_type = file.type_();
    if !file_type.starts_with("image/") {
        return Err("ไฟล์ที่เลือกไม่ใช่รูปภาพ".to_string());
    }
    
    // Check file extension
    let filename = file.name();
    let extension = get_file_extension(&filename)
        .ok_or_else(|| "ไม่พบนามสกุลไฟล์".to_string())?;
    
    if !SUPPORTED_FORMATS.contains(&extension.as_str()) {
        return Err(format!(
            "รูปแบบไฟล์ไม่รองรับ กรุณาใช้: {}",
            SUPPORTED_FORMATS.join(", ")
        ));
    }
    
    Ok(())
}

pub fn get_file_extension(filename: &str) -> Option<String> {
    filename
        .split('.')
        .last()
        .map(|ext| ext.to_lowercase())
}

pub fn format_file_size(bytes: u64) -> String {
    const UNITS: &[&str] = &["B", "KB", "MB", "GB"];
    let mut size = bytes as f64;
    let mut unit_index = 0;
    
    while size >= 1024.0 && unit_index < UNITS.len() - 1 {
        size /= 1024.0;
        unit_index += 1;
    }
    
    if unit_index == 0 {
        format!("{} {}", bytes, UNITS[unit_index])
    } else {
        format!("{:.1} {}", size, UNITS[unit_index])
    }
}

pub fn create_image_thumbnail(
    data_url: &str,
    max_width: u32,
    max_height: u32,
) -> Result<String, String> {
    // This would need to be implemented with canvas manipulation
    // For now, just return the original data URL
    Ok(data_url.to_string())
}

pub fn extract_base64_data(data_url: &str) -> Option<String> {
    data_url.split(',').nth(1).map(|s| s.to_string())
}

pub fn get_mime_type_from_data_url(data_url: &str) -> Option<String> {
    data_url
        .split(',')
        .next()
        .and_then(|header| {
            header
                .strip_prefix("data:")
                .and_then(|s| s.split(';').next())
                .map(|s| s.to_string())
        })
}