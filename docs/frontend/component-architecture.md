# Component Architecture

This document outlines the component architecture for the AI4Thai Crop Guardian frontend, built with Yew WebAssembly and following modern React-like patterns.

## 🏗️ Architecture Principles

### Component Design Principles
- **Single Responsibility**: Each component has one clear purpose
- **Composition over Inheritance**: Build complex UIs by composing simple components
- **Props Down, Events Up**: Data flows down through props, events bubble up
- **Immutable State**: State changes create new state rather than mutating existing
- **Predictable Rendering**: Same props always produce the same output

### Yew-Specific Patterns
- **Function Components**: Prefer function components over struct components
- **Hooks Pattern**: Use hooks for state management and side effects
- **Context API**: Share state across component trees
- **Callback Props**: Handle user interactions and component communication

## 📁 Component Organization

### Directory Structure
```
src/components/
├── layout/                  # Layout and structural components
│   ├── mod.rs
│   ├── app_layout.rs       # Main application layout
│   ├── page_layout.rs      # Individual page layouts
│   ├── bento_grid.rs       # Bento grid system
│   ├── sidebar.rs          # Navigation sidebar
│   └── header.rs           # Application header
├── ui/                     # Reusable UI components
│   ├── mod.rs
│   ├── button.rs           # Button variants
│   ├── input.rs            # Form inputs
│   ├── card.rs             # Card components
│   ├── modal.rs            # Modal dialogs
│   ├── loading.rs          # Loading indicators
│   └── status_card.rs      # Status display cards
├── chat/                   # Chat-specific components
│   ├── mod.rs
│   ├── chat_window.rs      # Main chat interface
│   ├── message_bubble.rs   # Individual messages
│   ├── input_bar.rs        # Message input
│   ├── typing_indicator.rs # Typing status
│   └── voice_controls.rs   # Voice input controls
├── camera/                 # Camera and image components
│   ├── mod.rs
│   ├── camera_capture.rs   # Camera interface
│   ├── image_preview.rs    # Image preview
│   ├── crop_selector.rs    # Crop type selection
│   └── upload_progress.rs  # Upload progress
├── diagnosis/              # Disease diagnosis components
│   ├── mod.rs
│   ├── result_display.rs   # Diagnosis results
│   ├── confidence_meter.rs # Confidence visualization
│   ├── treatment_plan.rs   # Treatment recommendations
│   └── history_list.rs     # Diagnosis history
└── forms/                  # Form components
    ├── mod.rs
    ├── login_form.rs       # User login
    ├── register_form.rs    # User registration
    ├── profile_form.rs     # Profile editing
    └── feedback_form.rs    # User feedback
```

## 🎯 Component Types

### 1. Layout Components
Structural components that define page layout and navigation.

```rust
// Example: App Layout Component
use yew::prelude::*;
use crate::components::layout::{Header, Sidebar};

#[derive(Properties, PartialEq)]
pub struct AppLayoutProps {
    pub children: Children,
    #[prop_or_default]
    pub show_sidebar: bool,
}

#[function_component(AppLayout)]
pub fn app_layout(props: &AppLayoutProps) -> Html {
    html! {
        <div class="app-layout">
            <Header />
            <div class="app-content">
                if props.show_sidebar {
                    <Sidebar />
                }
                <main class="main-content">
                    { for props.children.iter() }
                </main>
            </div>
        </div>
    }
}
```

### 2. UI Components
Reusable interface elements with consistent styling and behavior.

