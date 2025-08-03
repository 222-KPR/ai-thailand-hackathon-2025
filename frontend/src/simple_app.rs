//! AI4Thai Frontend - Crop Disease Detection Interface
//!
//! This module provides a React-like component for image upload and analysis.
//! Implements clean architecture patterns with separation of concerns.

use gloo_net::http::Request;
use serde_json::{json, Value};
use uuid::Uuid;
use wasm_bindgen::{closure::Closure, JsCast};
use yew::prelude::*;
use yew_router::prelude::*;

// Constants
const MAX_FILE_SIZE: usize = 10 * 1024 * 1024; // 10MB
const ALLOWED_TYPES: &[&str] = &["image/jpeg", "image/png", "image/webp"];
const API_BASE_URL: &str = "http://localhost:2001";

// Type definitions for better code organization
#[derive(Clone, PartialEq)]
pub enum AnalysisState {
    Idle,
    Processing,
    Success(String),
    Error(String),
}

#[derive(Clone, PartialEq)]
pub struct AppState {
    selected_file: Option<web_sys::File>,
    preview_url: Option<String>,
    user_query: String,
    plant_type: String,
    analysis_state: AnalysisState,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            selected_file: None,
            preview_url: None,
            user_query: String::new(),
            plant_type: "rice".to_string(),
            analysis_state: AnalysisState::Idle,
        }
    }
}

