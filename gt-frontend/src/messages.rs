use dioxus::prelude::*;

#[derive(Props)]
pub struct MessageProps<'a> {
    pub display_message: &'a Coroutine<UIMessage>,
}
pub struct UIMessage {
    pub r#type: UIMessageType,
    pub message: String,
}

pub enum UIMessageType {
    Error,
    Info,
}

impl UIMessage {
    pub fn info(message: String) -> Self {
        Self {
            r#type: UIMessageType::Info,
            message,
        }
    }

    pub fn error(message: String) -> Self {
        Self {
            r#type: UIMessageType::Error,
            message,
        }
    }
}
