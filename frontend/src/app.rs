use yew::prelude::*;
use yewdux::prelude::*;

use crate::components::chat::ChatInterface;
use crate::components::header::Header;
use crate::store::AppState;
use crate::i18n::I18nContext;

#[function_component(App)]
pub fn app() -> Html {
    let (state, _) = use_store::<AppState>();
    
    // Initialize i18n context
    let i18n_ctx = use_memo(|_| I18nContext::new(state.language.clone()), ());
    
    html! {
        <ContextProvider<I18nContext> context={(*i18n_ctx).clone()}>
            <div class="app">
                <Header />
                <main class="main-content">
                    <ChatInterface />
                </main>
            </div>
        </ContextProvider<I18nContext>>
    }
}