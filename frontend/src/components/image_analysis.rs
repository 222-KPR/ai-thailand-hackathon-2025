use yew::prelude::*;
use web_sys::{HtmlInputElement, File as WebFile, HtmlSelectElement};
use wasm_bindgen::JsCast;
use gloo_file::File;
use crate::types::{CropType, ChatMessage, Language, JobResponse, JobStatus};
use chrono::Utc;
use uuid::Uuid;
use wasm_bindgen_futures::spawn_local;

use crate::app::AppState;
use crate::i18n::I18nContext;
use crate::services::api::ApiService;
use crate::utils::image::{validate_image_file, ImageValidationError};

#[derive(Properties, PartialEq)]
pub struct ImageAnalysisProps {
    pub on_uploaded: Callback<()>,
}

pub enum AnalysisMode {
    Disease,
    Pest,
}

#[function_component(ImageAnalysis)]
pub fn image_analysis(props: &ImageAnalysisProps) -> Html {
    let app_state = use_context::<UseStateHandle<AppState>>().expect("AppState context not found");
    let i18n = use_context::<I18nContext>().expect("I18nContext not found");

    let file_input_ref = use_node_ref();
    let selected_crop = use_state(|| CropType::Rice);
    let analysis_mode = use_state(|| AnalysisMode::Disease);
    let is_analyzing = use_state(|| false);
    let validation_error = use_state(|| None::<String>);
    let current_job_id = use_state(|| None::<String>);

    let api_service = use_memo(|_| ApiService::new(), ());

    // File selection handler
    let on_file_select = {
        let app_state = app_state.clone();
        let selected_crop = selected_crop.clone();
        let analysis_mode = analysis_mode.clone();
        let is_analyzing = is_analyzing.clone();
        let validation_error = validation_error.clone();
        let current_job_id = current_job_id.clone();
        let api_service = api_service.clone();
        let on_uploaded = props.on_uploaded.clone();

        Callback::from(move |e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let files = input.files();

            if let Some(files) = files {
                if files.length() > 0 {
                    if let Some(web_file) = files.get(0) {
                        let file = File::from(web_file);
                        let crop_type = (*selected_crop).clone();
                        let mode = (*analysis_mode).clone();
                        let api = (*api_service).clone();

                        // Validate file
                        match validate_image_file(&file) {
                            Ok(_) => {
                                validation_error.set(None);
                                is_analyzing.set(true);

                                let app_state = app_state.clone();
                                let is_analyzing = is_analyzing.clone();
                                let current_job_id = current_job_id.clone();
                                let on_uploaded = on_uploaded.clone();

                                spawn_local(async move {
                                    match mode {
                                        AnalysisMode::Disease => {
                                            analyze_disease(api, file, crop_type, app_state, is_analyzing).await;
                                        }
                                        AnalysisMode::Pest => {
                                            analyze_pest(api, file, crop_type, app_state, is_analyzing, current_job_id).await;
                                        }
                                    }
                                    on_uploaded.emit(());
                                });
                            }
                            Err(e) => {
                                validation_error.set(Some(format!("ไฟล์ไม่ถูกต้อง: {}", e)));
                            }
                        }
                    }
                }
            }
        })
    };

    // Analysis mode change handler
    let on_mode_change = {
        let analysis_mode = analysis_mode.clone();
        Callback::from(move |e: Event| {
            let select: HtmlSelectElement = e.target_unchecked_into();
            let mode = match select.value().as_str() {
                "pest" => AnalysisMode::Pest,
                _ => AnalysisMode::Disease,
            };
            analysis_mode.set(mode);
        })
    };

    // Crop type change handler
    let on_crop_change = {
        let selected_crop = selected_crop.clone();
        Callback::from(move |e: Event| {
            let select: HtmlSelectElement = e.target_unchecked_into();
            let crop = match select.value().as_str() {
                "rice" => CropType::Rice,
                "cassava" => CropType::Cassava,
                "durian" => CropType::Durian,
                "mango" => CropType::Mango,
                "rubber" => CropType::Rubber,
                _ => CropType::Rice,
            };
            selected_crop.set(crop);
        })
    };

    html! {
        <div class="image-analysis-container">
            <div class="analysis-controls">
                <div class="control-group">
                    <label for="analysis-mode">{"ประเภทการวิเคราะห์:"}</label>
                    <select id="analysis-mode" onchange={on_mode_change}>
                        <option value="disease" selected={matches!(**analysis_mode, AnalysisMode::Disease)}>
                            {"ตรวจจับโรคพืช"}
                        </option>
                        <option value="pest" selected={matches!(**analysis_mode, AnalysisMode::Pest)}>
                            {"ตรวจจับแมลงศัตรูพืช"}
                        </option>
                    </select>
                </div>

                <div class="control-group">
                    <label for="crop-type">{"ประเภทพืช:"}</label>
                    <select id="crop-type" onchange={on_crop_change}>
                        <option value="rice" selected={matches!(**selected_crop, CropType::Rice)}>{"ข้าว"}</option>
                        <option value="cassava" selected={matches!(**selected_crop, CropType::Cassava)}>{"มันสำปะหลัง"}</option>
                        <option value="durian" selected={matches!(**selected_crop, CropType::Durian)}>{"ทุเรียน"}</option>
                        <option value="mango" selected={matches!(**selected_crop, CropType::Mango)}>{"มะม่วง"}</option>
                        <option value="rubber" selected={matches!(**selected_crop, CropType::Rubber)}>{"ยางพารา"}</option>
                    </select>
                </div>
            </div>

            <div class="file-upload">
                <input
                    type="file"
                    ref={file_input_ref}
                    accept="image/*"
                    onchange={on_file_select}
                    disabled={*is_analyzing}
                />

                if let Some(error) = &**validation_error {
                    <div class="error-message">{error}</div>
                }

                if *is_analyzing {
                    <div class="loading-message">
                        {"🔬 กำลังวิเคราะห์ภาพ..."}
                    </div>
                }
            </div>
        </div>
    }
}

