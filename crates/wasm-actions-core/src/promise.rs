use std::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};

use wasm_bindgen::JsValue;
use wasm_bindgen_futures::JsFuture;

use crate::error::Error;

pub struct Promise<T, F: Fn(JsValue) -> Result<T, Error>> {
    fut: Pin<Box<JsFuture>>,
    f: Box<F>,
}

impl<T, F> Promise<T, F>
where
    F: Fn(JsValue) -> Result<T, Error>,
{
    pub fn new(promise: js_sys::Promise, f: F) -> Self {
        let fut = JsFuture::from(promise);
        let fut = Box::new(fut);

        Self {
            fut: Pin::new(fut),
            f: Box::new(f),
        }
    }
}

impl<T, F: Fn(JsValue) -> Result<T, Error>> Future for Promise<T, F> {
    type Output = Result<T, Error>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let fut = self.fut.as_mut();
        match fut.poll(cx) {
            Poll::Ready(Ok(js)) => Poll::Ready((self.f)(js)),
            Poll::Ready(Err(js)) => Poll::Ready(Err(Error::from(js))),
            Poll::Pending => Poll::Pending,
        }
    }
}
