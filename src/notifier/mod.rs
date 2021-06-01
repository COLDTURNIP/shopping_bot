use anyhow::Result;
use async_trait::async_trait;

pub mod telegram;

pub type NotifyResult = Result<()>;

#[async_trait]
pub trait Notifier {
    async fn notify<T>(&self, message: T) -> NotifyResult where T: Into<String> + Send;
}