// Helper function for disease analysis
async fn analyze_disease(
    api: ApiService,
    file: File,
    crop_type: CropType,
    app_state: UseStateHandle<AppState>,
    is_analyzing: UseStateHandle<bool>,
) {
    // Convert file to base64
    match file_to_base64(&file).await {
        Ok(base64_data) => {
            match api.analyze_disease(base64_data, crop_type, 0.5).await {
                Ok(vision_result) => {
                    let message = ChatMessage {
                        id: Uuid::new_v4(),
                        content: format!(
                            "🔬 **ผลการวินิจฉัยโรคพืช**\n\n**โรคที่พบ:** {}\n**ความเชื่อมั่น:** {:.1}%\n**ความรุนแรง:** {}\n\n**คำแนะนำการรักษา:**\n{}",
                            vision_result.disease,
                            vision_result.confidence * 100.0,
                            vision_result.severity,
                            vision_result.treatment_steps.iter()
                                .map(|step| format!("{}. {} ({})", step.step, step.description, step.timing))
                                .collect::<Vec<_>>()
                                .join("\n")
                        ),
                        is_user: false,
                        timestamp: Utc::now(),
                        language: Language::Thai,
                    };

                    let mut new_state = (*app_state).clone();
                    new_state.add_message(message);
                    app_state.set(new_state);
                }
                Err(e) => {
                    let error_message = ChatMessage {
                        id: Uuid::new_v4(),
                        content: format!("❌ เกิดข้อผิดพลาดในการวิเคราะห์: {}", e),
                        is_user: false,
                        timestamp: Utc::now(),
                        language: Language::Thai,
                    };

                    let mut new_state = (*app_state).clone();
                    new_state.add_message(error_message);
                    app_state.set(new_state);
                }
            }
        }
        Err(e) => {
            let error_message = ChatMessage {
                id: Uuid::new_v4(),
                content: format!("❌ ไม่สามารถอ่านไฟล์ได้: {}", e),
                is_user: false,
                timestamp: Utc::now(),
                language: Language::Thai,
            };

            let mut new_state = (*app_state).clone();
            new_state.add_message(error_message);
            app_state.set(new_state);
        }
    }

    is_analyzing.set(false);
}

// Helper function for pest analysis
async fn analyze_pest(
    api: ApiService,
    file: File,
    crop_type: CropType,
    app_state: UseStateHandle<AppState>,
    is_analyzing: UseStateHandle<bool>,
    current_job_id: UseStateHandle<Option<String>>,
) {
    // Convert gloo_file::File to web_sys::File for the API
    let web_file = file.as_ref();

    match api.submit_pest_detection(web_file.clone(), crop_type, Some("User uploaded image".to_string())).await {
        Ok(job_response) => {
            current_job_id.set(Some(job_response.job_id.clone()));

            let message = ChatMessage {
                id: Uuid::new_v4(),
                content: format!("🕐 **เริ่มการตรวจจับแมลงศัตรูพืช**\n\n{}\n\nรหัสงาน: {}", job_response.message, job_response.job_id),
                is_user: false,
                timestamp: Utc::now(),
                language: Language::Thai,
            };

            let mut new_state = (*app_state).clone();
            new_state.add_message(message);
            app_state.set(new_state);

            // Poll for job completion
            poll_job_status(api, job_response.job_id, app_state, is_analyzing).await;
        }
        Err(e) => {
            let error_message = ChatMessage {
                id: Uuid::new_v4(),
                content: format!("❌ เกิดข้อผิดพลาดในการส่งงาน: {}", e),
                is_user: false,
                timestamp: Utc::now(),
                language: Language::Thai,
            };

            let mut new_state = (*app_state).clone();
            new_state.add_message(error_message);
            app_state.set(new_state);
            is_analyzing.set(false);
        }
    }
}

