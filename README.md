# task-flow-poc

A PoC for start multiple green thread based on the item of a iterator, then every of this items will be  process on a series of task stages.

The communication between stages are performed using `tokio::mpsc`.

For easiness defining tasks, I've created this library [task-flow](https://github.com/sombralibre/task-flow).

Implements the `Conduit` trait for a object wrapper for channel or whatever.

``` rust
#[derive(Clone)]
struct SenderWrapper<Chan>(Chan);

impl<Chan> SenderWrapper<Chan> {
    fn new(chan: Chan) -> Self {
        Self(chan)
    }
}

impl Conduit for SenderWrapper<UnboundedSender<usize>> {
    type Item = usize;
    type Error = SendError<Self::Item>;
    type Output = ();

    async fn try_send(&self, msg: Self::Item) -> Result<Self::Output, Self::Error> {
        self.0.send(msg)
    }
}
```

The full code example in [app.rs]("/src/bin/app.rs")
