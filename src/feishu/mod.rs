use std::time::Duration;

use base64::{engine::general_purpose, Engine as _};
use chrono::Local;
use hmac::{Hmac, Mac};

use log::info;
use serde::{Deserialize, Serialize};

pub struct FeiShu {
    webhook: String,
    secret: Option<String>,
    client: reqwest::Client,
}

impl FeiShu {
    pub fn new(webhook: String, secret: Option<String>) -> Self {
        let client = reqwest::ClientBuilder::new()
            .timeout(Duration::from_secs(3))
            .build()
            .unwrap();
        Self {
            webhook,
            secret,
            client,
        }
    }

    pub async fn send_text(&self, text: String) -> anyhow::Result<()> {
        info!("send text: {}", text);
        let message = TextMessage::new(text, self.secret.clone());
        self.client
            .post(&self.webhook)
            .header("Content-Type", "application/json")
            .body(message.to_string())
            .send()
            .await
            .map(|_| ())
            .map_err(|e| anyhow::anyhow!("{}", e))
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TextMessage {
    pub timestamp: Option<String>,
    pub sign: Option<String>,
    #[serde(rename = "msg_type")]
    pub msg_type: String,
    pub content: Content,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Content {
    pub text: String,
}

impl TextMessage {
    pub fn new(text: String, secret: Option<String>) -> Self {
        let mut timestamp = None;
        let mut sign = None;

        if let Some(secret) = secret {
            let t = Local::now().timestamp().to_string();
            let str_to_sign = format!("{t}\n{secret}");
            type HmacSha256 = Hmac<sha2::Sha256>;
            let mut mac = HmacSha256::new_from_slice(str_to_sign.as_bytes()).unwrap();
            mac.update(b"");
            let result = mac.finalize().into_bytes();
            sign = Some(general_purpose::STANDARD.encode(result));
            timestamp = Some(t);
        }

        Self {
            timestamp,
            sign,
            msg_type: "text".to_string(),
            content: Content { text },
        }
    }
}

impl ToString for TextMessage {
    fn to_string(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}
