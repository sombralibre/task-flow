use super::{errors::TaskError, pipe::Conduit};
use std::{future::Future, pin::Pin, sync::Arc};

type BoxFuture<'a, T> = Pin<Box<dyn Future<Output = T> + Send + 'a>>;
type StepFn<I, O, T> = Arc<
    Box<dyn Fn(Option<I>, Option<O>) -> BoxFuture<'static, Result<T, TaskError>> + Sync + Send>,
>;

///
/// Task required behavior.
///
#[trait_variant::make(Step: Send)]
pub trait LocalStep<I, J: Conduit, K>
where
    K: Send + Sync + 'static,
    I: Sync + Send + 'static,
{
    type Error: Send;

    #[allow(dead_code)]
    async fn run(
        self: Arc<Box<Self>>,
        inbox: Option<I>,
        outbox: Option<J>,
    ) -> Result<K, Self::Error>;
}

///
/// Step definition
///
//#[derive(Clone)]
pub struct TaskStep<I, J: Conduit, K>(StepFn<I, J, K>)
where
    I: Send + Sync + 'static,
    K: Send + Sync + 'static;

///
/// Step constructor
///
impl<I, J: Conduit, K> TaskStep<I, J, K>
where
    I: Sync + Send + 'static,
    J: Sync + Send + 'static,
    K: Sync + Send + 'static,
{
    pub fn new<Fut>(task: fn(Option<I>, Option<J>) -> Fut) -> Self
    where
        Fut: Future<Output = Result<K, TaskError>> + Send + 'static,
    {
        Self(Arc::new(Box::new(move |i, o| Box::pin(task(i, o)))))
    }
}

///
/// Implements the `Step` behavior for `TaskStep`
///
impl<I, J: Conduit, K> Step<I, J, K> for TaskStep<I, J, K>
where
    I: Sync + Send + 'static,
    K: Sync + Send + 'static,
{
    type Error = TaskError;
    async fn run(
        self: Arc<Box<Self>>,
        inbox: Option<I>,
        outbox: Option<J>,
    ) -> Result<K, Self::Error> {
        (Arc::clone(&self.0))(inbox, outbox).await
    }
}
