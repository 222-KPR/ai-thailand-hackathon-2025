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
    let _i18n = use_context::<I18nContext>().expect("I18nContext not found");

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

fn format_time(timestamp: &DateTime<Utc>) -> String {
    let local_time = timestamp.format("%H:%M");
    format!("{}", local_time)
}
