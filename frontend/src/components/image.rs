use yew::prelude::*;
use web_sys::{HtmlInputElement, FileReader, File, HtmlSelectElement, ProgressEvent};
use wasm_bindgen::{JsCast, closure::Closure};
use wasm_bindgen_futures::JsFuture;
use crate::types::{CropType, ChatMessage, Language};
use chrono::Utc;

use crate::app::AppState;
use crate::i18n::I18nContext;
use crate::services::api::ApiService;
use crate::utils::image::{validate_image_file, get_file_preview_info, FilePreviewInfo, ImageValidationError};

#[derive(Properties, PartialEq)]
pub struct ImageUploadProps {
    pub on_uploaded: Callback<()>,
}

#[function_component(ImageUpload)]
pub fn image_upload(props: &ImageUploadProps) -> Html {
    let app_state = use_context::<UseStateHandle<AppState>>().expect("AppState context not found");
    let i18n = use_context::<I18nContext>().expect("I18nContext not found");

    let file_input_ref = use_node_ref();
    let selected_crop = use_state(|| CropType::Rice);
    let is_uploading = use_state(|| false);
    let preview_url = use_state(|| None::<String>);
    let validation_error = use_state(|| None::<String>);
    let file_info = use_state(|| None::<FilePreviewInfo>);
    let upload_progress = use_state(|| 0u8);

    let on_file_select = {
        let app_state = app_state.clone();
        let selected_crop = selected_crop.clone();
        let is_uploading = is_uploading.clone();
        let preview_url = preview_url.clone();
        let validation_error = validation_error.clone();
        let file_info = file_info.clone();
        let upload_progress = upload_progress.clone();
        let on_uploaded = props.on_uploaded.clone();

        Callback::from(move |e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();

            if let Some(file_list) = input.files() {
                if let Some(file) = file_list.get(0) {
                    // Immediate validation
                    match validate_image_file(&file) {
                        Ok(_) => {
                            validation_error.set(None);
                            file_info.set(Some(get_file_preview_info(&file)));

                            let app_state = app_state.clone();
                            let selected_crop = (*selected_crop).clone();
                            let is_uploading = is_uploading.clone();
                            let preview_url = preview_url.clone();
                            let upload_progress = upload_progress.clone();
                            let on_uploaded = on_uploaded.clone();
                            let validation_error = validation_error.clone();

                            wasm_bindgen_futures::spawn_local(async move {
                                is_uploading.set(true);
                                upload_progress.set(0);

                                let progress_callback = {
                                    let upload_progress = upload_progress.clone();
                                    Callback::from(move |progress: u8| {
                                        upload_progress.set(progress);
                                    })
                                };

                                match read_file_with_progress(&file, progress_callback).await {
                                    Ok(data_url) => {
                                        // Extract base64 data
                                        let base64_data = data_url.split(',').nth(1).unwrap_or("").to_string();

                                        // Create preview URL
                                        preview_url.set(Some(data_url.clone()));

                                        // Update app state with loading
                                        {
                                            let mut new_state = (*app_state).clone();
                                            new_state.set_loading(true);
                                            new_state.set_error(None);
                                            app_state.set(new_state);
                                        }

                                        // Send to vision service for analysis
                                        match ApiService::analyze_image(base64_data, selected_crop, app_state.language.clone()).await {
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

                                                {
                                                    let mut new_state = (*app_state).clone();
                                                    new_state.add_message(diagnosis_message);
                                                    app_state.set(new_state);
                                                }

                                                // Get treatment advice
                                                match ApiService::get_treatment_advice(&vision_result, selected_crop, app_state.language.clone()).await {
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

                                                        {
                                                            let mut new_state = (*app_state).clone();
                                                            new_state.add_message(treatment_message);
                                                            new_state.set_loading(false);
                                                            app_state.set(new_state);
                                                        }
                                                    }
                                                    Err(err) => {
                                                        let error_message = ChatMessage {
                                                            role: "assistant".to_string(),
                                                            content: format!("ไม่สามารถรับคำแนะนำการรักษาได้: {}", err),
                                                            timestamp: Utc::now(),
                                                        };

                                                        {
                                                            let mut new_state = (*app_state).clone();
                                                            new_state.add_message(error_message);
                                                            new_state.set_loading(false);
                                                            new_state.set_error(Some(format!("Treatment advice error: {}", err)));
                                                            app_state.set(new_state);
                                                        }
                                                    }
                                                }
                                            }
                                            Err(err) => {
                                                let error_message = ChatMessage {
                                                    role: "assistant".to_string(),
                                                    content: format!("ไม่สามารถวิเคราะห์รูปภาพได้: {}", err),
                                                    timestamp: Utc::now(),
                                                };

                                                {
                                                    let mut new_state = (*app_state).clone();
                                                    new_state.add_message(error_message);
                                                    new_state.set_loading(false);
                                                    new_state.set_error(Some(format!("Vision analysis error: {}", err)));
                                                    app_state.set(new_state);
                                                }
                                            }
                                        }

                                        on_uploaded.emit(());
                                    }
                                    Err(err) => {
                                        validation_error.set(Some(format!("ไม่สามารถอ่านไฟล์ได้: {}", err)));
                                        let mut new_state = (*app_state).clone();
                                        new_state.set_loading(false);
                                        new_state.set_error(Some(format!("File read error: {}", err)));
                                        app_state.set(new_state);
                                    }
                                }

                                is_uploading.set(false);
                            });
                        }
                        Err(err) => {
                            validation_error.set(Some(err.to_string()));
                            file_info.set(None);
                        }
                    }
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
        <div class="image-upload enhanced">
            <div class="upload-header">
                <h3 class="upload-title">{i18n.t("chat.upload_image")}</h3>
            </div>

            // Validation errors
            {if let Some(error) = &*validation_error {
                html! {
                    <div class="validation-error">
                        <span class="error-icon">{"❌"}</span>
                        <span>{error.clone()}</span>
                    </div>
                }
            } else { html! {} }}

            // File info preview
            {if let Some(info) = &*file_info {
                html! {
                    <div class="file-info">
                        <div class="file-details">
                            <span class="file-name">{&info.name}</span>
                            <span class="file-size">{&info.size}</span>
                        </div>
                        <div class="file-meta">
                            <span class="file-type">{&info.type_}</span>
                        </div>
                    </div>
                }
            } else { html! {} }}

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
                    class="btn-upload enhanced"
                    onclick={open_file_dialog}
                    disabled={*is_uploading}
                >
                    <span class="upload-icon">{"📎"}</span>
                    <span class="upload-text">{"Choose Image"}</span>
                    <span class="upload-hint">{"JPG, PNG, WebP (max 10MB)"}</span>
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
                            <div class="progress-indicator" style={format!("width: {}%", *upload_progress)}></div>
                        </div>
                        <p class="progress-text">
                            {format!("Uploading... {}%", *upload_progress)}
                        </p>
                    </div>
                }
            } else {
                html! {}
            }}

            <input
                ref={file_input_ref}
                type="file"
                accept="image/jpeg,image/jpg,image/png,image/webp"
                onchange={on_file_select}
                style="display: none;"
            />
        </div>
    }
}