```rust
// Example: Button Component with Dopamine Colors
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ButtonProps {
    pub children: Children,
    #[prop_or(ButtonVariant::Primary)]
    pub variant: ButtonVariant,
    #[prop_or(ButtonSize::Medium)]
    pub size: ButtonSize,
    #[prop_or_default]
    pub disabled: bool,
    #[prop_or_default]
    pub loading: bool,
    #[prop_or_default]
    pub onclick: Callback<MouseEvent>,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub icon: Option<String>,
}

#[derive(PartialEq, Clone)]
pub enum ButtonVariant {
    Primary,    // Electric Blue
    Secondary,  // Vibrant Orange
    Accent,     // Energetic Pink
    Success,    // Lime Green
    Warning,    // Yellow
    Error,      // Red
    Ghost,      // Transparent
}

#[derive(PartialEq, Clone)]
pub enum ButtonSize {
    Small,
    Medium,
    Large,
}

#[function_component(Button)]
pub fn button(props: &ButtonProps) -> Html {
    let onclick = props.onclick.clone();
    let handle_click = Callback::from(move |e: MouseEvent| {
        if !props.disabled && !props.loading {
            onclick.emit(e);
        }
    });

    let class = classes!(
        "btn",
        match props.variant {
            ButtonVariant::Primary => "btn--primary",
            ButtonVariant::Secondary => "btn--secondary",
            ButtonVariant::Accent => "btn--accent",
            ButtonVariant::Success => "btn--success",
            ButtonVariant::Warning => "btn--warning",
            ButtonVariant::Error => "btn--error",
            ButtonVariant::Ghost => "btn--ghost",
        },
        match props.size {
            ButtonSize::Small => "btn--sm",
            ButtonSize::Medium => "btn--md",
            ButtonSize::Large => "btn--lg",
        },
        props.disabled.then_some("btn--disabled"),
        props.loading.then_some("btn--loading"),
        props.class.clone()
    );

    html! {
        <button
            {class}
            disabled={props.disabled || props.loading}
            onclick={handle_click}
        >
            if props.loading {
                <span class="btn__spinner" />
            }
            if let Some(icon) = &props.icon {
                <i class={format!("icon-{}", icon)} />
            }
            <span class="btn__content">
                { for props.children.iter() }
            </span>
        </button>
    }
}
```

### 3. Feature Components
Components that implement specific application features.

```rust
// Example: Chat Window Component
use yew::prelude::*;
use yewdux::prelude::*;
use crate::stores::chat_store::{ChatStore, ChatAction};
use crate::components::chat::{MessageBubble, InputBar};

#[derive(Properties, PartialEq)]
pub struct ChatWindowProps {
    pub conversation_id: String,
}

#[function_component(ChatWindow)]
pub fn chat_window(props: &ChatWindowProps) -> Html {
    let chat_state = use_store_value::<ChatStore>();
    let chat_dispatch = use_dispatch::<ChatStore>();
    
    // Load conversation on mount
    {
        let conversation_id = props.conversation_id.clone();
        let dispatch = chat_dispatch.clone();
        use_effect_with(conversation_id.clone(), move |_| {
            dispatch.apply(ChatAction::LoadConversation(conversation_id));
            || {}
        });
    }
    
    // Auto-scroll to bottom when new messages arrive
    let messages_ref = use_node_ref();
    {
        let messages_ref = messages_ref.clone();
        let message_count = chat_state.messages.len();
        use_effect_with(message_count, move |_| {
            if let Some(element) = messages_ref.cast::<web_sys::Element>() {
                element.set_scroll_top(element.scroll_height());
            }
            || {}
        });
    }
    
    let on_send_message = {
        let dispatch = chat_dispatch.clone();
        let conversation_id = props.conversation_id.clone();
        Callback::from(move |message: String| {
            dispatch.apply(ChatAction::SendMessage {
                conversation_id: conversation_id.clone(),
                content: message,
            });
        })
    };

    html! {
        <div class="chat-window">
            <div class="chat-header">
                <h2>{ "AI Agricultural Assistant" }</h2>
                <span class="chat-status">
                    if chat_state.is_typing {
                        { "AI is typing..." }
                    } else {
                        { "Online" }
                    }
                </span>
            </div>
            
            <div class="chat-messages" ref={messages_ref}>
                { for chat_state.messages.iter().map(|message| {
                    html! {
                        <MessageBubble
                            key={message.id.clone()}
                            message={message.clone()}
                        />
                    }
                })}
                
                if chat_state.is_loading {
                    <div class="typing-indicator">
                        <span class="dot"></span>
                        <span class="dot"></span>
                        <span class="dot"></span>
                    </div>
                }
            </div>
            
            <InputBar
                on_send={on_send_message}
                disabled={chat_state.is_loading}
                placeholder="Ask about your crops..."
            />
        </div>
    }
}
```

