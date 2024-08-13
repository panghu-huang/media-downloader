use parking_lot::Mutex;
use std::{
  sync::Arc,
  task::{Context, Poll, Waker},
};

pub type Formatter<T, R> = fn(T) -> R;

struct State<T, R> {
  items: Vec<T>,
  formatter: Formatter<T, R>,
  is_completed: bool,
  wakers: Vec<Waker>,
  receiver_count: usize,
}

#[derive(Clone)]
pub struct Stream<T, R> {
  state: Arc<Mutex<State<T, R>>>,
}

pub struct Receiver<T, R> {
  state: Arc<Mutex<State<T, R>>>,
  index: usize,
}

impl<T, R> Stream<T, R> {
  pub fn new(formatter: Formatter<T, R>) -> Self {
    Self {
      state: Arc::new(Mutex::new(State {
        items: Vec::new(),
        formatter,
        is_completed: false,
        wakers: Vec::new(),
        receiver_count: 0,
      })),
    }
  }

  pub fn recv(&self) -> Receiver<T, R> {
    self.state.lock().receiver_count += 1;

    Receiver {
      state: self.state.clone(),
      index: 0,
    }
  }

  pub fn send(&self, item: T) {
    let mut state = self.state.lock();

    state.items.push(item);

    for waker in state.wakers.drain(..) {
      waker.wake();
    }
  }

  pub fn end(&self) {
    let mut state = self.state.lock();

    state.is_completed = true;

    for waker in state.wakers.drain(..) {
      waker.wake();
    }
  }

  pub fn receiver_count(&self) -> usize {
    self.state.lock().receiver_count
  }
}

impl<T: Clone, R> tokio_stream::Stream for Receiver<T, R> {
  type Item = R;

  fn poll_next(
    mut self: std::pin::Pin<&mut Self>,
    cx: &mut Context<'_>,
  ) -> Poll<Option<Self::Item>> {
    let mut state = self.state.lock();

    state.wakers.push(cx.waker().clone());

    if self.index < state.items.len() {
      let item = state.items.get(self.index).cloned().unwrap();

      let formatted = (state.formatter)(item);

      drop(state);

      self.index += 1;

      // Wake up the next receiver.
      cx.waker().wake_by_ref();

      return Poll::Ready(Some(formatted));
    }

    if state.is_completed {
      return Poll::Ready(None);
    }

    Poll::Pending
  }
}

impl<T, R> Drop for Receiver<T, R> {
  fn drop(&mut self) {
    let mut state = self.state.lock();

    state.receiver_count -= 1;

    if state.receiver_count == 0 {
      state.items.clear();
      state.is_completed = false;
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use std::future::Future;
  use tokio_stream::StreamExt;

  #[tokio::test]
  async fn test_stream() {
    let stream = Stream::new(|x| x * 2);

    let mut receiver = stream.recv();

    stream.send(1);
    stream.send(2);
    stream.send(3);

    stream.end();

    assert_eq!(stream.receiver_count(), 1);

    assert_eq!(receiver.next().await, Some(2));
    assert_eq!(receiver.next().await, Some(4));
    assert_eq!(receiver.next().await, Some(6));
    assert_eq!(receiver.next().await, None);

    let mut receiver1 = stream.recv();

    assert_eq!(receiver1.next().await, Some(2));
    assert_eq!(receiver1.next().await, Some(4));
    assert_eq!(receiver1.next().await, Some(6));
    assert_eq!(receiver1.next().await, None);

    assert_eq!(stream.receiver_count(), 2);

    drop(receiver);
    drop(receiver1);

    assert_eq!(stream.receiver_count(), 0);
  }

  #[tokio::test]
  async fn test_pending() {
    std::future::poll_fn(|ctx| {
      let stream = Stream::new(|x| x * 2);

      stream.send(1);

      let mut receiver = stream.recv();

      let mut next = || {
        let next = receiver.next();

        let mut next = Box::pin(next);
        next.as_mut().poll(ctx)
      };

      assert_eq!(next(), Poll::Ready(Some(2)));
      assert_eq!(next(), Poll::Pending);

      stream.end();

      assert_eq!(next(), Poll::Ready(None));

      Poll::Ready(())
    })
    .await;
  }
}
