use super::activity::ActivityMapper;
use super::payload::PresencePayload;
use super::reply::IpcReply;
use discord_rich_presence::{DiscordIpc, DiscordIpcClient};

#[derive(Default)]
pub struct DiscordLink {
    client: Option<DiscordIpcClient>,
    clientId: String,
    lastPayload: Option<PresencePayload>,
    pub lastError: Option<String>,
}

impl DiscordLink {
    pub fn isConnected(&self) -> bool {
        self.client.is_some()
    }

    pub fn publish(&mut self, clientId: &str, payload: Option<&PresencePayload>) {
        if clientId != self.clientId {
            self.disconnect();
            self.clientId = clientId.to_string();
        }
        if self.client.is_some() && self.lastPayload.as_ref() == payload {
            return;
        }
        if !self.ensureConnected() {
            return;
        }
        let Some(client) = self.client.as_mut() else {
            return;
        };
        let sent = match payload {
            Some(content) => client.set_activity(ActivityMapper::map(content)),
            None => client.clear_activity(),
        };
        if let Err(error) = sent {
            self.lastError = Some(format!("Связь с Discord потеряна: {error}"));
            self.disconnect();
            return;
        }
        self.lastPayload = payload.cloned();
        self.lastError = IpcReply::expectSuccess(client).err();
    }

    pub fn disconnect(&mut self) {
        if let Some(mut client) = self.client.take() {
            let _ = client.clear_activity();
            let _ = client.close();
        }
        self.lastPayload = None;
    }

    fn ensureConnected(&mut self) -> bool {
        if self.client.is_some() {
            return true;
        }
        if self.clientId.trim().is_empty() {
            self.lastError = Some("Не указан Application ID Discord".to_string());
            return false;
        }
        let mut client = DiscordIpcClient::new(self.clientId.trim());
        if let Err(error) = client.connect() {
            self.lastError = Some(format!("Discord недоступен: {error}"));
            return false;
        }
        if let Err(message) = IpcReply::expectReady(&mut client) {
            self.lastError = Some(message);
            let _ = client.close();
            return false;
        }
        self.client = Some(client);
        self.lastPayload = None;
        true
    }
}
