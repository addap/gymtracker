use log::info;
use serde::de::DeserializeOwned;
use std::result::Result;

use crate::messages::UIMessage;
use reqwest::Response;

pub trait RequestExt {
    async fn handle_result<T>(self, error: UIMessage) -> Result<T, UIMessage>
    where
        T: DeserializeOwned;
}

impl RequestExt for reqwest::Result<Response> {
    async fn handle_result<T>(self, error: UIMessage) -> Result<T, UIMessage>
    where
        T: DeserializeOwned,
    {
        match self {
            Ok(res) => match res.json::<T>().await {
                Ok(json) => Ok(json),
                Err(e) => {
                    info!("{}", e);
                    Err(error)
                }
            },
            Err(e) => {
                info!("{}", e);
                Err(UIMessage::server_error())
            }
        }
    }
}
