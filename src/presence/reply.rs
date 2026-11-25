use discord_rich_presence::{DiscordIpc, DiscordIpcClient};
use serde_json::Value;

pub struct ReplyFailure {
    pub message: String,
    pub connectionLost: bool,
}

pub struct IpcReply;

impl IpcReply {
    pub fn expectSuccess(client: &mut DiscordIpcClient) -> Result<(), ReplyFailure> {
        let reply = client.recv().map(|(_, payload)| payload).map_err(|_| ReplyFailure {
            message: "Discord закрыл соединение — проверьте Application ID".to_string(),
            connectionLost: true,
        })?;
        match reply.get("evt").and_then(Value::as_str) {
            Some("ERROR") => Err(ReplyFailure {
                message: Self::describe(&reply),
                connectionLost: false,
            }),
            _ => Ok(()),
        }
    }

    fn describe(reply: &Value) -> String {
        let message = reply
            .get("data")
            .and_then(|data| data.get("message"))
            .and_then(Value::as_str)
            .unwrap_or("неизвестная ошибка");
        format!("Discord отклонил статус: {message}")
    }
}
