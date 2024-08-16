use crate::{
    errors::TaskError,
    params::{ParamsParser, ParserResult},
    steps::Step,
};

use futures::future::Future;
use std::marker::PhantomData;
use tokio::time::Duration;
use tokio_stream::Stream;

///
/// tasks
///
#[derive(Debug, Clone)]
pub struct Task<Inbox, Outbox, Output, S>
where
    S: Step<Inbox, Outbox, Output> + Clone + Send + Sync + 'static,
    Inbox: Stream + Send + Sync + 'static,
    Outbox: Send + Sync + 'static,
{
    pub inbox: Option<Inbox>,
    pub outbox: Option<Outbox>,
    pub step: S,
    pub timer: Option<Duration>,
    _phantom: PhantomData<Output>,
}

///
/// basic constructor
///
impl<Inbox, Outbox, Output, S> Task<Inbox, Outbox, Output, S>
where
    S: Step<Inbox, Outbox, Output> + Clone + Send + Sync + 'static,
    Inbox: Stream + Send + Sync + 'static,
    Outbox: Send + Sync + 'static,
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
                step,
                timer,
                _phantom: PhantomData,
            }),
        }
    }

    pub async fn start<Fut>(
        self,
        task: impl Fn(Task<Inbox, Outbox, Output, S>) -> Fut,
    ) -> Result<(), TaskError>
    where
        Fut: Future<Output = Result<(), TaskError>> + Send + Sync + 'static,
    {
        task(self).await
    }
}