/// Main image upload and analysis component
#[function_component(SimpleImageUpload)]
pub fn simple_image_upload() -> Html {
    let app_state = use_state(AppState::default);
    let file_input_ref = use_node_ref();

    // File selection handler with validation
    let on_file_select = {
        let app_state = app_state.clone();

        Callback::from(move |e: Event| {
            let input: web_sys::HtmlInputElement = e.target_unchecked_into();

            if let Some(file_list) = input.files() {
                if let Some(file) = file_list.get(0) {
                    // Validate file type
                    if !ALLOWED_TYPES.contains(&file.type_().as_str()) {
                        app_state.set(AppState {
                            analysis_state: AnalysisState::Error(
                                "Invalid file type. Please upload JPEG, PNG, or WebP images."
                                    .to_string(),
                            ),
                            ..(*app_state).clone()
                        });
                        return;
                    }

                    // Validate file size
                    if file.size() as usize > MAX_FILE_SIZE {
                        app_state.set(AppState {
                            analysis_state: AnalysisState::Error(
                                "File too large. Maximum size is 10MB.".to_string(),
                            ),
                            ..(*app_state).clone()
                        });
                        return;
                    }

                    // Create preview URL
                    let reader = web_sys::FileReader::new().unwrap();
                    let app_state_clone = app_state.clone();
                    let file_clone = file.clone();

                    let onload = {
                        let reader = reader.clone();
                        Closure::wrap(Box::new(move || {
                            if let Ok(result) = reader.result() {
                                if let Some(url) = result.as_string() {
                                    app_state_clone.set(AppState {
                                        selected_file: Some(file_clone.clone()),
                                        preview_url: Some(url),
                                        analysis_state: AnalysisState::Idle,
                                        ..(*app_state_clone).clone()
                                    });
                                }
                            }
                        }) as Box<dyn Fn()>)
                    };

                    reader.set_onload(Some(onload.as_ref().unchecked_ref()));
                    let _ = reader.read_as_data_url(&file);
                    onload.forget();
                }
            }
        })
    };

    // Query input handler
    let on_query_change = {
        let app_state = app_state.clone();

        Callback::from(move |e: Event| {
            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
            app_state.set(AppState {
                user_query: input.value(),
                ..(*app_state).clone()
            });
        })
    };

    // Plant type selection handler
    let on_plant_type_change = {
        let app_state = app_state.clone();

        Callback::from(move |e: Event| {
            let select: web_sys::HtmlSelectElement = e.target_unchecked_into();
            app_state.set(AppState {
                plant_type: select.value(),
                ..(*app_state).clone()
            });
        })
    };

    // Analysis trigger with comprehensive validation
    let on_analyze = {
        let app_state = app_state.clone();

        Callback::from(move |_| {
            let current_state = (*app_state).clone();

            // Validation checks
            if current_state.selected_file.is_none() {
                app_state.set(AppState {
                    analysis_state: AnalysisState::Error(
                        "Please select an image first.".to_string(),
                    ),
                    ..current_state
                });
                return;
            }

            if current_state.user_query.trim().is_empty() {
                app_state.set(AppState {
                    analysis_state: AnalysisState::Error(
                        "Please describe what you want to analyze.".to_string(),
                    ),
                    ..current_state
                });
                return;
            }

            // Clone values before moving current_state
            let data_url = current_state.preview_url.clone().unwrap_or_default();
            let query = current_state.user_query.clone();
            let crop_type = current_state.plant_type.clone();

            // Start analysis
            app_state.set(AppState {
                analysis_state: AnalysisState::Processing,
                ..current_state
            });

            let app_state_clone = app_state.clone();

            wasm_bindgen_futures::spawn_local(async move {
                match analyze_image_from_url(data_url, query, crop_type).await {
                    Ok(result) => {
                        app_state_clone.set(AppState {
                            analysis_state: AnalysisState::Success(result),
                            ..(*app_state_clone).clone()
                        });
                    }
                    Err(error) => {
                        app_state_clone.set(AppState {
                            analysis_state: AnalysisState::Error(error),
                            ..(*app_state_clone).clone()
                        });
                    }
                }
            });
        })
    };

    // Render component
    let current_state = (*app_state).clone();

    html! {
        <div class="image-upload-container">
            <h2>{"📷 ตรวจวินิจฉัยโรคพืช"}</h2>
            <p>{"อัปโหลดรูปภาพพืชของคุณเพื่อตรวจจับโรค"}</p>

            <div class="upload-section" style="margin-bottom: 30px;">
                <input
                    ref={&file_input_ref}
                    type="file"
                    accept="image/*"
                    onchange={on_file_select}
                    style="margin-bottom: 20px; padding: 12px; border: 2px solid #ddd; border-radius: 8px; width: 100%; box-sizing: border-box;"
                />

                {if let Some(url) = current_state.preview_url.as_ref() {
                    html! {
                        <div class="preview-container" style="text-align: center; margin-bottom: 30px;">
                            <h3>{"ภาพที่เลือก:"}</h3>
                            <img src={url.clone()} alt="Preview" style="max-width: 400px; max-height: 300px; border: 2px solid #ddd; border-radius: 8px; box-shadow: 0 4px 8px rgba(0,0,0,0.1);" />
                        </div>
                    }
                } else {
                    html! {
                        <div class="placeholder" style="text-align: center; padding: 40px; border: 2px dashed #ccc; border-radius: 8px; background: #f9f9f9;">
                            <p style="color: #666; font-size: 1.1rem;">{"📁 กรุณาเลือกไฟล์รูปภาพ"}</p>
                        </div>
                    }
                }}
            </div>

            {if current_state.preview_url.is_some() {
                html! {
                    <div class="analysis-section" style="background: #f8f9fa; padding: 20px; border-radius: 8px; border: 1px solid #e9ecef;">
                        <h3>{"🔬 ข้อมูลสำหรับการวิเคราะห์"}</h3>

                        <div style="margin-bottom: 20px;">
                            <label style="display: block; margin-bottom: 8px; font-weight: bold; color: #333;">
                                {"🌱 ประเภทพืช:"}
                            </label>
                            <select
                                value={current_state.plant_type.clone()}
                                onchange={on_plant_type_change}
                                style="width: 100%; padding: 12px; border: 1px solid #ddd; border-radius: 6px; font-size: 16px;"
                            >
                                <option value="rice">{"🌾 ข้าว (Rice)"}</option>
                                <option value="cassava">{"🥔 มันสำปะหลัง (Cassava)"}</option>
                                <option value="durian">{"🥭 ทุเรียน (Durian)"}</option>
                                <option value="mango">{"🥭 มะม่วง (Mango)"}</option>
                                <option value="rubber">{"🌳 ยางพารา (Rubber)"}</option>
                            </select>
                        </div>

                        <div style="margin-bottom: 20px;">
                            <label style="display: block; margin-bottom: 8px; font-weight: bold; color: #333;">
                                {"💬 คำถามเพิ่มเติม (ไม่บังคับ):"}
                            </label>
                            <input
                                type="text"
                                value={current_state.user_query.clone()}
                                onchange={on_query_change}
                                placeholder="เช่น 'ใบเหลือง มีจุดด่างดำ' หรือ 'พืชเริ่มเหี่ยวแล้ว'"
                                style="width: 100%; padding: 12px; border: 1px solid #ddd; border-radius: 6px; font-size: 16px; box-sizing: border-box;"
                            />
                            <small style="color: #666;">{"กรอกอาการที่สังเกตเห็นเพื่อช่วยให้ AI วิเคราะห์ได้แม่นยำยิ่งขึ้น"}</small>
                        </div>

                        <button
                            onclick={on_analyze}
                            disabled={matches!(current_state.analysis_state, AnalysisState::Processing)}
                            style="width: 100%; padding: 15px; background: linear-gradient(135deg, #28a745, #20c997); color: white; border: none; border-radius: 8px; font-size: 18px; font-weight: bold; cursor: pointer; transition: all 0.3s ease;"
                        >
                            {match current_state.analysis_state {
                                AnalysisState::Processing => "🔄 กำลังวิเคราะห์...",
                                _ => "🚀 เริ่มวิเคราะห์โรคพืช"
                            }}
                        </button>
                    </div>
                }
            } else {
                html! {}
            }}

            {match current_state.analysis_state {
                AnalysisState::Error(ref error) => html! {
                    <div class="error-message" style="margin-top: 20px; padding: 15px; background: #f8d7da; border: 1px solid #f5c6cb; border-radius: 8px; color: #721c24;">
                        <h4>{"❌ เกิดข้อผิดพลาด:"}</h4>
                        <p>{error}</p>
                    </div>
                },
                AnalysisState::Success(ref result) => html! {
                    <div class="analysis-result" style="margin-top: 20px; padding: 20px; background: #d1f2eb; border: 1px solid #7dcea0; border-radius: 8px;">
                        <h3>{"✅ ผลการวิเคราะห์:"}</h3>
                        <div style="white-space: pre-wrap; line-height: 1.6; color: #2d5016;">
                            {result}
                        </div>
                    </div>
                },
                _ => html! {}
            }}
        </div>
    }
}

