use std::fmt::{Debug, Display};

use actix_web::{http::StatusCode, web, HttpRequest, Responder};

use serde::{Deserialize, Serialize};

use crate::{error::IntoHttpError, feishu};

pub async fn version(_req: HttpRequest) -> impl Responder {
    format!("{}/{}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"))
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Alert {
    // we may deserialize only necessary fields what we needs
    // #[serde(rename = "SessionID")]
    // pub session_id: String,
    // #[serde(rename = "Region")]
    // pub region: String,
    // #[serde(rename = "ResourceType")]
    // pub resource_type: String,
    // #[serde(rename = "ResourceId")]
    // pub resource_id: String,
    // #[serde(rename = "MetricName")]
    // pub metric_name: String,
    #[serde(rename = "AlarmTime")]
    pub alarm_time: Option<i64>,
    #[serde(rename = "RecoveryTime")]
    pub recovery_time: Option<i64>,
    #[serde(rename = "Content", alias = "content")]
    pub content: String,
}

impl Display for Alert {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut title = "⚠️";
        if let Some(recovery_time) = self.recovery_time {
            if recovery_time > 0 {
                title = "✅";
            }
        }

        write!(f, "{title} {}", self.content)
    }
}

pub async fn notify(
    feishu: web::Data<feishu::FeiShu>,
    alert: web::Json<Alert>,
) -> actix_web::Result<impl Responder> {
    feishu
        .send_text(format!("{alert}"))
        .await
        .http_error("send feishu failed", StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok("ok")
}
