use discord_rich_presence::{DiscordIpc, DiscordIpcClient};
use serde_json::Value;

pub struct IpcReply;

impl IpcReply {
    pub fn expectReady(client: &mut DiscordIpcClient) -> Result<(), String> {
        let rejected = "Discord закрыл соединение — проверьте Application ID".to_string();
        let reply = Self::read(client).map_err(|_| rejected)?;
        match Self::event(&reply) {
            Some("READY") => Ok(()),
            _ => Err(Self::describe(&reply, "Discord отклонил подключение")),
        }
    }

    pub fn expectSuccess(client: &mut DiscordIpcClient) -> Result<(), String> {
        let reply = Self::read(client)?;
        match Self::event(&reply) {
            Some("ERROR") => Err(Self::describe(&reply, "Discord отклонил статус")),
            _ => Ok(()),
        }
    }

    fn read(client: &mut DiscordIpcClient) -> Result<Value, String> {
        client
            .recv()
            .map(|(_, payload)| payload)
            .map_err(|error| format!("Нет ответа от Discord: {error}"))
    }

    fn event(reply: &Value) -> Option<&str> {
        reply.get("evt").and_then(Value::as_str)
    }

    fn describe(reply: &Value, prefix: &str) -> String {
        let message = reply
            .get("data")
            .and_then(|data| data.get("message"))
            .or_else(|| reply.get("message"))
            .and_then(Value::as_str)
            .unwrap_or("неизвестная ошибка");
        format!("{prefix}: {message}")
    }
}
