use super::payload::PresencePayload;
use crate::config::{ActivityKind, StatusField};
use discord_rich_presence::activity::{Activity, ActivityType, Assets, Button, StatusDisplayType, Timestamps};

pub struct ActivityMapper;

impl ActivityMapper {
    pub fn map(payload: &PresencePayload) -> Activity<'_> {
        let mut activity = Activity::new()
            .activity_type(Self::kind(payload.activityKind))
            .status_display_type(Self::field(payload.statusField));
        if !payload.details.is_empty() {
            activity = activity.details(payload.details.as_str());
        }
        if !payload.state.is_empty() {
            activity = activity.state(payload.state.as_str());
        }
        activity = activity.assets(Self::assets(payload));
        if let Some(start) = payload.startSeconds {
            activity = activity.timestamps(Timestamps::new().start(start));
        }
        if !payload.buttons.is_empty() {
            let buttons = payload
                .buttons
                .iter()
                .map(|(label, url)| Button::new(label.as_str(), url.as_str()))
                .collect();
            activity = activity.buttons(buttons);
        }
        activity
    }

    fn assets(payload: &PresencePayload) -> Assets<'_> {
        let mut assets = Assets::new();
        if !payload.largeImage.is_empty() {
            assets = assets.large_image(payload.largeImage.as_str());
            if !payload.largeText.is_empty() {
                assets = assets.large_text(payload.largeText.as_str());
            }
        }
        if !payload.smallImage.is_empty() {
            assets = assets.small_image(payload.smallImage.as_str());
            if !payload.smallText.is_empty() {
                assets = assets.small_text(payload.smallText.as_str());
            }
        }
        assets
    }

    fn kind(kind: ActivityKind) -> ActivityType {
        match kind {
            ActivityKind::Playing => ActivityType::Playing,
            ActivityKind::Listening => ActivityType::Listening,
            ActivityKind::Watching => ActivityType::Watching,
            ActivityKind::Competing => ActivityType::Competing,
        }
    }

    fn field(field: StatusField) -> StatusDisplayType {
        match field {
            StatusField::Name => StatusDisplayType::Name,
            StatusField::State => StatusDisplayType::State,
            StatusField::Details => StatusDisplayType::Details,
        }
    }
}
