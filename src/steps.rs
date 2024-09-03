use super::{errors::TaskError, pipe::Conduit};
use std::{future::Future, pin::Pin, sync::Arc};

type BoxFuture<'a, T> = Pin<Box<dyn Future<Output = T> + Send + 'a>>;

///
/// Task required behavior.
///
#[trait_variant::make(Step: Send)]
pub trait LocalStep<Inbox, Outbox: Conduit, Output>
where
    Output: Send + Sync + 'static,
    Outbox: Sync + Send + 'static,
    Inbox: Sync + Send + 'static,
{
    type Error: Send;

    #[allow(dead_code)]
    async fn run(self: Arc<Box<Self>> , inbox: Option<Inbox>, outbox: Option<Outbox>)
        -> Result<Output, Self::Error>;
}

///
/// Step definition
///
//#[derive(Clone)]
pub struct TaskStep<Inbox, Outbox: Conduit, Output>(
    Arc<
        Box<
            dyn Fn(Option<Inbox>, Option<Outbox>) -> BoxFuture<'static, Result<Output, TaskError>>
                + Sync
                + Send,
        >,
    >,
)
where
    Inbox: Send + Sync + 'static,
    Outbox: Send + Sync + 'static,
    Output: Send + Sync + 'static;

///
/// Step constructor
///
impl<Inbox, Outbox: Conduit, Output> TaskStep<Inbox, Outbox, Output>
where
    Inbox: Sync + Send + 'static,
    Outbox: Sync + Send + 'static,
    Output: Sync + Send + 'static,
{
    pub fn new<Fut>(task: fn(Option<Inbox>, Option<Outbox>) -> Fut) -> Self
    where
        Fut: Future<Output = Result<Output, TaskError>> + Send + 'static,
    {
        Self(Arc::new(Box::new(move |i, o| Box::pin(task(i, o)))))
    }
}

///
/// Implements the `Step` behavior for `TaskStep`
///
impl<Inbox, Outbox: Conduit, Output> Step<Inbox, Outbox, Output> for TaskStep<Inbox, Outbox, Output>
where
    Inbox: Sync + Send + 'static,
    Outbox: Sync + Send + 'static,
    Output: Sync + Send + 'static,
{
    type Error = TaskError;
    async fn run(
        self: Arc<Box<Self>> ,
        inbox: Option<Inbox>,
        outbox: Option<Outbox>,
    ) -> Result<Output, Self::Error> {
        let fnc = Arc::clone(&self.0);
        let output: Output = (fnc)(inbox, outbox).await?;
        Ok(output)
    }
}
