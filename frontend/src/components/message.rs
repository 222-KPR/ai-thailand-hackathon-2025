use yew::prelude::*;
use chrono::{DateTime, Utc};
use shared::ChatMessage;

use crate::i18n::I18nContext;

#[derive(Properties, PartialEq)]
pub struct MessageProps {
    pub message: ChatMessage,
    pub is_bot: bool,
}

#[function_component(MessageBubble)]
pub fn message_bubble(props: &MessageProps) -> Html {
    let i18n = use_context::<I18nContext>().expect("I18nContext not found");
    
    let message_class = if props.is_bot {
        classes!("message", "message-bot")
    } else {
        classes!("message", "message-user")
    };
    
    let formatted_time = format_time(&props.message.timestamp);
    
    html! {
        <div class={message_class}>
            <div class="message-avatar">
                {if props.is_bot {
                    html! { <span class="avatar-bot">{"🤖"}</span> }
                } else {
                    html! { <span class="avatar-user">{"👤"}</span> }
                }}
            </div>
            
            <div class="message-content">
                <div class="message-bubble">
                    <div class="message-text">
                        {render_message_content(&props.message.content)}
                    </div>
                    <div class="message-time">
                        {formatted_time}
                    </div>
                </div>
            </div>
        </div>
    }
}

fn render_message_content(content: &str) -> Html {
    // Check if content contains structured data (JSON)
    if content.starts_with('{') && content.ends_with('}') {
        // Try to parse and render structured content
        if let Ok(structured) = serde_json::from_str::<serde_json::Value>(content) {
            return render_structured_content(&structured);
        }
    }
    
    // Render as plain text with line breaks
    html! {
        <>
            {content.lines().enumerate().map(|(i, line)| {
                html! {
                    <>
                        {if i > 0 { html! { <br /> } } else { html! {} }}
                        {line}
                    </>
                }
            }).collect::<Html>()}
        </>
    }
}

fn render_structured_content(data: &serde_json::Value) -> Html {
    match data {
        serde_json::Value::Object(obj) => {
            if let Some(message_type) = obj.get("type").and_then(|v| v.as_str()) {
                match message_type {
                    "diagnosis" => render_diagnosis_content(obj),
                    "treatment" => render_treatment_content(obj),
                    "error" => render_error_content(obj),
                    _ => render_generic_content(obj),
                }
            } else {
                render_generic_content(obj)
            }
        }
        _ => html! { {data.to_string()} }
    }
}

fn render_diagnosis_content(obj: &serde_json::Map<String, serde_json::Value>) -> Html {
    let disease = obj.get("disease").and_then(|v| v.as_str()).unwrap_or("Unknown");
    let confidence = obj.get("confidence").and_then(|v| v.as_f64()).unwrap_or(0.0);
    let severity = obj.get("severity").and_then(|v| v.as_str()).unwrap_or("Unknown");
    
    html! {
        <div class="diagnosis-result">
            <div class="diagnosis-header">
                <span class="diagnosis-icon">{"🔬"}</span>
                <h4>{"ผลการวินิจฉัย"}</h4>
            </div>
            <div class="diagnosis-details">
                <div class="diagnosis-item">
                    <strong>{"โรค: "}</strong>
                    <span>{disease}</span>
                </div>
                <div class="diagnosis-item">
                    <strong>{"ความเชื่อมั่น: "}</strong>
                    <span class="confidence">{format!("{:.1}%", confidence * 100.0)}</span>
                </div>
                <div class="diagnosis-item">
                    <strong>{"ความรุนแรง: "}</strong>
                    <span class={classes!("severity", format!("severity-{}", severity.to_lowercase()))}>
                        {severity}
                    </span>
                </div>
            </div>
        </div>
    }
}

fn render_treatment_content(obj: &serde_json::Map<String, serde_json::Value>) -> Html {
    let advice = obj.get("advice").and_then(|v| v.as_str()).unwrap_or("");
    let steps = obj.get("steps").and_then(|v| v.as_array()).unwrap_or(&Vec::new());
    let cost_min = obj.get("cost_min").and_then(|v| v.as_f64()).unwrap_or(0.0);
    let cost_max = obj.get("cost_max").and_then(|v| v.as_f64()).unwrap_or(0.0);
    
    html! {
        <div class="treatment-advice">
            <div class="treatment-header">
                <span class="treatment-icon">{"💊"}</span>
                <h4>{"คำแนะนำการรักษา"}</h4>
            </div>
            
            {if !advice.is_empty() {
                html! {
                    <div class="treatment-advice-text">
                        <p>{advice}</p>
                    </div>
                }
            } else {
                html! {}
            }}
            
            {if !steps.is_empty() {
                html! {
                    <div class="treatment-steps">
                        <h5>{"ขั้นตอนการรักษา:"}</h5>
                        <ol class="steps-list">
                            {steps.iter().enumerate().map(|(i, step)| {
                                html! {
                                    <li class="step-item">
                                        {step.as_str().unwrap_or(&format!("Step {}", i + 1))}
                                    </li>
                                }
                            }).collect::<Html>()}
                        </ol>
                    </div>
                }
            } else {
                html! {}
            }}
            
            {if cost_min > 0.0 && cost_max > 0.0 {
                html! {
                    <div class="treatment-cost">
                        <div class="cost-item">
                            <strong>{"ค่าใช้จ่ายประมาณ: "}</strong>
                            <span class="cost-range">
                                {format!("{:.0}-{:.0} บาท", cost_min, cost_max)}
                            </span>
                        </div>
                    </div>
                }
            } else {
                html! {}
            }}
        </div>
    }
}

fn render_error_content(obj: &serde_json::Map<String, serde_json::Value>) -> Html {
    let message = obj.get("message").and_then(|v| v.as_str()).unwrap_or("เกิดข้อผิดพลาด");
    
    html! {
        <div class="error-message">
            <div class="error-header">
                <span class="error-icon">{"⚠️"}</span>
                <h4>{"ข้อผิดพลาด"}</h4>
            </div>
            <p class="error-text">{message}</p>
        </div>
    }
}

fn render_generic_content(obj: &serde_json::Map<String, serde_json::Value>) -> Html {
    html! {
        <div class="generic-content">
            {obj.iter().map(|(key, value)| {
                html! {
                    <div class="content-item">
                        <strong>{format!("{}: ", key)}</strong>
                        <span>{value.to_string()}</span>
                    </div>
                }
            }).collect::<Html>()}
        </div>
    }
}

fn format_time(timestamp: &DateTime<Utc>) -> String {
    let local_time = timestamp.format("%H:%M");
    format!("{}", local_time)
}