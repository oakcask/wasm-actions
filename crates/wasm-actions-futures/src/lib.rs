use std::cell::RefCell;
use std::future::Future;
use std::pin::Pin;
use std::rc::Rc;
use std::sync::mpsc::{self, *};
use std::sync::{Arc, Mutex};
use std::task::{self, Context, Waker};

#[doc(hidden)]
pub use js_sys::Promise;
use wasm_bindgen::JsValue;
use wasm_bindgen::prelude::Closure;

/// Invokes queueMicrotask
///
/// <https://developer.mozilla.org/en-US/docs/Web/API/HTML_DOM_API/Microtask_guide>
pub fn queue_microtask<F: FnOnce() + 'static>(f: F) {
    struct NeedsDrop {
        callback: Option<Closure<dyn FnMut()>>,
    }
    let rc = Rc::new(RefCell::new(NeedsDrop { callback: None }));
    let task = rc.clone();
    rc.borrow_mut().callback = Some(Closure::once(move || {
        f();
        // making sure drop callback Closure here.
        drop(task.borrow_mut().callback.take());
    }));
    wasm_actions_node_sys::queue_microtask(rc.borrow().callback.as_ref().unwrap());
}

/// Join handle to await microtask
pub struct JoinHandle<T>
where
    T: Sized,
{
    rx: Option<Receiver<T>>,
    state: Arc<Mutex<State>>,
}

impl<T: Sized + 'static, E: Sized + 'static> JoinHandle<Result<T, E>> {
    /// Converts untyped Promise into a typed Future that awaits Result<T, E>
    ///
    /// # Example
    ///
    /// ```
    /// # use wasm_bindgen::JsValue;
    /// # use js_sys::Promise;
    /// # use wasm_actions_futures::JoinHandle;
    /// # #[wasm_bindgen_test::wasm_bindgen_test]
    /// # async fn test() {
    /// let promise = JsValue::from_str("resolved!");
    /// let promise = Promise::resolve(&promise);
    ///
    /// let fut = JoinHandle::from_promise(promise, move |v| v.as_string().ok_or("failed"), move |_| Err("failed (rejected)"));
    /// assert_eq!(fut.await, Ok(String::from("resolved!")));
    /// # }
    /// ```
    pub fn from_promise<
        ResolveFn: FnOnce(JsValue) -> Result<T, E> + 'static,
        RejectFn: FnOnce(JsValue) -> Result<T, E> + 'static,
    >(
        promise: js_sys::Promise,
        resolve: ResolveFn,
        reject: RejectFn,
    ) -> Self {
        struct NeedsDrop<T, E> {
            tx: Sender<Result<T, E>>,
            state: Arc<Mutex<State>>,
            cb: Option<(Closure<dyn FnMut(JsValue)>, Closure<dyn FnMut(JsValue)>)>,
        }
        let (tx, rx) = mpsc::channel();
        let state = Arc::new(Mutex::new(State { waker: None }));
        let slot = Rc::new(RefCell::new(NeedsDrop {
            tx,
            cb: None,
            state: state.clone(),
        }));
        let resolve = {
            let resolve_slot = slot.clone();
            Closure::once(move |value| {
                let mut s = resolve_slot.borrow_mut();
                let _ = s.tx.send(resolve(value));
                if let Ok(mut st) = s.state.clone().lock() {
                    if let Some(w) = st.waker.take() {
                        w.wake();
                    }
                }
                drop(s.cb.take());
            })
        };
        let reject = {
            let reject_slot = slot.clone();
            Closure::once(move |value| {
                let mut s = reject_slot.borrow_mut();
                let _ = s.tx.send(reject(value));
                if let Ok(mut st) = s.state.clone().lock() {
                    if let Some(w) = st.waker.take() {
                        w.wake();
                    }
                }
                drop(s.cb.take());
            })
        };

        let _ = promise.then2(&resolve, &reject);
        slot.borrow_mut().cb = Some((resolve, reject));
        JoinHandle {
            rx: Some(rx),
            state,
        }
    }
}

