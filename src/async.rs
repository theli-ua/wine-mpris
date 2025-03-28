use std::{future::Future, sync::OnceLock};

use tokio::{runtime::Handle, task::JoinHandle};

static RT: OnceLock<Handle> = OnceLock::new();

pub fn get_runtime() -> &'static Handle {
    RT.get_or_init(|| {
        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_io()
            .build()
            .unwrap();
        let handle = rt.handle().clone();
        std::thread::spawn(move || rt.block_on(futures::future::pending::<()>()));
        handle
    })
}

#[track_caller]
pub fn spawn<F>(future: F) -> JoinHandle<F::Output>
where
    F: Future + Send + 'static,
    F::Output: Send + 'static,
{
    get_runtime().spawn(future)
}
