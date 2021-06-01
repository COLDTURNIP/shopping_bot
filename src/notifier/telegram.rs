use teloxide::Bot as TelBot;
use teloxide::prelude::*;
use super::*;

pub struct Bot {
    client: TelBot,
    channel_name: String,
}

impl Bot {
    pub fn new<TS, CS>(token: TS, channel: CS) -> Bot
        where TS: Into<String>, CS: Into<String> {
        Bot { client: teloxide::Bot::new(token), channel_name: channel.into() }
    }
}

#[async_trait]
impl Notifier for Bot {
    async fn notify<T>(&self, message: T) -> NotifyResult where T: Into<String> + Send {
        let msg = self.client.send_message(self.channel_name.clone(), message);
        msg.send().await?;
        Ok(())
    }
}
