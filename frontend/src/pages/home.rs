// Homepage Component - Simple Version

use yew::prelude::*;
use yew_router::prelude::*;
use crate::app::Route;

#[function_component(HomePage)]
pub fn home_page() -> Html {

    // Navigation callbacks
    let navigator = use_navigator().unwrap();
    let navigate_to_camera = {
        let navigator = navigator.clone();
        Callback::from(move |_| {
            navigator.push(&Route::Camera);
        })
    };

    html! {
        <div class="home-page">
            <div class="hero-section">
                <h1>{"🌾 AI4Thai Crop Guardian"}</h1>
                <p>{"ระบบตรวจจับโรคพืชด้วย AI สำหรับเกษตรกรไทย"}</p>

                <button class="camera-button" onclick={navigate_to_camera}>
                    <span class="icon">{"📷"}</span>
                    <span class="text">{"เริ่มตรวจวินิจฉัยโรคพืช"}</span>
                </button>

                <div class="features">
                    <div class="feature-card">
                        <h3>{"🔍 Disease Detection"}</h3>
                        <p>{"Upload crop images for AI-powered disease identification"}</p>
                    </div>
                    <div class="feature-card">
                        <h3>{"💬 Smart Chat"}</h3>
                        <p>{"Get farming advice and treatment recommendations"}</p>
                    </div>
                    <div class="feature-card">
                        <h3>{"🌐 Thai Support"}</h3>
                        <p>{"Native Thai language processing and responses"}</p>
                    </div>
                </div>
            </div>
        </div>
    }
}
