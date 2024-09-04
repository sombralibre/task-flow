
///
/// Simple sender interface
///
/// Example:
///
/// ```
///use tokio::sync::mpsc::UnboundedSender;
///
/// #[derive(Clone)]
///struct SenderWrapper<Chan>(Chan);
///
///impl<Chan> SenderWrapper<Chan> {
///    fn new(chan: Chan) -> Self {
///        Self(chan)
///    }
///}
///
///impl Conduit for SenderWrapper<UnboundedSender<usize>> {
///    type Item = usize;
///    type Error = SendError<Self::Item>;
///    type Output = ();
///
///    async fn try_send(&self, msg: Self::Item) -> Result<Self::Output, Self::Error> {
///        self.0.send(msg)
///    }
///}
/// ```
#[trait_variant::make(Conduit: Send)]
pub trait LocalConduit {
    type Error;
    type Output;
    type Item;

    #[allow(dead_code)]
    async fn try_send(&self, msg: Self::Item) -> Result<Self::Output, Self::Error>;
}
