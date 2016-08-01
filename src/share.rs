use std::marker::PhantomData;
use std::sync::{Arc, Mutex};
use mould::prelude::*;

pub type Shared<T> = Arc<Mutex<T>>;

// TODO ShareService (share mutex directly)

/// Share data by clone
pub struct CloneService<T: Clone> {
    shared: Shared<T>,
}

impl<T, S: Clone + 'static> Service<T> for CloneService<S> {
    fn route(&self, request: &Request) -> Box<Worker<T>> {
        if request.action == "get-shared" {
            Box::new(GetSharedWorker::new(self.shared.clone()))
        } else if request.action == "set-shared" {
            Box::new(SetSharedWorker::new(self.shared.clone()))
        } else {
            let msg = format!("Unknown action '{}' for share service!", request.action);
            Box::new(RejectWorker::new(msg))
        }
    }
}

struct GetSharedWorker<T> {
    shared: Shared<T>,
}

impl<T> GetSharedWorker<T> {
    fn new(shared: Shared<T>) -> Self {
        GetSharedWorker {
            shared: shared,
        }
    }
}

impl<T, S> Worker<T> for GetSharedWorker<S> {
    fn prepare(&mut self, session: &mut T, mut request: Request) -> worker::Result<Shortcut> {
        Ok(Shortcut::Done)
    }
}

struct SetSharedWorker<T> {
    shared: Shared<T>,
}

impl<T> SetSharedWorker<T> {
    fn new(shared: Shared<T>) -> Self {
        SetSharedWorker {
            shared: shared,
        }
    }
}

impl<T, S> Worker<T> for SetSharedWorker<S> {
    fn prepare(&mut self, session: &mut T, mut request: Request) -> worker::Result<Shortcut> {
        Ok(Shortcut::Done)
    }
}
