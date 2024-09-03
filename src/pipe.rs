
///
/// Simple sender interface
///
// #[async_trait]
// pub trait Conduit<T> {
//     async fn try_send(&self, msg: T) -> Result<(), SendError<T>>
//     where
//         T: Sync + Send + 'static;
// }
#[trait_variant::make(Conduit: Send)]
pub trait LocalConduit {
    type Error;
    type Output;
    type Item;

    #[allow(dead_code)]
    async fn try_send(&self, msg: Self::Item) -> Result<Self::Output, Self::Error>;
}

//
// Implements the `TaskSender` interface for `Sender<T>`
//
// #[async_trait]
// impl<T> Conduit<T> for Sender<T>
// where
//     T: Sync + Send + 'static,
// {
//     async fn try_send(&self, msg: T) -> Result<(), SendError<T>> {
//         self.send(msg).await
//     }
// }

//
// Implements the `TaskSender` interface for `UnboundedSender<T>`
//
// #[async_trait]
// impl<T> Conduit<T> for UnboundedSender<T>
// where
//     T: Sync + Send + 'static,
// {
//     async fn try_send(&self, msg: T) -> Result<(), SendError<T>> {
//         self.send(msg)
//     }
// }
