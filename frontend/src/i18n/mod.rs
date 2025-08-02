use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use shared::Language;

#[derive(Debug, Clone, PartialEq)]
pub struct I18nContext {
    pub language: Language,
    messages: HashMap<String, String>,
}

impl I18nContext {
    pub fn new(language: Language) -> Self {
        let messages = match language {
            Language::Thai => thai_messages(),
            Language::English => english_messages(),
        };
        
        Self { language, messages }
    }
    
    pub fn t(&self, key: &str) -> String {
        self.messages
            .get(key)
            .cloned()
            .unwrap_or_else(|| {
                log::warn!("Missing translation for key: {}", key);
                key.to_string()
            })
    }
    
    pub fn tf(&self, key: &str, args: &[(&str, &str)]) -> String {
        let mut message = self.t(key);
        for (placeholder, value) in args {
            message = message.replace(&format!("{{{}}}", placeholder), value);
        }
        message
    }
}

fn thai_messages() -> HashMap<String, String> {
    let mut messages = HashMap::new();
    
    // App title and headers
    messages.insert("app.title".to_string(), "AI4Thai Crop Guardian".to_string());
    messages.insert("app.subtitle".to_string(), "ระบบวินิจฉัยโรคพืชด้วย AI".to_string());
    messages.insert("app.description".to_string(), "ตรวจหาโรคพืช แนะนำการรักษา ด้วยเทคโนโลยี AI".to_string());
    
    // Chat interface
    messages.insert("chat.placeholder".to_string(), "ถามเกี่ยวกับโรคพืช หรือแนบรูปภาพใบไม้ที่เป็นโรค...".to_string());
    messages.insert("chat.send".to_string(), "ส่ง".to_string());
    messages.insert("chat.upload_image".to_string(), "แนบรูปภาพ".to_string());
    messages.insert("chat.take_photo".to_string(), "ถ่ายรูป".to_string());
    messages.insert("chat.voice_input".to_string(), "พูด".to_string());
    messages.insert("chat.clear".to_string(), "ล้างข้อความ".to_string());
    
    // Image upload
    messages.insert("image.select_crop".to_string(), "เลือกประเภทพืช".to_string());
    messages.insert("image.crop.rice".to_string(), "ข้าว".to_string());
    messages.insert("image.crop.cassava".to_string(), "มันสำปะหลัง".to_string());
    messages.insert("image.crop.durian".to_string(), "ทุเรียน".to_string());
    messages.insert("image.crop.mango".to_string(), "มะม่วง".to_string());
    messages.insert("image.crop.rubber".to_string(), "ยางพารา".to_string());
    messages.insert("image.uploading".to_string(), "กำลังอัพโหลดรูปภาพ...".to_string());
    messages.insert("image.analyzing".to_string(), "กำลังวิเคราะห์รูปภาพ...".to_string());
    messages.insert("image.error".to_string(), "เกิดข้อผิดพลาดในการอัพโหลดรูปภาพ".to_string());
    
    // Diagnosis results
    messages.insert("diagnosis.title".to_string(), "ผลการวินิจฉัยโรคพืช".to_string());
    messages.insert("diagnosis.disease".to_string(), "โรคที่พบ: {disease}".to_string());
    messages.insert("diagnosis.confidence".to_string(), "ความเชื่อมั่น: {confidence}%".to_string());
    messages.insert("diagnosis.severity".to_string(), "ความรุนแรง: {severity}".to_string());
    messages.insert("diagnosis.getting_advice".to_string(), "กำลังขอคำแนะนำการรักษา...".to_string());
    
    // Severity levels
    messages.insert("severity.low".to_string(), "น้อย".to_string());
    messages.insert("severity.medium".to_string(), "ปานกลาง".to_string());
    messages.insert("severity.high".to_string(), "สูง".to_string());
    messages.insert("severity.critical".to_string(), "วิกฤต".to_string());
    
    // Treatment advice
    messages.insert("treatment.title".to_string(), "คำแนะนำการรักษา".to_string());
    messages.insert("treatment.steps".to_string(), "ขั้นตอนการรักษา:".to_string());
    messages.insert("treatment.materials".to_string(), "วัสดุที่ต้องใช้:".to_string());
    messages.insert("treatment.cost".to_string(), "ค่าใช้จ่ายประมาณ: {min}-{max} บาท".to_string());
    messages.insert("treatment.timeline".to_string(), "ระยะเวลา: {days} วัน".to_string());
    messages.insert("treatment.prevention".to_string(), "การป้องกัน:".to_string());
    messages.insert("treatment.organic".to_string(), "ทางเลือกออร์แกนิก".to_string());
    
    // Connection status
    messages.insert("status.connected".to_string(), "เชื่อมต่อแล้ว".to_string());
    messages.insert("status.connecting".to_string(), "กำลังเชื่อมต่อ...".to_string());
    messages.insert("status.disconnected".to_string(), "ไม่ได้เชื่อมต่อ".to_string());
    messages.insert("status.error".to_string(), "เกิดข้อผิดพลาด".to_string());
    
    // Error messages
    messages.insert("error.network".to_string(), "เกิดข้อผิดพลาดในการเชื่อมต่อเครือข่าย".to_string());
    messages.insert("error.image_too_large".to_string(), "รูปภาพใหญ่เกินไป (สูงสุด 5MB)".to_string());
    messages.insert("error.invalid_image".to_string(), "รูปภาพไม่ถูกต้อง".to_string());
    messages.insert("error.service_unavailable".to_string(), "บริการไม่พร้อมใช้งาน".to_string());
    
    // Welcome messages
    messages.insert("welcome.title".to_string(), "ยินดีต้อนรับสู่ AI4Thai Crop Guardian".to_string());
    messages.insert("welcome.subtitle".to_string(), "ช่วยเหลือเกษตรกรไทยด้วยเทคโนโลยี AI".to_string());
    messages.insert("welcome.how_to_use".to_string(), "วิธีใช้งาน:".to_string());
    messages.insert("welcome.step1".to_string(), "1. ถ่ายรูปหรือแนบรูปภาพใบไม้ที่เป็นโรค".to_string());
    messages.insert("welcome.step2".to_string(), "2. เลือกประเภทพืชที่ตรงกับรูปภาพ".to_string());
    messages.insert("welcome.step3".to_string(), "3. รอผลการวิเคราะห์และคำแนะนำการรักษา".to_string());
    messages.insert("welcome.step4".to_string(), "4. สามารถถามคำถามเพิ่มเติมได้ในแชท".to_string());
    
    // Language toggle
    messages.insert("language.thai".to_string(), "ไทย".to_string());
    messages.insert("language.english".to_string(), "English".to_string());
    messages.insert("language.switch".to_string(), "เปลี่ยนภาษา".to_string());
    
    messages
}

