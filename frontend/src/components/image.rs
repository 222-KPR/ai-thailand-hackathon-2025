use yew::prelude::*;
use yewdux::prelude::*;
use web_sys::{HtmlInputElement, FileReader, File};
use wasm_bindgen::{JsCast, closure::Closure};
use wasm_bindgen_futures::JsFuture;
use shared::{CropType, ChatMessage};
use chrono::Utc;

use crate::store::{AppState, AppAction, ImageState, ImageMetadata};
use crate::i18n::I18nContext;
use crate::services::api::ApiService;

#[derive(Properties, PartialEq)]
pub struct ImageUploadProps {
    pub on_uploaded: Callback<()>,
}

#[function_component(ImageUpload)]
pub fn image_upload(props: &ImageUploadProps) -> Html {
    let (state, dispatch) = use_store::<AppState>();
    let i18n = use_context::<I18nContext>().expect("I18nContext not found");
    
    let file_input_ref = use_node_ref();
    let selected_crop = use_state(|| CropType::Rice);
    let is_uploading = use_state(|| false);
    let preview_url = use_state(|| None::<String>);
    
    let on_file_select = {
        let dispatch = dispatch.clone();
        let selected_crop = selected_crop.clone();
        let is_uploading = is_uploading.clone();
        let preview_url = preview_url.clone();
        let on_uploaded = props.on_uploaded.clone();
        
        Callback::from(move |e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            
            if let Some(file_list) = input.files() {
                if let Some(file) = file_list.get(0) {
                    let dispatch = dispatch.clone();
                    let selected_crop = (*selected_crop).clone();
                    let is_uploading = is_uploading.clone();
                    let preview_url = preview_url.clone();
                    let on_uploaded = on_uploaded.clone();
                    
                    wasm_bindgen_futures::spawn_local(async move {
                        is_uploading.set(true);
                        
                        match read_file_as_data_url(&file).await {
                            Ok(data_url) => {
                                // Extract base64 data
                                let base64_data = data_url.split(',').nth(1).unwrap_or("").to_string();
                                
                                // Create preview URL
                                preview_url.set(Some(data_url.clone()));
                                
                                // Create image metadata
                                let metadata = ImageMetadata {
                                    size_bytes: file.size() as u64,
                                    width: 0, // Will be filled by the backend
                                    height: 0, // Will be filled by the backend
                                    format: get_file_extension(&file.name()).unwrap_or("jpg".to_string()),
                                    filename: file.name(),
                                };
                                
                                let image_state = ImageState {
                                    data: base64_data.clone(),
                                    crop_type: selected_crop,
                                    metadata,
                                };
                                
                                dispatch.apply(AppAction::SetCurrentImage(image_state));
                                
                                // Send to vision service for analysis
                                match ApiService::analyze_image(&base64_data, selected_crop).await {
                                    Ok(vision_result) => {
                                        // Add diagnosis message
                                        let diagnosis_message = ChatMessage {
                                            role: "assistant".to_string(),
                                            content: format!("🔬 **ผลการวินิจฉัย**\n\n**โรคที่พบ:** {}\n**ความเชื่อมั่น:** {:.1}%\n**ความรุนแรง:** {:?}\n\nกำลังขอคำแนะนำการรักษา...", 
                                                vision_result.disease, 
                                                vision_result.confidence * 100.0,
                                                vision_result.severity
                                            ),
                                            timestamp: Utc::now(),
                                        };
                                        dispatch.apply(AppAction::AddMessage(diagnosis_message));
                                        
                                        // Get treatment advice
                                        match ApiService::get_treatment_advice(&vision_result).await {
                                            Ok(llm_result) => {
                                                let treatment_message = ChatMessage {
                                                    role: "assistant".to_string(),
                                                    content: format!("💊 **คำแนะนำการรักษา**\n\n{}\n\n**ขั้นตอนการรักษา:**\n{}\n\n**การป้องกัน:**\n{}", 
                                                        llm_result.advice,
                                                        llm_result.treatment_plan.steps.iter()
                                                            .enumerate()
                                                            .map(|(i, step)| format!("{}. {}", i + 1, step.description))
                                                            .collect::<Vec<_>>()
                                                            .join("\n"),
                                                        llm_result.prevention_tips.join("\n")
                                                    ),
                                                    timestamp: Utc::now(),
                                                };
                                                dispatch.apply(AppAction::AddMessage(treatment_message));
                                            }
                                            Err(err) => {
                                                let error_message = ChatMessage {
                                                    role: "assistant".to_string(),
                                                    content: format!("ไม่สามารถรับคำแนะนำการรักษาได้: {}", err),
                                                    timestamp: Utc::now(),
                                                };
                                                dispatch.apply(AppAction::AddMessage(error_message));
                                            }
                                        }
                                    }
                                    Err(err) => {
                                        let error_message = ChatMessage {
                                            role: "assistant".to_string(),
                                            content: format!("ไม่สามารถวิเคราะห์รูปภาพได้: {}", err),
                                            timestamp: Utc::now(),
                                        };
                                        dispatch.apply(AppAction::AddMessage(error_message));
                                    }
                                }
                                
                                on_uploaded.emit(());
                            }
                            Err(err) => {
                                dispatch.apply(AppAction::SetError(Some(format!("ไม่สามารถอ่านไฟล์ได้: {}", err))));
                            }
                        }
                        
                        is_uploading.set(false);
                    });
                }
            }
        })
    };
    
    let on_crop_change = {
        let selected_crop = selected_crop.clone();
        Callback::from(move |e: Event| {
            let select: web_sys::HtmlSelectElement = e.target_unchecked_into();
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
    
    let open_file_dialog = {
        let file_input_ref = file_input_ref.clone();
        Callback::from(move |_: MouseEvent| {
            if let Some(input) = file_input_ref.cast::<HtmlInputElement>() {
                input.click();
            }
        })
    };
    
    let open_camera = {
        Callback::from(move |_: MouseEvent| {
            // TODO: Implement camera capture
            log::info!("Camera capture not implemented yet");
        })
    };
    
    html! {
        <div class="image-upload">
            <div class="upload-header">
                <h3 class="upload-title">{i18n.t("chat.upload_image")}</h3>
            </div>
            
            <div class="crop-selector">
                <label class="crop-label" for="crop-select">
                    {i18n.t("image.select_crop")}
                </label>
                <select 
                    id="crop-select"
                    class="crop-select"
                    onchange={on_crop_change}
                    value={match *selected_crop {
                        CropType::Rice => "rice",
                        CropType::Cassava => "cassava",
                        CropType::Durian => "durian",
                        CropType::Mango => "mango",
                        CropType::Rubber => "rubber",
                    }}
                >
                    <option value="rice">{i18n.t("image.crop.rice")}</option>
                    <option value="cassava">{i18n.t("image.crop.cassava")}</option>
                    <option value="durian">{i18n.t("image.crop.durian")}</option>
                    <option value="mango">{i18n.t("image.crop.mango")}</option>
                    <option value="rubber">{i18n.t("image.crop.rubber")}</option>
                </select>
            </div>
            
            {if let Some(url) = &*preview_url {
                html! {
                    <div class="image-preview">
                        <img src={url.clone()} alt="Preview" class="preview-image" />
                    </div>
                }
            } else {
                html! {}
            }}
            
            <div class="upload-buttons">
                <button
                    class="btn btn-primary"
                    onclick={open_file_dialog}
                    disabled={*is_uploading}
                >
                    <span class="btn-icon">{"📁"}</span>
                    <span class="btn-text">{i18n.t("chat.upload_image")}</span>
                </button>
                
                <button
                    class="btn btn-secondary"
                    onclick={open_camera}
                    disabled={*is_uploading}
                >
                    <span class="btn-icon">{"📷"}</span>
                    <span class="btn-text">{i18n.t("chat.take_photo")}</span>
                </button>
            </div>
            
            {if *is_uploading {
                html! {
                    <div class="upload-progress">
                        <div class="progress-bar">
                            <div class="progress-indicator"></div>
                        </div>
                        <p class="progress-text">{i18n.t("image.analyzing")}</p>
                    </div>
                }
            } else {
                html! {}
            }}
            
            <input
                ref={file_input_ref}
                type="file"
                accept="image/*"
                onchange={on_file_select}
                style="display: none;"
            />
        </div>
    }
}

async fn read_file_as_data_url(file: &File) -> Result<String, String> {
    let file_reader = FileReader::new().map_err(|_| "Failed to create FileReader")?;
    
    let promise = js_sys::Promise::new(&mut |resolve, reject| {
        let file_reader_clone = file_reader.clone();
        
        let onload = Closure::wrap(Box::new(move |_: web_sys::Event| {
            if let Ok(result) = file_reader_clone.result() {
                if let Some(result_str) = result.as_string() {
                    resolve.call1(&wasm_bindgen::JsValue::UNDEFINED, &result_str.into()).unwrap();
                } else {
                    reject.call1(&wasm_bindgen::JsValue::UNDEFINED, &"Failed to read file as string".into()).unwrap();
                }
            } else {
                reject.call1(&wasm_bindgen::JsValue::UNDEFINED, &"Failed to get file reader result".into()).unwrap();
            }
        }) as Box<dyn FnMut(_)>);
        
        file_reader.set_onload(Some(onload.as_ref().unchecked_ref()));
        onload.forget();
        
        let onerror = Closure::wrap(Box::new(move |_: web_sys::Event| {
            reject.call1(&wasm_bindgen::JsValue::UNDEFINED, &"File read error".into()).unwrap();
        }) as Box<dyn FnMut(_)>);
        
        file_reader.set_onerror(Some(onerror.as_ref().unchecked_ref()));
        onerror.forget();
        
        file_reader.read_as_data_url(file).unwrap();
    });
    
    let result = JsFuture::from(promise).await.map_err(|_| "Promise rejected")?;
    result.as_string().ok_or_else(|| "Result is not a string".to_string())
}

fn get_file_extension(filename: &str) -> Option<String> {
    filename.split('.').last().map(|ext| ext.to_lowercase())
}