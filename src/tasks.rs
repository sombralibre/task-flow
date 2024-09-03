use crate::{
    errors::TaskError,
    params::{ParamsParser, ParserResult},
    pipe::Conduit,
    steps::Step,
};

use std::{future::Future, marker::PhantomData, time::Duration};

///
/// tasks
///
#[derive(Debug, Clone)]
pub struct Task<Inbox, InboxItem, Outbox, Output, S>
where
    S: Step<InboxItem, Outbox, Output> + Send + Sync + 'static,
    InboxItem: Send + Sync + 'static,
    Inbox: Send + Sync + 'static,
    Outbox: Send + Sync + 'static + Conduit,
    Output: Send + Sync + 'static,
{
    pub inbox: Option<Inbox>,
    pub outbox: Option<Outbox>,
    pub step: std::sync::Arc<Box<S>>,
    pub timer: Option<Duration>,
    _phantom: PhantomData<Output>,
    _ignore: PhantomData<InboxItem>,
}

///
/// basic constructor
///
impl<Inbox, InboxItem, Outbox, Output, S> Task<Inbox, InboxItem, Outbox, Output, S>
where
    S: Step<InboxItem, Outbox, Output> + Send + Sync + 'static,
    InboxItem: Send + Sync + 'static,
    Inbox: Send + Sync + 'static,
    Outbox: Send + Sync + 'static + Conduit,
    Output: Send + Sync + 'static,
{
    pub fn new<P>(
        inbox: Option<Inbox>,
        outbox: Option<Outbox>,
        step: S,
        timer: Option<Duration>,
    ) -> Result<Self, TaskError>
    where
        P: ParamsParser,
    {
        match P::parse((&inbox, &outbox, &timer)) {
            ParserResult::Invalid => Err(TaskError::Parse),
            ParserResult::Valid => Ok(Self {
                inbox,
                outbox,
                step: std::sync::Arc::new(Box::new(step)),
                timer,
                _phantom: PhantomData,
                _ignore: PhantomData,
            }),
        }
    }

    pub async fn start<Fut>(
        self: Box<Self>,
        task: impl Fn(Task<Inbox, InboxItem, Outbox, Output, S>) -> Fut,
    ) -> Result<(), TaskError>
    where
        Fut: Future<Output = Result<(), TaskError>> + Send + Sync + 'static,
    {
        task(*self).await
    }
}