// Simple Routes
#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/camera")]
    Camera,
    #[not_found]
    #[at("/404")]
    NotFound,
}

// Simple Home Component
#[function_component(SimpleHome)]
pub fn simple_home() -> Html {
    let navigator = use_navigator().unwrap();
    let navigate_to_camera = {
        let navigator = navigator.clone();
        Callback::from(move |_| {
            navigator.push(&Route::Camera);
        })
    };

    html! {
        <div style="max-width: 800px; margin: 0 auto; padding: 20px; font-family: Arial, sans-serif;">
            <div style="text-align: center; background: linear-gradient(135deg, #667eea 0%, #764ba2 100%); color: white; padding: 40px; border-radius: 12px; margin-bottom: 30px;">
                <h1 style="margin: 0 0 10px 0; font-size: 2.5rem;">{"🌾 AI4Thai Crop Guardian"}</h1>
                <p style="margin: 0 0 30px 0; font-size: 1.2rem;">{"ระบบตรวจจับโรคพืชด้วย AI สำหรับเกษตรกรไทย"}</p>

                <button
                    onclick={navigate_to_camera}
                    style="background: #ff6b6b; color: white; border: none; padding: 15px 30px; border-radius: 8px; font-size: 1.1rem; cursor: pointer; display: flex; align-items: center; gap: 10px; margin: 0 auto;"
                >
                    <span>{"📷"}</span>
                    <span>{"เริ่มตรวจวินิจฉัยโรคพืช"}</span>
                </button>
            </div>

            <div style="display: grid; grid-template-columns: repeat(auto-fit, minmax(250px, 1fr)); gap: 20px;">
                <div style="background: #fef3c7; padding: 20px; border-radius: 8px;">
                    <h3 style="color: #92400e; margin: 0 0 10px 0;">{"🔍 Disease Detection"}</h3>
                    <p style="color: #78350f; margin: 0;">{"Upload crop images for AI-powered disease identification"}</p>
                </div>
                <div style="background: #dcfce7; padding: 20px; border-radius: 8px;">
                    <h3 style="color: #166534; margin: 0 0 10px 0;">{"💬 Smart Chat"}</h3>
                    <p style="color: #15803d; margin: 0;">{"Get farming advice and treatment recommendations"}</p>
                </div>
                <div style="background: #dbeafe; padding: 20px; border-radius: 8px;">
                    <h3 style="color: #1d4ed8; margin: 0 0 10px 0;">{"🌐 Thai Support"}</h3>
                    <p style="color: #1e40af; margin: 0;">{"Native Thai language processing and responses"}</p>
                </div>
            </div>
        </div>
    }
}

// Route switch
fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <SimpleHome /> },
        Route::Camera => html! {
            <div style="max-width: 800px; margin: 0 auto; padding: 20px; font-family: Arial, sans-serif;">
                <SimpleImageUpload />
            </div>
        },
        Route::NotFound => html! { <div>{"Page Not Found"}</div> },
    }
}

