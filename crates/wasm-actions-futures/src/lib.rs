use std::cell::RefCell;
use std::future::Future;
use std::pin::Pin;
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use std::sync::mpsc::{self, *};
use std::task::{self, Context, Wake, Waker};

use wasm_bindgen::JsValue;
use wasm_bindgen::prelude::Closure;

/// Invokes queueMicrotask
/// 
/// https://developer.mozilla.org/en-US/docs/Web/API/HTML_DOM_API/Microtask_guide
pub fn queue_microtask<F: FnOnce() -> () + 'static>(f: F) {
    struct NeedsDrop {
        callback: Option<Closure<dyn FnMut() -> ()>>,
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
where T: Sized {
    rx: Receiver<T>,
    state: Arc<Mutex<State>>,
}

struct State {
    waker: Option<Waker>,
}

impl<T: Sized> Future for JoinHandle<T>
where T: Sized {
    type Output = T;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> task::Poll<Self::Output> {
        let rx = &self.rx;
        let mut state = self.state.lock().unwrap();
        match rx.try_recv() {
            Ok(t) => task::Poll::Ready(t),
            Err(_) => {
                state.waker = Some(cx.waker().clone());
                task::Poll::Pending
            }
        }
    }
}

struct Microtask<F, T>
where F: Future<Output =  T> + Send, T: Sized + Send {
    tx: Sender<T>,
    state: Arc<Mutex<State>>,
    // this lock is not exactly needed but Rust requires Mutex 
    // because this Future is behind Arc to implement Wake.
    fut: Mutex<Pin<Box<F>>>,
}

impl<F: Future<Output = T> + Send + 'static, T: Sized + Send + 'static> Microtask<F, T> {
    fn new(fut: F) -> (Arc<Self>, JoinHandle<T>) {
        let (tx, rx) = mpsc::channel();
        let state = Arc::new(Mutex::new(State { waker: None }));
        let task = Arc::new(Self {
            tx,
            fut: Mutex::new(Box::pin(fut)),
            state: state.clone()
        });
        let joiner = JoinHandle::<_> {
            rx,
            state    
        };
        (task, joiner)
    }

    fn poll(self: Arc<Self>) {
        let this = self.clone();
        let p = {
            let mut fut = this.fut.lock().unwrap();
            let waker = self.into();
            let mut cx = Context::from_waker(&waker);           
            fut.as_mut().poll(&mut cx)
        };

        if let task::Poll::Ready(outcome) = p {
            let _ = this.tx.send(outcome);
            let mut st= this.state.lock().unwrap();
            if let Some(waker) = st.waker.take() {
                waker.wake();
            }
        }
    }

    fn schedule(self: Arc<Self>) {
        queue_microtask(move || self.poll());
    }
}


impl<F: Future<Output = T> + Send + 'static, T: Sized + Send + 'static> Wake for Microtask<F, T> {
    fn wake(self: Arc<Self>) {
        self.schedule();
    }
}

/// Converts untyped Promise into a typed Future that awaits Result<T, E>
/// 
/// # Example
/// 
/// ```
/// # use wasm_bindgen::JsValue;
/// # use js_sys::Promise;
/// # #[wasm_bindgen_test::wasm_bindgen_test]
/// # async fn test() {
/// let promise = JsValue::from_str("resolved!");
/// let promise = Promise::resolve(&promise);
/// 
/// let fut = wasm_actions_futures::from_promise(promise, move |v| v.as_string().ok_or("failed"), move |_| Err("failed (rejected)"));
/// assert_eq!(fut.await, Ok(String::from("resolved!")));
/// # }
/// ```
pub fn from_promise<
    F: FnOnce(JsValue) -> Result<T, E> + 'static,
    R: FnOnce(JsValue) -> Result<T, E> + 'static,
    T: Sized + 'static, E: Sized + 'static> (promise: js_sys::Promise, resolve: F, reject: R) -> JoinHandle<Result<T, E>> {
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
    JoinHandle { rx, state }
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
pub fn spawn_microtask<F: Future<Output = T> + Send + 'static, T: Sized + Send + 'static>(fut: F) -> JoinHandle<T> {
    let (task, joiner) = Microtask::new(fut);
    task.schedule();
    joiner
}

#[cfg(test)]
mod tests {
    use wasm_bindgen_test::wasm_bindgen_test;

    use crate::spawn_microtask;

    #[wasm_bindgen_test]
    async fn test_spawn_microtask() {
        let handle = spawn_microtask(async move { 42 });
        assert_eq!(handle.await, 42);
    }   
}
