use super::{errors::TaskError, notify::Informer};
use async_trait::async_trait;
use futures::future::{BoxFuture, Future};

///
/// Task required behavior.
///
#[async_trait]
pub(crate) trait Step<Inbox, Outbox, Output> {
    type Error: Send;

    async fn run(self, inbox: Option<Inbox>, outbox: Option<Outbox>) -> Result<Output, Self::Error>
    where
        Output: Send,
        Outbox: Informer<Outbox>;
}

///
/// Step definition
///
pub struct TaskStep<Input, Outbox, Output>(
    Box<dyn Fn(Option<Input>, Option<Outbox>) -> BoxFuture<'static, Result<Output, TaskError>>>,
);

///
/// Step constructor
///
impl<Inbox, Outbox, Output> TaskStep<Inbox, Outbox, Output>
where
    Inbox: 'static,
    Outbox: 'static,
{
    pub fn new<Fut>(task: fn(Option<Inbox>, Option<Outbox>) -> Fut) -> Self
    where
        Fut: Future<Output = Result<Output, TaskError>> + Send + Sync + 'static,
    {
        Self(Box::new(move |i, o| Box::pin(task(i, o))))
    }
}

///
/// Implements the `Step` behavior for `TaskStep`
///
#[async_trait]
impl<Inbox, Outbox, Output> Step<Inbox, Outbox, Output> for TaskStep<Inbox, Outbox, Output>
where
    Inbox: Sync + Send,
    Outbox: Sync + Send,
    Output: Sync + Send,
    Self: Sync + Send,
{
    type Error = TaskError;
    async fn run(
        self,
        inbox: Option<Inbox>,
        outbox: Option<Outbox>,
    ) -> Result<Output, Self::Error> {
        let output: Output = (self.0)(inbox, outbox).await?;
        Ok(output)
    }
}
