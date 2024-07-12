use core::future::Future;
use core::pin::Pin;
use core::task;

use crate::Never;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Poll<Y, R> {
    Yield(Y),
    Return(R),
}

pub trait Coroutine<A = ()> {
    type Yield;
    type Return;

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