/// Read file with progress tracking
async fn read_file_with_progress(file: &File, progress_callback: Callback<u8>) -> Result<String, String> {
    let file_reader = FileReader::new().map_err(|_| "Failed to create FileReader")?;

    let promise = js_sys::Promise::new(&mut |resolve, reject| {
        let file_reader_clone = file_reader.clone();

        // Progress handler
        let progress_handler = {
            let progress_callback = progress_callback.clone();
            Closure::wrap(Box::new(move |event: ProgressEvent| {
                if event.length_computable() {
                    let progress = ((event.loaded() / event.total()) * 100.0) as u8;
                    progress_callback.emit(progress.min(99)); // Keep 100% for completion
                }
            }) as Box<dyn FnMut(_)>)
        };

        file_reader.set_onprogress(Some(progress_handler.as_ref().unchecked_ref()));
        progress_handler.forget();

        // Load handler
        let onload = Closure::wrap(Box::new(move |_: web_sys::Event| {
            progress_callback.emit(100); // Complete
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

        // Error handler
        let onerror = Closure::wrap(Box::new(move |_: web_sys::Event| {
            reject.call1(&wasm_bindgen::JsValue::UNDEFINED, &"File read error".into()).unwrap();
        }) as Box<dyn FnMut(_)>);

        file_reader.set_onerror(Some(onerror.as_ref().unchecked_ref()));
        onerror.forget();

        // Start reading
        file_reader.read_as_data_url(file).unwrap();
    });

    let result = JsFuture::from(promise).await.map_err(|_| "Promise rejected")?;
    result.as_string().ok_or_else(|| "Result is not a string".to_string())
}

/// Legacy file reading function for backward compatibility
async fn read_file_as_data_url(file: &File) -> Result<String, String> {
    let dummy_callback = Callback::from(|_: u8| {});
    read_file_with_progress(file, dummy_callback).await
}

fn get_file_extension(filename: &str) -> Option<String> {
    filename.split('.').last().map(|ext| ext.to_lowercase())
}
