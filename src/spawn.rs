use poll_promise::Promise;
use std::future::Future;

#[cfg(not(target_arch = "wasm32"))]
pub fn spawn<T: Send>(future: impl Future<Output = T> + Send + 'static) -> Promise<T> {
    Promise::spawn_async(future)
}

#[cfg(target_arch = "wasm32")]
pub fn spawn<T: Send>(future: impl Future<Output = T> + 'static) -> Promise<T> {
    Promise::spawn_local(future)
}
