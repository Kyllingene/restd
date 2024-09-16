//! The module for stable coroutines.

use core::future::Future;
use core::pin::Pin;
use core::task;

use crate::Never;

/// The result of [`Coroutine::poll`].
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Poll<Y, R> {
    /// The coroutine will continue running, but has yielded this value.
    Yield(Y),
    /// The coroutine has finished, returning this value.
    Return(R),
}

/// A type that can be repeatedly polled until completion, yielding values each
/// time, and returning a value at the end.
pub trait Coroutine<A = ()> {
    /// The type yielded on each poll.
    ///
    /// For iterators, this is `Option<Item>`; for futures, this is `()`.
    type Yield;

    /// The type returned when the coroutine is finished.
    ///
    /// For iterators, this is `!`, since an iterator continues to yield `None`.
    /// For futures, this is `Output`.
    type Return;

    // TODO: enforce this more rigorously
    /// Advance the coroutine one "step".
    ///
    /// If this returns [`Poll::Return`], it is invalid to continue polling.
    /// Since this is a safe fn, "invalid" cannot mean UB; most likely it will
    /// merely panic.
    fn poll(self: Pin<&mut Self>, arg: A) -> Poll<Self::Yield, Self::Return>;
}

impl<I> Coroutine for I
where
    I: Iterator + Unpin,
{
    type Yield = Option<I::Item>;
    type Return = Never;

    fn poll(self: Pin<&mut Self>, (): ()) -> Poll<Self::Yield, Self::Return> {
        Poll::Yield(self.get_mut().next())
    }
}

impl<'a, 'b, F> Coroutine<&'a mut task::Context<'b>> for F
where
    F: Future,
{
    type Yield = ();
    type Return = F::Output;

    fn poll(
        self: Pin<&mut Self>,
        ctx: &'a mut task::Context<'b>,
    ) -> Poll<Self::Yield, Self::Return> {
        match Future::poll(self, ctx) {
            task::Poll::Pending => Poll::Yield(()),
            task::Poll::Ready(v) => Poll::Return(v),
        }
    }
}