## 🔄 State Management Patterns

### Local State with Hooks
```rust
use yew::prelude::*;

#[function_component(Counter)]
pub fn counter() -> Html {
    let count = use_state(|| 0);
    
    let increment = {
        let count = count.clone();
        Callback::from(move |_| {
            count.set(*count + 1);
        })
    };
    
    html! {
        <div>
            <p>{ format!("Count: {}", *count) }</p>
            <button onclick={increment}>{ "Increment" }</button>
        </div>
    }
}
```

### Global State with Yewdux
```rust
use yewdux::prelude::*;

#[derive(Default, Clone, PartialEq, Store)]
pub struct AppState {
    pub user: Option<User>,
    pub theme: Theme,
    pub language: Language,
}

#[derive(Clone, PartialEq)]
pub enum AppAction {
    SetUser(User),
    SetTheme(Theme),
    SetLanguage(Language),
}

impl Reducer<AppState> for AppAction {
    fn apply(self, mut state: Rc<AppState>) -> Rc<AppState> {
        let state = Rc::make_mut(&mut state);
        match self {
            AppAction::SetUser(user) => state.user = Some(user),
            AppAction::SetTheme(theme) => state.theme = theme,
            AppAction::SetLanguage(language) => state.language = language,
        }
        state.into()
    }
}

// Usage in component
#[function_component(UserProfile)]
pub fn user_profile() -> Html {
    let state = use_store_value::<AppState>();
    let dispatch = use_dispatch::<AppState>();
    
    let change_theme = {
        let dispatch = dispatch.clone();
        Callback::from(move |theme: Theme| {
            dispatch.apply(AppAction::SetTheme(theme));
        })
    };
    
    html! {
        <div>
            if let Some(user) = &state.user {
                <h1>{ format!("Welcome, {}", user.name) }</h1>
            }
            <button onclick={change_theme}>{ "Toggle Theme" }</button>
        </div>
    }
}
```

## 🎨 Styling Patterns

### CSS Classes with Yew
```rust
use yew::prelude::*;

#[function_component(StyledComponent)]
pub fn styled_component() -> Html {
    let is_active = use_state(|| false);
    
    let class = classes!(
        "base-class",
        "another-class",
        is_active.then_some("active"),
        "conditional-class".to_string()
    );
    
    html! {
        <div {class}>
            { "Styled content" }
        </div>
    }
}
```

### CSS Custom Properties
```rust
#[function_component(ThemedCard)]
pub fn themed_card(props: &ThemedCardProps) -> Html {
    let style = format!(
        "--card-color: {}; --card-size: {}px",
        props.color,
        props.size
    );
    
    html! {
        <div class="themed-card" {style}>
            { &props.children }
        </div>
    }
}
```

## 🔗 Component Communication

### Parent-Child Communication
```rust
// Parent Component
#[function_component(Parent)]
pub fn parent() -> Html {
    let selected_item = use_state(|| None::<String>);
    
    let on_item_select = {
        let selected_item = selected_item.clone();
        Callback::from(move |item: String| {
            selected_item.set(Some(item));
        })
    };
    
    html! {
        <div>
            <ItemList on_select={on_item_select} />
            if let Some(item) = selected_item.as_ref() {
                <ItemDetails item={item.clone()} />
            }
        </div>
    }
}

// Child Component
#[derive(Properties, PartialEq)]
pub struct ItemListProps {
    pub on_select: Callback<String>,
}

#[function_component(ItemList)]
pub fn item_list(props: &ItemListProps) -> Html {
    let items = vec!["Item 1", "Item 2", "Item 3"];
    
    html! {
        <ul>
            { for items.iter().map(|item| {
                let on_select = props.on_select.clone();
                let item = item.to_string();
                html! {
                    <li key={item.clone()}>
                        <button onclick={
                            let item = item.clone();
                            move |_| on_select.emit(item.clone())
                        }>
                            { item }
                        </button>
                    </li>
                }
            })}
        </ul>
    }
}
```