impl<T: Into<JsValue> + Sized + 'static, E: Into<JsValue> + Sized + 'static>
    From<JoinHandle<Result<T, E>>> for Promise
{
    /// Converts JoinHandle to Promise
    ///
    /// # Example
    /// ```
    /// # use wasm_bindgen::JsError;
    /// # use wasm_actions_futures::spawn_microtask;
    /// # use wasm_actions_futures::JoinHandle;
    /// # #[wasm_bindgen_test::wasm_bindgen_test]
    /// # async fn test() {
    /// let fut: JoinHandle<Result<i32, JsError>> = spawn_microtask(async move { Ok(42) });
    /// let promise: js_sys::Promise = fut.into();
    /// # let result = wasm_bindgen_futures::JsFuture::from(promise).await;
    /// # assert_eq!(result.map(|js| js.as_f64()), Ok(Some(42.0)));
    /// # }
    /// ```  
    fn from(value: JoinHandle<Result<T, E>>) -> Self {
        wasm_bindgen_futures::future_to_promise(async move {
            match value.await {
                Ok(ok) => Ok(ok.into()),
                Err(err) => Err(err.into()),
            }
        })
    }
}

struct State {
    waker: Option<Waker>,
}

impl<T: Sized> Future for JoinHandle<T>
where
    T: Sized,
{
    type Output = T;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> task::Poll<Self::Output> {
        let this = self.get_mut();
        match this.rx.as_ref().unwrap().try_recv() {
            Ok(t) => {
                this.rx.take(); // drop rx
                task::Poll::Ready(t)
            }
            Err(_) => {
                let mut state = this.state.lock().unwrap();
                state.waker = Some(cx.waker().clone());
                task::Poll::Pending
            }
        }
    }
}

/// Poll Rust future in microtask queue
///
/// # Example
/// ```
/// # use wasm_actions_futures::spawn_microtask;
/// # #[wasm_bindgen_test::wasm_bindgen_test]
/// # async fn test() {
/// let handle = spawn_microtask(async move { 42 });
/// assert_eq!(handle.await, 42);
/// # }
/// ```
pub fn spawn_microtask<F: Future<Output = T> + 'static, T: Sized + 'static>(
    fut: F,
) -> JoinHandle<T> {
    let (tx, rx) = mpsc::channel();
    let state = Arc::new(Mutex::new(State { waker: None }));
    let state_for_microtask = state.clone();
    wasm_bindgen_futures::spawn_local(async move {
        let _ = tx.send(fut.await); // send may fail but it's okay.
        let mut st = state_for_microtask.lock().unwrap();
        if let Some(waker) = st.waker.take() {
            waker.wake();
        }
    });
    JoinHandle {
        rx: Some(rx),
        state,
    }
}

impl<T: From<JsValue> + Sized + 'static, E: From<JsValue> + Sized + 'static>
    Into<JoinHandle<Result<T, E>>> for Promise
{
    /// Converts Promise into JoinHandle
    ///
    /// The result of JoinHanle will be Ok on promise resolves,
    /// and will be Err on promise rejects.
    ///
    /// # Example
    ///
    /// ```
    /// # use wasm_bindgen::JsValue;
    /// # use js_sys::Promise;
    /// # use wasm_actions_futures::JoinHandle;
    /// # #[wasm_bindgen_test::wasm_bindgen_test]
    /// # async fn test() {
    /// let promise = JsValue::from_str("resolved!");
    /// let promise = Promise::resolve(&promise);
    ///
    /// let fut: JoinHandle<Result<JsValue, JsValue>> = promise.into();
    /// assert_eq!(fut.await, Ok(JsValue::from("resolved!")));
    /// # }
    /// ```
    fn into(self) -> JoinHandle<Result<T, E>> {
        JoinHandle::from_promise(self, |r| Ok(r.into()), |e| Err(e.into()))
    }
}