fn english_messages() -> HashMap<String, String> {
    let mut messages = HashMap::new();
    
    // App title and headers
    messages.insert("app.title".to_string(), "AI4Thai Crop Guardian".to_string());
    messages.insert("app.subtitle".to_string(), "AI-Powered Crop Disease Detection".to_string());
    messages.insert("app.description".to_string(), "Detect plant diseases and get treatment advice with AI technology".to_string());
    
    // Chat interface
    messages.insert("chat.placeholder".to_string(), "Ask about plant diseases or upload an image of diseased leaves...".to_string());
    messages.insert("chat.send".to_string(), "Send".to_string());
    messages.insert("chat.upload_image".to_string(), "Upload Image".to_string());
    messages.insert("chat.take_photo".to_string(), "Take Photo".to_string());
    messages.insert("chat.voice_input".to_string(), "Voice".to_string());
    messages.insert("chat.clear".to_string(), "Clear".to_string());
    
    // Image upload
    messages.insert("image.select_crop".to_string(), "Select Crop Type".to_string());
    messages.insert("image.crop.rice".to_string(), "Rice".to_string());
    messages.insert("image.crop.cassava".to_string(), "Cassava".to_string());
    messages.insert("image.crop.durian".to_string(), "Durian".to_string());
    messages.insert("image.crop.mango".to_string(), "Mango".to_string());
    messages.insert("image.crop.rubber".to_string(), "Rubber".to_string());
    messages.insert("image.uploading".to_string(), "Uploading image...".to_string());
    messages.insert("image.analyzing".to_string(), "Analyzing image...".to_string());
    messages.insert("image.error".to_string(), "Error uploading image".to_string());
    
    // Diagnosis results
    messages.insert("diagnosis.title".to_string(), "Plant Disease Diagnosis".to_string());
    messages.insert("diagnosis.disease".to_string(), "Disease found: {disease}".to_string());
    messages.insert("diagnosis.confidence".to_string(), "Confidence: {confidence}%".to_string());
    messages.insert("diagnosis.severity".to_string(), "Severity: {severity}".to_string());
    messages.insert("diagnosis.getting_advice".to_string(), "Getting treatment advice...".to_string());
    
    // Severity levels
    messages.insert("severity.low".to_string(), "Low".to_string());
    messages.insert("severity.medium".to_string(), "Medium".to_string());
    messages.insert("severity.high".to_string(), "High".to_string());
    messages.insert("severity.critical".to_string(), "Critical".to_string());
    
    // Treatment advice
    messages.insert("treatment.title".to_string(), "Treatment Recommendations".to_string());
    messages.insert("treatment.steps".to_string(), "Treatment Steps:".to_string());
    messages.insert("treatment.materials".to_string(), "Materials needed:".to_string());
    messages.insert("treatment.cost".to_string(), "Estimated cost: {min}-{max} THB".to_string());
    messages.insert("treatment.timeline".to_string(), "Timeline: {days} days".to_string());
    messages.insert("treatment.prevention".to_string(), "Prevention:".to_string());
    messages.insert("treatment.organic".to_string(), "Organic Alternative".to_string());
    
    // Connection status
    messages.insert("status.connected".to_string(), "Connected".to_string());
    messages.insert("status.connecting".to_string(), "Connecting...".to_string());
    messages.insert("status.disconnected".to_string(), "Disconnected".to_string());
    messages.insert("status.error".to_string(), "Error".to_string());
    
    // Error messages
    messages.insert("error.network".to_string(), "Network connection error".to_string());
    messages.insert("error.image_too_large".to_string(), "Image too large (max 5MB)".to_string());
    messages.insert("error.invalid_image".to_string(), "Invalid image format".to_string());
    messages.insert("error.service_unavailable".to_string(), "Service unavailable".to_string());
    
    // Welcome messages
    messages.insert("welcome.title".to_string(), "Welcome to AI4Thai Crop Guardian".to_string());
    messages.insert("welcome.subtitle".to_string(), "Helping Thai farmers with AI technology".to_string());
    messages.insert("welcome.how_to_use".to_string(), "How to use:".to_string());
    messages.insert("welcome.step1".to_string(), "1. Take or upload a photo of diseased leaves".to_string());
    messages.insert("welcome.step2".to_string(), "2. Select the crop type matching your image".to_string());
    messages.insert("welcome.step3".to_string(), "3. Wait for analysis results and treatment advice".to_string());
    messages.insert("welcome.step4".to_string(), "4. Ask follow-up questions in the chat".to_string());
    
    // Language toggle
    messages.insert("language.thai".to_string(), "ไทย".to_string());
    messages.insert("language.english".to_string(), "English".to_string());
    messages.insert("language.switch".to_string(), "Switch Language".to_string());
    
    messages
}