### Context API for Deep Prop Drilling
```rust
use yew::prelude::*;
use std::rc::Rc;

#[derive(Clone, PartialEq)]
pub struct ThemeContext {
    pub theme: Theme,
    pub set_theme: Callback<Theme>,
}

#[function_component(ThemeProvider)]
pub fn theme_provider(props: &ChildrenProps) -> Html {
    let theme = use_state(|| Theme::Light);
    
    let set_theme = {
        let theme = theme.clone();
        Callback::from(move |new_theme: Theme| {
            theme.set(new_theme);
        })
    };
    
    let context = ThemeContext {
        theme: (*theme).clone(),
        set_theme,
    };
    
    html! {
        <ContextProvider<ThemeContext> context={context}>
            { for props.children.iter() }
        </ContextProvider<ThemeContext>>
    }
}

// Usage in nested component
#[function_component(ThemedButton)]
pub fn themed_button() -> Html {
    let theme_context = use_context::<ThemeContext>()
        .expect("ThemeContext not found");
    
    let class = match theme_context.theme {
        Theme::Light => "btn btn--light",
        Theme::Dark => "btn btn--dark",
    };
    
    html! {
        <button {class}>
            { "Themed Button" }
        </button>
    }
}
```

## 🔄 Lifecycle and Effects

### Effect Hooks
```rust
use yew::prelude::*;
use gloo_timers::future::TimeoutFuture;

#[function_component(DataFetcher)]
pub fn data_fetcher() -> Html {
    let data = use_state(|| None::<String>);
    let loading = use_state(|| true);
    
    // Fetch data on mount
    {
        let data = data.clone();
        let loading = loading.clone();
        use_effect_with((), move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                // Simulate API call
                TimeoutFuture::new(1000).await;
                data.set(Some("Fetched data".to_string()));
                loading.set(false);
            });
            || {}
        });
    }
    
    html! {
        <div>
            if *loading {
                <p>{ "Loading..." }</p>
            } else if let Some(data) = data.as_ref() {
                <p>{ data }</p>
            } else {
                <p>{ "No data" }</p>
            }
        </div>
    }
}
```

## 🧪 Testing Patterns

### Component Testing
```rust
#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen_test::*;
    use yew::platform::spawn_local;
    
    wasm_bindgen_test_configure!(run_in_browser);
    
    #[wasm_bindgen_test]
    fn test_button_click() {
        let div = gloo_utils::document().create_element("div").unwrap();
        let clicked = Rc::new(RefCell::new(false));
        
        let onclick = {
            let clicked = clicked.clone();
            Callback::from(move |_| {
                *clicked.borrow_mut() = true;
            })
        };
        
        let props = ButtonProps {
            children: html! { "Click me" }.into(),
            onclick,
            ..Default::default()
        };
        
        yew::Renderer::<Button>::with_root_and_props(div.clone(), props)
            .render();
        
        let button = div.query_selector("button").unwrap().unwrap();
        button.click();
        
        assert!(*clicked.borrow());
    }
}
```

## 📚 Best Practices

### Component Design
1. **Keep components small and focused**
2. **Use descriptive prop names**
3. **Provide default values for optional props**
4. **Handle loading and error states**
5. **Make components accessible by default**

### Performance
1. **Use `PartialEq` for props to enable optimization**
2. **Memoize expensive computations**
3. **Lazy load heavy components**
4. **Optimize re-renders with proper key props**

### Maintainability
1. **Document component APIs**
2. **Use TypeScript-like prop patterns**
3. **Follow consistent naming conventions**
4. **Write comprehensive tests**
5. **Keep business logic separate from UI logic**

This component architecture provides a solid foundation for building scalable, maintainable, and performant frontend applications with Yew WebAssembly.
