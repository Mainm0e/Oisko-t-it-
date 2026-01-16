use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type", content = "data")]
pub enum AppEvent {
    CommentCreated {
        id: Uuid,
        application_id: Uuid,
        visitor_name: String,
        company: String,
        role: String,
    },
    ApplicationStatusUpdated {
        id: Uuid,
        company: String,
        status: String,
    },
}
