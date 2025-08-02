// Camera Capture Component - 2025 Design System
// Modern camera interface with guidance overlay and bento controls

use yew::prelude::*;
use web_sys::{HtmlVideoElement, HtmlCanvasElement, MediaStream, MediaDevices};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use crate::components::layout::{BentoGrid, BentoCard};
use crate::components::ui::{GradientButton, ButtonVariant, ButtonSize};
use crate::styles::{use_theme, Typography, TypographyVariant, TypographyColor};

#[derive(Debug, Clone, PartialEq)]
pub enum CameraState {
    Inactive,
    Loading,
    Active,
    Capturing,
    Preview,
    Error(String),
}

#[derive(Debug, Clone, PartialEq)]
pub enum CameraFacing {
    User,      // Front camera
    Environment, // Back camera
}

#[derive(Properties, PartialEq)]
pub struct CameraCaptureProps {
    pub on_capture: Callback<String>, // Base64 image data
    pub on_cancel: Option<Callback<()>>,
    pub crop_type: Option<String>,
    pub class: Option<String>,
}

#[function_component(CameraCapture)]
pub fn camera_capture(props: &CameraCaptureProps) -> Html {
    let theme = use_theme();
    let colors = &theme.colors;

    let video_ref = use_node_ref();
    let canvas_ref = use_node_ref();
    let camera_state = use_state(|| CameraState::Inactive);
    let camera_facing = use_state(|| CameraFacing::Environment);
    let stream_handle = use_state(|| None::<MediaStream>);
    let captured_image = use_state(|| None::<String>);
    let show_tips = use_state(|| true);

    // Start camera
    let start_camera = {
        let video_ref = video_ref.clone();
        let camera_state = camera_state.clone();
        let camera_facing = camera_facing.clone();
        let stream_handle = stream_handle.clone();

        Callback::from(move |_| {
            let video_ref = video_ref.clone();
            let camera_state = camera_state.clone();
            let camera_facing = camera_facing.clone();
            let stream_handle = stream_handle.clone();

            camera_state.set(CameraState::Loading);

            wasm_bindgen_futures::spawn_local(async move {
                match get_user_media(&camera_facing).await {
                    Ok(stream) => {
                        if let Some(video) = video_ref.cast::<HtmlVideoElement>() {
                            video.set_src_object(Some(&stream));
                            stream_handle.set(Some(stream));
                            camera_state.set(CameraState::Active);
                        }
                    }
                    Err(error) => {
                        camera_state.set(CameraState::Error(error));
                    }
                }
            });
        })
    };

    // Switch camera
    let switch_camera = {
        let camera_facing = camera_facing.clone();
        let start_camera = start_camera.clone();

        Callback::from(move |_| {
            let new_facing = match *camera_facing {
                CameraFacing::User => CameraFacing::Environment,
                CameraFacing::Environment => CameraFacing::User,
            };
            camera_facing.set(new_facing);
            start_camera.emit(());
        })
    };

    // Capture image
    let capture_image = {
        let video_ref = video_ref.clone();
        let canvas_ref = canvas_ref.clone();
        let camera_state = camera_state.clone();
        let captured_image = captured_image.clone();
        let on_capture = props.on_capture.clone();

        Callback::from(move |_| {
            let video_ref = video_ref.clone();
            let canvas_ref = canvas_ref.clone();
            let camera_state = camera_state.clone();
            let captured_image = captured_image.clone();
            let on_capture = on_capture.clone();

            camera_state.set(CameraState::Capturing);

            if let (Some(video), Some(canvas)) = (
                video_ref.cast::<HtmlVideoElement>(),
                canvas_ref.cast::<HtmlCanvasElement>()
            ) {
                let context = canvas
                    .get_context("2d")
                    .unwrap()
                    .unwrap()
                    .dyn_into::<web_sys::CanvasRenderingContext2d>()
                    .unwrap();

                // Set canvas size to video size
                let video_width = video.video_width();
                let video_height = video.video_height();
                canvas.set_width(video_width);
                canvas.set_height(video_height);

                // Draw video frame to canvas
                context.draw_image_with_html_video_element(&video, 0.0, 0.0).unwrap();

                // Get image data as base64
                let image_data = canvas.to_data_url().unwrap();
                captured_image.set(Some(image_data.clone()));
                camera_state.set(CameraState::Preview);

                // Emit captured image
                on_capture.emit(image_data);
            }
        })
    };

    // Retake photo
    let retake_photo = {
        let camera_state = camera_state.clone();
        let captured_image = captured_image.clone();

        Callback::from(move |_| {
            captured_image.set(None);
            camera_state.set(CameraState::Active);
        })
    };

    // Open gallery
    let open_gallery = Callback::from(|_| {
        // Implement gallery opening logic
        web_sys::console::log_1(&"Open gallery".into());
    });

    // Toggle tips
    let toggle_tips = {
        let show_tips = show_tips.clone();
        Callback::from(move |_| {
            show_tips.set(!*show_tips);
        })
    };

    // Cancel callback
    let on_cancel = {
        let on_cancel = props.on_cancel.clone();
        Callback::from(move |_| {
            if let Some(on_cancel) = &on_cancel {
                on_cancel.emit(());
            }
        })
    };

    html! {
        <div class={classes!("camera-interface", props.class.clone())}>
            // Camera Header
            <div class="camera-header">
                <Typography variant={TypographyVariant::H3} class="camera-title thai-text">
                    {"📷 ถ่ายรูปตรวจโรคพืช"}
                </Typography>
                if let Some(crop_type) = &props.crop_type {
                    <Typography variant={TypographyVariant::Body2} color={TypographyColor::Secondary} class="thai-text">
                        {format!("ประเภทพืช: {}", crop_type)}
                    </Typography>
                }
            </div>

            // Camera Container
            <div class="camera-container">
                // Camera Viewfinder
                <div class="camera-viewfinder">
                    if matches!(*camera_state, CameraState::Preview) {
                        // Preview captured image
                        if let Some(image_data) = &*captured_image {
                            <img src={image_data.clone()} alt="Captured image" class="captured-image" />
                        }
                    } else {
                        // Live camera feed
                        <video
                            ref={video_ref.clone()}
                            autoplay=true
                            playsinline=true
                            muted=true
                            class="camera-video"
                        />
                    }

                    // Hidden canvas for capture
                    <canvas ref={canvas_ref.clone()} style="display: none;"></canvas>

                    // Camera Overlay
                    <div class="camera-overlay">
                        // Focus frame
                        <div class="focus-frame">
                            <div class="focus-corner focus-corner-tl"></div>
                            <div class="focus-corner focus-corner-tr"></div>
                            <div class="focus-corner focus-corner-bl"></div>
                            <div class="focus-corner focus-corner-br"></div>
                        </div>

                        // Guidance text
                        <div class="guidance-text">
                            <Typography variant={TypographyVariant::Body2} color={TypographyColor::Inverse} class="thai-text">
                                {"📍 วางใบพืชที่เป็นโรคให้อยู่ในกรอบ"}
                            </Typography>
                        </div>

                        // Loading indicator
                        if matches!(*camera_state, CameraState::Loading) {
                            <div class="camera-loading">
                                <div class="camera-loading-spinner"></div>
                                <Typography variant={TypographyVariant::Body2} color={TypographyColor::Inverse}>
                                    {"กำลังเปิดกล้อง..."}
                                </Typography>
                            </div>
                        }

                        // Capturing indicator
                        if matches!(*camera_state, CameraState::Capturing) {
                            <div class="camera-capturing">
                                <div class="capture-flash"></div>
                                <Typography variant={TypographyVariant::Body2} color={TypographyColor::Inverse}>
                                    {"📸 กำลังถ่ายรูป..."}
                                </Typography>
                            </div>
                        }
                    </div>
                </div>

                // Control Panel with Bento Layout
                <div class="camera-controls">
                    if matches!(*camera_state, CameraState::Preview) {
                        // Preview controls
                        <BentoGrid columns={2} gap="1rem">
                            <BentoCard color={colors.accent_lime_green} hover_effect={true}>
                                <GradientButton
                                    variant={ButtonVariant::Success}
                                    size={ButtonSize::Medium}
                                    onclick={Callback::from(move |_| {
                                        // Use the captured image
                                        web_sys::console::log_1(&"Use captured image".into());
                                    })}
                                    icon="✅"
                                    full_width={true}
                                >
                                    {"ใช้รูปนี้"}
                                </GradientButton>
                            </BentoCard>

                            <BentoCard color={colors.text_secondary} hover_effect={true}>
                                <GradientButton
                                    variant={ButtonVariant::Secondary}
                                    size={ButtonSize::Medium}
                                    onclick={retake_photo}
                                    icon="🔄"
                                    full_width={true}
                                >
                                    {"ถ่ายใหม่"}
                                </GradientButton>
                            </BentoCard>
                        </BentoGrid>
                    } else {
                        // Camera controls
                        <BentoGrid columns={3} gap="1rem">
                            <BentoCard color={colors.accent_lime_green} hover_effect={true}>
                                <GradientButton
                                    variant={ButtonVariant::Success}
                                    size={ButtonSize::Medium}
                                    onclick={switch_camera}
                                    icon="🔄"
                                    full_width={true}
                                    disabled={!matches!(*camera_state, CameraState::Active)}
                                >
                                    {"เปลี่ยนกล้อง"}
                                </GradientButton>
                            </BentoCard>

                            <BentoCard
                                gradient={colors.get_primary_gradient()}
                                hover_effect={true}
                            >
                                if matches!(*camera_state, CameraState::Inactive) {
                                    <GradientButton
                                        variant={ButtonVariant::Primary}
                                        size={ButtonSize::Medium}
                                        onclick={start_camera}
                                        icon="📷"
                                        full_width={true}
                                    >
                                        {"เปิดกล้อง"}
                                    </GradientButton>
                                } else {
                                    <GradientButton
                                        variant={ButtonVariant::Primary}
                                        size={ButtonSize::Medium}
                                        onclick={capture_image}
                                        icon="📸"
                                        full_width={true}
                                        disabled={!matches!(*camera_state, CameraState::Active)}
                                        loading={matches!(*camera_state, CameraState::Capturing)}
                                    >
                                        {"ถ่ายรูป"}
                                    </GradientButton>
                                }
                            </BentoCard>

                            <BentoCard color={colors.accent_purple} hover_effect={true}>
                                <GradientButton
                                    variant={ButtonVariant::Custom {
                                        primary: colors.accent_purple.to_string(),
                                        secondary: colors.primary_energetic_pink.to_string()
                                    }}
                                    size={ButtonSize::Medium}
                                    onclick={open_gallery}
                                    icon="🖼️"
                                    full_width={true}
                                >
                                    {"แกลเลอรี"}
                                </GradientButton>
                            </BentoCard>
                        </BentoGrid>
                    }
                </div>

                // Error display
                if let CameraState::Error(error) = &*camera_state {
                    <div class="camera-error">
                        <Typography variant={TypographyVariant::H5} color={TypographyColor::Error}>
                            {"❌ เกิดข้อผิดพลาด"}
                        </Typography>
                        <Typography variant={TypographyVariant::Body2} color={TypographyColor::Secondary} class="thai-text">
                            {error}
                        </Typography>
                        <GradientButton
                            variant={ButtonVariant::Primary}
                            onclick={start_camera}
                            icon="🔄"
                        >
                            {"ลองใหม่"}
                        </GradientButton>
                    </div>
                }
            </div>

            // Tips Section
            if *show_tips && !matches!(*camera_state, CameraState::Preview) {
                <div class="camera-tips">
                    <div class="tips-header">
                        <Typography variant={TypographyVariant::H5}>
                            {"💡 เคล็ดลับการถ่ายรูปที่ดี"}
                        </Typography>
                        <button class="tips-close" onclick={toggle_tips}>{"×"}</button>
                    </div>

                    <BentoGrid columns={2} gap="0.75rem">
                        <BentoCard class="tip-card">
                            <div class="tip-content">
                                <div class="tip-icon">{"🌞"}</div>
                                <Typography variant={TypographyVariant::Body2} class="thai-text">
                                    {"ใช้แสงธรรมชาติ หลีกเลี่ยงแสงแฟลช"}
                                </Typography>
                            </div>
                        </BentoCard>

                        <BentoCard class="tip-card">
                            <div class="tip-content">
                                <div class="tip-icon">{"🔍"}</div>
                                <Typography variant={TypographyVariant::Body2} class="thai-text">
                                    {"ถ่ายใกล้ส่วนที่เป็นโรค"}
                                </Typography>
                            </div>
                        </BentoCard>

                        <BentoCard class="tip-card">
                            <div class="tip-content">
                                <div class="tip-icon">{"📐"}</div>
                                <Typography variant={TypographyVariant::Body2} class="thai-text">
                                    {"ถ่ายหลายมุมมอง"}
                                </Typography>
                            </div>
                        </BentoCard>

                        <BentoCard class="tip-card">
                            <div class="tip-content">
                                <div class="tip-icon">{"🎯"}</div>
                                <Typography variant={TypographyVariant::Body2} class="thai-text">
                                    {"โฟกัสให้ชัดเจน"}
                                </Typography>
                            </div>
                        </BentoCard>
                    </BentoGrid>
                </div>
            }

            // Cancel button
            if let Some(_) = &props.on_cancel {
                <div class="camera-cancel">
                    <GradientButton
                        variant={ButtonVariant::Secondary}
                        onclick={on_cancel}
                        icon="❌"
                    >
                        {"ยกเลิก"}
                    </GradientButton>
                </div>
            }
        </div>
    }
}

