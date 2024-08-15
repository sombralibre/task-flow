use async_trait::async_trait;
use tokio::sync::mpsc::{error::SendError, Sender, UnboundedSender};

///
/// Simple sender interface
///
#[async_trait]
pub(crate) trait Informer<T> {
    async fn send(&self, value: T) -> Result<(), SendError<T>>
    where
        T: Sync + Send;
}

///
/// Implements the `TaskSender` interface for `Sender<T>`
///
#[async_trait]
impl<T> Informer<T> for Sender<T>
where
    T: Sync + Send,
{
    async fn send(&self, value: T) -> Result<(), SendError<T>> {
        self.send(value).await
    }
}

///
/// Implements the `TaskSender` interface for `UnboundedSender<T>`
///
#[async_trait]
impl<T> Informer<T> for UnboundedSender<T>
where
    T: Sync + Send,
{
    async fn send(&self, value: T) -> Result<(), SendError<T>> {
        self.send(value)
    }
}
