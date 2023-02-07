use chrono::Duration;
use dioxus::prelude::*;

use crate::MESSAGE_TIMEOUT;

#[derive(Props)]
pub struct MessageProps<'a> {
    pub display_message: &'a Coroutine<UIMessage>,
}

#[derive(Debug, Clone)]
pub struct UIMessage {
    pub r#type: UIMessageType,
    pub message: String,
    pub timeout: Duration,
}

#[derive(Debug, Clone, Copy)]
pub enum UIMessageType {
    Error,
    Info,
}

impl Default for UIMessage {
    fn default() -> Self {
        Self {
            r#type: UIMessageType::Info,
            message: Default::default(),
            timeout: Duration::milliseconds(*MESSAGE_TIMEOUT),
        }
    }
}

impl UIMessage {
    pub fn info(message: String) -> Self {
        Self {
            r#type: UIMessageType::Info,
            message,
            ..Default::default()
        }
    }

    pub fn error(message: String) -> Self {
        Self {
            r#type: UIMessageType::Error,
            message,
            ..Default::default()
        }
    }

    pub fn server_error() -> Self {
        UIMessage::error("Connection to server failed".to_string())
    }
}