// Helper function to get user media
async fn get_user_media(facing: &CameraFacing) -> Result<MediaStream, String> {
    let window = web_sys::window().ok_or("No window object")?;
    let navigator = window.navigator();
    let media_devices = navigator
        .media_devices()
        .map_err(|_| "MediaDevices not supported")?;

    let mut constraints = web_sys::MediaStreamConstraints::new();

    // Video constraints
    let video_constraints = js_sys::Object::new();
    js_sys::Reflect::set(
        &video_constraints,
        &"facingMode".into(),
        &match facing {
            CameraFacing::User => "user".into(),
            CameraFacing::Environment => "environment".into(),
        },
    ).unwrap();

    constraints.video(&video_constraints);
    constraints.audio(&false.into());

    let promise = media_devices
        .get_user_media_with_constraints(&constraints)
        .map_err(|_| "Failed to get user media")?;

    let stream = JsFuture::from(promise)
        .await
        .map_err(|_| "Failed to get media stream")?;

    Ok(stream.dyn_into::<MediaStream>().unwrap())
}

// CSS for camera interface
pub fn generate_camera_css() -> String {
    r#"/* Camera Interface Styles - 2025 Design */

.camera-interface {
  display: flex;
  flex-direction: column;
  gap: var(--space-lg);
  padding: var(--space-lg);
  background: var(--color-bg-light);
  min-height: 100vh;
}

.camera-header {
  text-align: center;
  margin-bottom: var(--space-md);
}

.camera-title {
  margin-bottom: var(--space-sm);
}

.camera-container {
  flex: 1;
  display: flex;
  flex-direction: column;
  gap: var(--space-lg);
}

/* Camera Viewfinder */
.camera-viewfinder {
  position: relative;
  width: 100%;
  max-width: 500px;
  margin: 0 auto;
  aspect-ratio: 4/3;
  border-radius: var(--radius-2xl);
  overflow: hidden;
  background: #000;
  box-shadow: var(--shadow-xl);
}

.camera-video,
.captured-image {
  width: 100%;
  height: 100%;
  object-fit: cover;
}

/* Camera Overlay */
.camera-overlay {
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  display: flex;
  flex-direction: column;
  justify-content: space-between;
  align-items: center;
  padding: var(--space-lg);
  background: linear-gradient(
    to bottom,
    rgba(0, 0, 0, 0.3) 0%,
    transparent 30%,
    transparent 70%,
    rgba(0, 0, 0, 0.3) 100%
  );
}

/* Focus Frame */
.focus-frame {
  position: relative;
  width: 80%;
  height: 60%;
  border: 2px solid rgba(255, 255, 255, 0.8);
  border-radius: var(--radius-lg);
  background: transparent;
}

.focus-corner {
  position: absolute;
  width: 20px;
  height: 20px;
  border: 3px solid var(--color-primary-electric-blue);
}

.focus-corner-tl {
  top: -3px;
  left: -3px;
  border-right: none;
  border-bottom: none;
  border-radius: var(--radius-sm) 0 0 0;
}

.focus-corner-tr {
  top: -3px;
  right: -3px;
  border-left: none;
  border-bottom: none;
  border-radius: 0 var(--radius-sm) 0 0;
}

.focus-corner-bl {
  bottom: -3px;
  left: -3px;
  border-right: none;
  border-top: none;
  border-radius: 0 0 0 var(--radius-sm);
}

.focus-corner-br {
  bottom: -3px;
  right: -3px;
  border-left: none;
  border-top: none;
  border-radius: 0 0 var(--radius-sm) 0;
}

/* Guidance Text */
.guidance-text {
  background: rgba(0, 0, 0, 0.7);
  padding: var(--space-sm) var(--space-md);
  border-radius: var(--radius-full);
  backdrop-filter: blur(10px);
}

/* Loading States */
.camera-loading,
.camera-capturing {
  position: absolute;
  top: 50%;
  left: 50%;
  transform: translate(-50%, -50%);
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: var(--space-md);
  background: rgba(0, 0, 0, 0.8);
  padding: var(--space-xl);
  border-radius: var(--radius-xl);
  backdrop-filter: blur(10px);
}

.camera-loading-spinner {
  width: 40px;
  height: 40px;
  border: 3px solid rgba(255, 255, 255, 0.3);
  border-top: 3px solid var(--color-primary-electric-blue);
  border-radius: 50%;
  animation: camera-spin 1s linear infinite;
}

@keyframes camera-spin {
  0% { transform: rotate(0deg); }
  100% { transform: rotate(360deg); }
}

/* Capture Flash Effect */
.capture-flash {
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: white;
  opacity: 0;
  animation: capture-flash 0.3s ease-out;
}

@keyframes capture-flash {
  0% { opacity: 0; }
  50% { opacity: 0.8; }
  100% { opacity: 0; }
}

/* Camera Controls */
.camera-controls {
  max-width: 500px;
  margin: 0 auto;
  width: 100%;
}

/* Error Display */
.camera-error {
  text-align: center;
  padding: var(--space-xl);
  background: var(--color-surface-light);
  border-radius: var(--radius-xl);
  border-left: 4px solid var(--color-error);
  box-shadow: var(--shadow-md);
}

/* Tips Section */
.camera-tips {
  background: var(--color-surface-light);
  border-radius: var(--radius-xl);
  padding: var(--space-lg);
  box-shadow: var(--shadow-md);
  border-left: 4px solid var(--color-accent-yellow);
}

.tips-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: var(--space-md);
}

