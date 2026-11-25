use crate::config::{ActivityKind, StatusField};

#[derive(Clone, Debug, PartialEq)]
pub struct PresencePayload {
    pub name: String,
    pub details: String,
    pub state: String,
    pub largeImage: String,
    pub largeText: String,
    pub smallImage: String,
    pub smallText: String,
    pub startSeconds: Option<i64>,
    pub buttons: Vec<(String, String)>,
    pub activityKind: ActivityKind,
    pub statusField: StatusField,
}