// Simple App Component
#[function_component(SimpleApp)]
pub fn simple_app() -> Html {
    html! {
        <BrowserRouter>
            <div class="app">
                <main>
                    <Switch<Route> render={switch} />
                </main>
            </div>
        </BrowserRouter>
    }
}

// Helper function to extract base64 from data URL
fn extract_base64_from_url(data_url: &str) -> Result<String, String> {
    if let Some(base64_data) = data_url.split(',').nth(1) {
        Ok(base64_data.to_string())
    } else {
        Err("Failed to extract base64 from data URL".to_string())
    }
}

// Function to analyze image using AI service
async fn analyze_image_from_url(
    data_url: String,
    user_query: String,
    crop_type: String,
) -> Result<String, String> {
    // Extract base64 data from the data URL
    let base64_data = extract_base64_from_url(&data_url)?;

    // Map crop type to the format expected by the API
    let crop_type_enum = match crop_type.as_str() {
        "rice" => "Rice",
        "cassava" => "Cassava",
        "durian" => "Durian",
        "mango" => "Mango",
        "rubber" => "Rubber",
        _ => "Rice", // default
    };

    // Prepare the request payload
    let request_payload = json!({
        "image_data": base64_data,
        "crop_type": crop_type_enum,
        "user_query": user_query.trim(),
        "conversation_id": Uuid::new_v4().to_string()
    });

    // Make the API call to analyze endpoint
    let response = Request::post(&format!("{API_BASE_URL}/analyze"))
        .header("Content-Type", "application/json")
        .body(request_payload.to_string())
        .map_err(|e| format!("Failed to build request: {e}"))?
        .send()
        .await
        .map_err(|e| format!("Network error: {e}"))?;

    if response.ok() {
        let response_text = response
            .text()
            .await
            .map_err(|e| format!("Failed to read response: {e}"))?;

        // Try to parse as JSON and extract the result
        match serde_json::from_str::<Value>(&response_text) {
            Ok(json_value) => {
                if let Some(data) = json_value.get("data") {
                    if let Some(vision_result) = data.get("vision_result") {
                        if let Some(disease) = vision_result.get("disease") {
                            let disease_name = disease.as_str().unwrap_or("Unknown");
                            let confidence = vision_result
                                .get("confidence")
                                .and_then(|c| c.as_f64())
                                .unwrap_or(0.0);
                            let severity = vision_result
                                .get("severity")
                                .and_then(|s| s.as_str())
                                .unwrap_or("Unknown");

                            let mut result = format!(
                                "🔍 ผลการตรวจจับโรค:\n• โรค: {}\n• ความมั่นใจ: {:.1}%\n• ระดับความรุนแรง: {}\n",
                                disease_name, confidence * 100.0, severity
                            );

                            if let Some(llm_result) = data.get("llm_result") {
                                if let Some(advice) = llm_result.get("advice") {
                                    result.push_str(&format!(
                                        "\n💡 คำแนะนำการรักษา:\n{}\n",
                                        advice.as_str().unwrap_or("")
                                    ));
                                }

                                if let Some(treatment_plan) = llm_result.get("treatment_plan") {
                                    if let Some(steps) = treatment_plan.get("steps") {
                                        if let Some(steps_array) = steps.as_array() {
                                            result.push_str("\n📋 ขั้นตอนการรักษา:\n");
                                            for (i, step) in steps_array.iter().enumerate() {
                                                if let Some(description) = step.get("description") {
                                                    result.push_str(&format!(
                                                        "{}. {}\n",
                                                        i + 1,
                                                        description.as_str().unwrap_or("")
                                                    ));
                                                }
                                            }
                                        }
                                    }
                                }
                            }

                            Ok(result)
                        } else {
                            Ok("ไม่สามารถตรวจจับโรคได้ กรุณาลองใหม่ด้วยรูปภาพที่ชัดเจนขึ้น".to_string())
                        }
                    } else {
                        Ok(format!("ไม่พบข้อมูลการวิเคราะห์ในผลตอบกลับ: {response_text}"))
                    }
                } else {
                    Ok(format!("รูปแบบผลตอบกลับไม่ถูกต้อง: {response_text}"))
                }
            }
            Err(_) => {
                // If not JSON, return the response as is
                Ok(response_text)
            }
        }
    } else {
        let error_text = response
            .text()
            .await
            .unwrap_or_else(|_| format!("HTTP Error {}", response.status()));
        Err(format!(
            "การเรียก API ผิดพลาด ({}): {}",
            response.status(),
            error_text
        ))
    }
}