// Helper function to poll job status
async fn poll_job_status(
    api: ApiService,
    job_id: String,
    app_state: UseStateHandle<AppState>,
    is_analyzing: UseStateHandle<bool>,
) {
    use gloo_timers::future::sleep;
    use std::time::Duration;

    for _ in 0..30 { // Max 30 attempts (30 seconds)
        sleep(Duration::from_secs(1)).await;

        match api.get_job_status(&job_id).await {
            Ok(status) => {
                match status.status.as_str() {
                    "completed" => {
                        if let Some(result) = status.result {
                            let message = ChatMessage {
                                id: Uuid::new_v4(),
                                content: format!(
                                    "✅ **ผลการตรวจจับแมลงศัตรูพืช**\n\n{}\n\n**แมลงที่ตรวจพบ:** {}\n**จำนวนที่ตรวจพบ:** {}\n\n**ระดับความเชื่อมั่น:**\n{}",
                                    result.message,
                                    if result.detected_pests.is_empty() { "ไม่พบแมลงศัตรูพืช".to_string() } else { result.detected_pests.join(", ") },
                                    result.total_detections,
                                    result.confidence_scores.iter()
                                        .map(|(pest, confidence)| format!("- {}: {:.1}%", pest, confidence * 100.0))
                                        .collect::<Vec<_>>()
                                        .join("\n")
                                ),
                                is_user: false,
                                timestamp: Utc::now(),
                                language: Language::Thai,
                            };

                            let mut new_state = (*app_state).clone();
                            new_state.add_message(message);
                            app_state.set(new_state);
                        }
                        is_analyzing.set(false);
                        return;
                    }
                    "failed" => {
                        let error_message = ChatMessage {
                            id: Uuid::new_v4(),
                            content: format!("❌ การตรวจจับล้มเหลว: {}", status.error.unwrap_or_else(|| "ไม่ทราบสาเหตุ".to_string())),
                            is_user: false,
                            timestamp: Utc::now(),
                            language: Language::Thai,
                        };

                        let mut new_state = (*app_state).clone();
                        new_state.add_message(error_message);
                        app_state.set(new_state);
                        is_analyzing.set(false);
                        return;
                    }
                    _ => {
                        // Still processing, continue polling
                        continue;
                    }
                }
            }
            Err(_) => {
                // Continue polling on error
                continue;
            }
        }
    }

    // Timeout
    let timeout_message = ChatMessage {
        id: Uuid::new_v4(),
        content: "⏱️ การตรวจจับใช้เวลานานเกินไป กรุณาลองใหม่อีกครั้ง".to_string(),
        is_user: false,
        timestamp: Utc::now(),
        language: Language::Thai,
    };

    let mut new_state = (*app_state).clone();
    new_state.add_message(timeout_message);
    app_state.set(new_state);
    is_analyzing.set(false);
}

// Helper function to convert file to base64
async fn file_to_base64(file: &File) -> Result<String, String> {
    use wasm_bindgen_futures::JsFuture;
    use web_sys::{FileReader, Blob};
    use wasm_bindgen::JsCast;

    let file_reader = FileReader::new().map_err(|_| "Failed to create FileReader")?;
    let blob: &Blob = file.as_ref();

    file_reader.read_as_data_url(blob).map_err(|_| "Failed to read file")?;

    let promise = js_sys::Promise::new(&mut |resolve, _reject| {
        let onload = wasm_bindgen::closure::Closure::wrap(Box::new(move |_event: web_sys::Event| {
            resolve.call0(&wasm_bindgen::JsValue::NULL).unwrap();
        }) as Box<dyn FnMut(_)>);

        file_reader.set_onload(Some(onload.as_ref().unchecked_ref()));
        onload.forget();
    });

    JsFuture::from(promise).await.map_err(|_| "Promise failed")?;

    let result = file_reader.result().map_err(|_| "Failed to get result")?;
    let data_url = result.as_string().ok_or("Result is not a string")?;

    // Extract base64 part from data URL
    if let Some(comma_pos) = data_url.find(',') {
        Ok(data_url[comma_pos + 1..].to_string())
    } else {
        Err("Invalid data URL format".to_string())
    }
}
