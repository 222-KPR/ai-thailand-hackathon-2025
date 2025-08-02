use wasm_bindgen::prelude::*;
use yew::prelude::*;

// Initialize WASM
#[wasm_bindgen(start)]
pub fn run_app() {
    wasm_logger::init(wasm_logger::Config::default());
    console_error_panic_hook::set_once();
    yew::Renderer::<App>::new().render();
}

#[function_component(App)]
fn app() -> Html {
    let api_status = use_state(|| "Checking...".to_string());
    let chat_response = use_state(|| "".to_string());
    let message_input = use_state(|| "".to_string());

    // Check API health on component mount
    {
        let api_status = api_status.clone();
        use_effect_with((), move |_| {
            let api_status = api_status.clone();
            wasm_bindgen_futures::spawn_local(async move {
                match check_api_health().await {
                    Ok(status) => api_status.set(format!("✅ API Healthy: {status}")),
                    Err(e) => api_status.set(format!("❌ API Error: {e}")),
                }
            });
            || ()
        });
    }

    let on_message_change = {
        let message_input = message_input.clone();
        Callback::from(move |e: Event| {
            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
            message_input.set(input.value());
        })
    };

    let on_send_message = {
        let message_input = message_input.clone();
        let chat_response = chat_response.clone();
        Callback::from(move |_: MouseEvent| {
            let message = (*message_input).clone();
            let chat_response = chat_response.clone();
            let message_input = message_input.clone();

            if !message.trim().is_empty() {
                chat_response.set("Sending...".to_string());
                wasm_bindgen_futures::spawn_local(async move {
                    match send_chat_message(&message).await {
                        Ok(response) => {
                            chat_response.set(response);
                            message_input.set("".to_string());
                        }
                        Err(e) => chat_response.set(format!("Error: {e}")),
                    }
                });
            }
        })
    };

    html! {
        <div style="font-family: Arial, sans-serif; max-width: 800px; margin: 0 auto; padding: 20px;">
            <header style="text-align: center; margin-bottom: 30px;">
                <h1 style="color: #2563eb;">{"🌾 AI4Thai Crop Guardian"}</h1>
                <p style="color: #6b7280;">{"AI-powered crop disease detection and advisory system"}</p>
                <div style="background: #f3f4f6; padding: 10px; border-radius: 8px; margin: 10px 0;">
                    <strong>{"API Status: "}</strong>{&*api_status}
                </div>
            </header>

            <main>
                <section style="background: white; border: 1px solid #e5e7eb; border-radius: 12px; padding: 20px; margin-bottom: 20px; box-shadow: 0 1px 3px rgba(0,0,0,0.1);">
                    <h2 style="color: #1f2937; margin-bottom: 15px;">{"💬 Chat with AI Assistant"}</h2>

                    <div style="margin-bottom: 15px;">
                        <input
                            type="text"
                            placeholder="Ask about crop diseases, treatments, or farming advice..."
                            value={(*message_input).clone()}
                            onchange={on_message_change}
                            style="width: 100%; padding: 12px; border: 1px solid #d1d5db; border-radius: 8px; font-size: 16px;"
                        />
                    </div>

                    <button
                        onclick={on_send_message}
                        disabled={message_input.trim().is_empty()}
                        style="background: #2563eb; color: white; border: none; padding: 12px 24px; border-radius: 8px; cursor: pointer; font-size: 16px; font-weight: 600;"
                    >
                        {"Send Message"}
                    </button>

                    {if !chat_response.is_empty() {
                        html! {
                            <div style="margin-top: 20px; padding: 15px; background: #f9fafb; border-radius: 8px; border-left: 4px solid #10b981;">
                                <h3 style="color: #065f46; margin: 0 0 10px 0;">{"🤖 AI Response:"}</h3>
                                <p style="margin: 0; line-height: 1.6;">{&*chat_response}</p>
                            </div>
                        }
                    } else {
                        html! {}
                    }}
                </section>

                <section style="background: white; border: 1px solid #e5e7eb; border-radius: 12px; padding: 20px; box-shadow: 0 1px 3px rgba(0,0,0,0.1);">
                    <h2 style="color: #1f2937; margin-bottom: 15px;">{"🚀 Features"}</h2>
                    <div style="display: grid; grid-template-columns: repeat(auto-fit, minmax(250px, 1fr)); gap: 15px;">
                        <div style="padding: 15px; background: #fef3c7; border-radius: 8px;">
                            <h3 style="color: #92400e; margin: 0 0 8px 0;">{"🔍 Disease Detection"}</h3>
                            <p style="color: #78350f; margin: 0; font-size: 14px;">{"Upload crop images for AI-powered disease identification"}</p>
                        </div>
                        <div style="padding: 15px; background: #dcfce7; border-radius: 8px;">
                            <h3 style="color: #166534; margin: 0 0 8px 0;">{"💬 Smart Chat"}</h3>
                            <p style="color: #15803d; margin: 0; font-size: 14px;">{"Get farming advice and treatment recommendations"}</p>
                        </div>
                        <div style="padding: 15px; background: #dbeafe; border-radius: 8px;">
                            <h3 style="color: #1d4ed8; margin: 0 0 8px 0;">{"🌐 Thai Support"}</h3>
                            <p style="color: #1e40af; margin: 0; font-size: 14px;">{"Native Thai language processing and responses"}</p>
                        </div>
                    </div>
                </section>
            </main>

            <footer style="text-align: center; margin-top: 30px; padding-top: 20px; border-top: 1px solid #e5e7eb; color: #6b7280;">
                <p>{"Made with ❤️ for Thai farmers by KPR team for AI Thailand Hackathon 2025"}</p>
            </footer>
        </div>
    }
}

// API functions
async fn check_api_health() -> Result<String, String> {
    let response = gloo_net::http::Request::get("http://localhost:3000/health")
        .send()
        .await
        .map_err(|e| format!("Network error: {e:?}"))?;

    if response.ok() {
        let text = response
            .text()
            .await
            .map_err(|e| format!("Parse error: {e:?}"))?;
        Ok(text)
    } else {
        Err(format!("HTTP {}", response.status()))
    }
}

async fn send_chat_message(message: &str) -> Result<String, String> {
    let body = serde_json::json!({
        "message": message,
        "conversation_id": null
    });

    let response = gloo_net::http::Request::post("http://localhost:3000/api/v1/chat")
        .header("Content-Type", "application/json")
        .body(body.to_string())
        .map_err(|e| format!("Request build error: {e:?}"))?
        .send()
        .await
        .map_err(|e| format!("Network error: {e:?}"))?;

    if response.ok() {
        let json: serde_json::Value = response
            .json()
            .await
            .map_err(|e| format!("Parse error: {e:?}"))?;

        if let Some(data) = json.get("data") {
            if let Some(response_text) = data.get("response") {
                return Ok(response_text.as_str().unwrap_or("No response").to_string());
            }
        }

        Ok("Received response but couldn't parse it".to_string())
    } else {
        Err(format!("HTTP {}", response.status()))
    }
}
