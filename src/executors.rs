use crate::AgnostikExecutor;
use crate::join_handle::{InnerJoinHandle, JoinHandle};
use std::future::Future;
use std::cell::RefCell;

#[cfg(feature = "runtime_asyncstd")]
pub(crate) struct AsyncStdExecutor;

#[cfg(feature = "runtime_asyncstd")]
impl AsyncStdExecutor {
    pub fn new() -> Self {
        AsyncStdExecutor {}
    }
}

#[cfg(feature = "runtime_asyncstd")]
impl AgnostikExecutor for AsyncStdExecutor {
    fn spawn<F, T>(&self, future: F) -> JoinHandle<T>
    where
        F: Future<Output = T> + Send + 'static,
        T: Send + 'static,
    {
        let handle = async_std::task::spawn(future);
        JoinHandle(InnerJoinHandle::AsyncStd(handle))
    }

    fn spawn_blocking<F, T>(&self, task: F) -> JoinHandle<T>
    where
        F: FnOnce() -> T + Send + 'static,
        T: Send + 'static,
    {
        let handle = async_std::task::spawn_blocking(task);
        JoinHandle(InnerJoinHandle::AsyncStd(handle))
    }

    fn block_on<F, T>(&self, future: F) -> T
    where
        F: Future<Output = T> + Send + 'static,
        T: Send + 'static,
    {
        async_std::task::block_on(future)
    }
}

#[cfg(feature = "runtime_tokio")]
pub(crate) struct TokioExecutor(RefCell<tokio::runtime::Runtime>);

#[cfg(feature = "runtime_tokio")]
impl TokioExecutor {
    pub fn new() -> Self {
        Self::with_runtime(tokio::runtime::Runtime::new().expect("failed to create runtime"))
    }

    pub fn with_runtime(runtime: tokio::runtime::Runtime) -> Self {
        TokioExecutor(RefCell::new(runtime))
    }
}

#[cfg(feature = "runtime_tokio")]
impl AgnostikExecutor for TokioExecutor {
    fn spawn<F, T>(&self, future: F) -> JoinHandle<T>
    where
        F: Future<Output = T> + Send + 'static,
        T: Send + 'static,
    {
        let handle = self.0.borrow().spawn(future);
        JoinHandle(InnerJoinHandle::Tokio(handle))
    }

    fn spawn_blocking<F, T>(&self, task: F) -> JoinHandle<T>
    where
        F: FnOnce() -> T + Send + 'static,
        T: Send + 'static,
    {
        let handle = tokio::task::spawn_blocking(task);
        JoinHandle(InnerJoinHandle::Tokio(handle))
    }

    fn block_on<F, T>(&self, future: F) -> T
    where
        F: Future<Output = T> + Send + 'static,
        T: Send + 'static,
    {
        let mut runtime = self.0.borrow_mut();
        runtime.block_on(future)
    }
}

#[cfg(feature = "runtime_bastion")]
pub(crate) struct BastionExecutor;

#[cfg(feature = "runtime_bastion")]
impl BastionExecutor {
    pub fn new() -> Self {
        BastionExecutor {}
    }
}

#[cfg(feature = "runtime_bastion")]
use lightproc::prelude::*;
#[cfg(feature = "runtime_bastion")]
use bastion_executor::prelude::*;


#[cfg(feature = "runtime_bastion")]
impl AgnostikExecutor for BastionExecutor {
    fn spawn<F, T>(&self, future: F) -> JoinHandle<T>
    where
        F: Future<Output = T> + Send + 'static,
        T: Send + 'static,
    {
        let handle = bastion_executor::pool::spawn(future, ProcStack::default());
        JoinHandle(InnerJoinHandle::Bastion(handle))
    }

    fn spawn_blocking<F, T>(&self, task: F) -> JoinHandle<T>
    where
        F: FnOnce() -> T + Send + 'static,
        T: Send + 'static,
    {
        let handle = spawn_blocking(async { task() }, ProcStack::default());
        JoinHandle(InnerJoinHandle::Bastion(handle))
    }

    fn block_on<F, T>(&self, future: F) -> T
    where
        F: Future<Output = T> + Send + 'static,
        T: Send + 'static,
    {
        bastion_executor::run::run(future, ProcStack::default())
    }
}
