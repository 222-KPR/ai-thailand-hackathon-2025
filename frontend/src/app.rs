use yew::prelude::*;
use yew_router::prelude::*;
use shared::{ChatMessage, Language};
use uuid::Uuid;

use crate::components::{
    chat::ChatInterface,
    header::Header,
};
use crate::i18n::I18nContext;

/// Main application routes
#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[not_found]
    #[at("/404")]
    NotFound,
}

/// Global application state
#[derive(Clone, PartialEq)]
pub struct AppState {
    pub language: Language,
    pub conversation_id: Uuid,
    pub messages: Vec<ChatMessage>,
    pub is_loading: bool,
    pub error_message: Option<String>,
    pub user_authenticated: bool,
    pub api_health: bool,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            language: Language::Thai,
            conversation_id: Uuid::new_v4(),
            messages: Vec::new(),
            is_loading: false,
            error_message: None,
            user_authenticated: false,
            api_health: true,
        }
    }
}

impl AppState {
    /// Add a new message to the conversation
    pub fn add_message(&mut self, message: ChatMessage) {
        self.messages.push(message);
    }

    /// Clear all messages
    pub fn clear_messages(&mut self) {
        self.messages.clear();
        self.conversation_id = Uuid::new_v4();
    }

    /// Set loading state
    pub fn set_loading(&mut self, loading: bool) {
        self.is_loading = loading;
    }

    /// Set error message
    pub fn set_error(&mut self, error: Option<String>) {
        self.error_message = error;
    }

    /// Toggle language
    pub fn toggle_language(&mut self) {
        self.language = match self.language {
            Language::Thai => Language::English,
            Language::English => Language::Thai,
        };
    }
}

/// Route switch component
fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <ChatInterface /> },
        Route::NotFound => html! { <div>{"Page Not Found"}</div> },
    }
}

/// Main application component
#[function_component(App)]
pub fn app() -> Html {
    let state = use_state(AppState::default);
    
    // Initialize i18n context
    let i18n_ctx = use_memo(
        state.language.clone(),
        |language| I18nContext::new(language.clone()),
    );
    
    // Check API health on mount
    {
        let state = state.clone();
        use_effect_with((), move |_| {
            let state = state.clone();
            wasm_bindgen_futures::spawn_local(async move {
                match crate::services::api::ApiService::check_health().await {
                    Ok(_) => {
                        let mut new_state = (*state).clone();
                        new_state.api_health = true;
                        state.set(new_state);
                    }
                    Err(_) => {
                        let mut new_state = (*state).clone();
                        new_state.api_health = false;
                        new_state.error_message = Some("API service unavailable".to_string());
                        state.set(new_state);
                    }
                }
            });
        });
    }
    
    html! {
        <BrowserRouter>
            <ContextProvider<UseStateHandle<AppState>> context={state.clone()}>
                <ContextProvider<I18nContext> context={(*i18n_ctx).clone()}>
                    <div class="app">
                        <Header />
                        
                        {if !state.api_health {
                            html! {
                                <div class="api-error-banner">
                                    <span class="error-icon">{"⚠️"}</span>
                                    <span>{"API service is currently unavailable. Some features may not work."}</span>
                                </div>
                            }
                        } else {
                            html! {}
                        }}
                        
                        <main class="main-content">
                            <Switch<Route> render={switch} />
                        </main>
                        
                        <footer class="app-footer">
                            <p>{"AI4Thai Crop Guardian © 2025"}</p>
                        </footer>
                    </div>
                </ContextProvider<I18nContext>>
            </ContextProvider<UseStateHandle<AppState>>>
        </BrowserRouter>
    }
}