.tips-close {
  background: none;
  border: none;
  font-size: 1.5rem;
  cursor: pointer;
  color: var(--color-text-secondary);
  padding: var(--space-xs);
  border-radius: var(--radius-sm);
  transition: background-color 0.2s ease;
}

.tips-close:hover {
  background: var(--color-bg-light);
}

.tip-card {
  background: var(--color-bg-light);
  border: none;
  box-shadow: none;
  padding: var(--space-md);
}

.tip-content {
  display: flex;
  align-items: center;
  gap: var(--space-sm);
  text-align: left;
}

.tip-icon {
  font-size: 1.25rem;
  flex-shrink: 0;
}

/* Cancel Button */
.camera-cancel {
  text-align: center;
  margin-top: var(--space-lg);
}

/* Responsive Design */
@media (max-width: 768px) {
  .camera-interface {
    padding: var(--space-md);
  }

  .camera-viewfinder {
    aspect-ratio: 3/4;
  }

  .camera-overlay {
    padding: var(--space-md);
  }

  .focus-frame {
    width: 90%;
    height: 70%;
  }

  .tips-header {
    flex-direction: column;
    gap: var(--space-sm);
    align-items: flex-start;
  }
}

@media (max-width: 480px) {
  .camera-interface {
    padding: var(--space-sm);
    gap: var(--space-md);
  }

  .camera-loading,
  .camera-capturing {
    padding: var(--space-lg);
  }

  .tip-content {
    flex-direction: column;
    text-align: center;
    gap: var(--space-xs);
  }
}

/* Accessibility */
@media (prefers-reduced-motion: reduce) {
  .camera-loading-spinner {
    animation: none;
  }

  .capture-flash {
    animation: none;
  }
}
"#.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_camera_states() {
        let inactive = CameraState::Inactive;
        let active = CameraState::Active;
        assert_ne!(inactive, active);
    }

    #[test]
    fn test_camera_facing() {
        let user = CameraFacing::User;
        let environment = CameraFacing::Environment;
        assert_ne!(user, environment);
    }

    #[test]
    fn test_css_generation() {
        let css = generate_camera_css();
        assert!(css.contains("camera-interface"));
        assert!(css.contains("camera-viewfinder"));
        assert!(css.contains("@keyframes"));
    }
}
