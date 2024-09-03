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
pub struct Task<I, T, J, K, S>
where
    S: Step<T, J, K> + Send + Sync + 'static,
    T: Send + Sync + 'static,
    I: Send + Sync + 'static,
    J: Send + Sync + 'static + Conduit,
    K: Send + Sync + 'static,
{
    pub inbox: Option<I>,
    pub outbox: Option<J>,
    pub step: std::sync::Arc<Box<S>>,
    pub timer: Option<Duration>,
    _phantom: PhantomData<K>,
    _ignore: PhantomData<T>,
}

///
/// basic constructor
///
impl<I, T, J, K, S> Task<I, T, J, K, S>
where
    S: Step<T, J, K> + Send + Sync + 'static,
    T: Send + Sync + 'static,
    I: Send + Sync + 'static,
    J: Send + Sync + 'static + Conduit,
    K: Send + Sync + 'static,
{
    pub fn new<P>(
        inbox: Option<I>,
        outbox: Option<J>,
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
        task: impl Fn(Task <I, T, J, K, S>) -> Fut + Send,
    ) -> Result<(), TaskError>
    where
        Fut: Future<Output = Result<(), TaskError>> + Send + Sync + 'static,
    {
        task(*self).await
    }
}
