use std::sync::atomic::{AtomicPtr, Ordering};

use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "message")]
pub enum Request {
    Ping,
    /// A second launch asking the running instance to bring its window back.
    Show,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "message")]
pub enum Response {
    Pong,
    Shown,
}

static SHOW: AtomicPtr<()> = AtomicPtr::new(std::ptr::null_mut());

/// Runs on the IPC task's thread: signal or post only, never block.
pub fn set_show_handler(handler: Option<fn()>) {
    let ptr = handler
        .map(|f| f as *mut ())
        .unwrap_or(std::ptr::null_mut());
    SHOW.store(ptr, Ordering::Release);
}

fn show_handler() -> Option<fn()> {
    let ptr = SHOW.load(Ordering::Acquire);
    if ptr.is_null() {
        return None;
    }
    // SAFETY: the pointer is only ever set from a `fn()` in
    // `set_show_handler`, and fn pointers have static lifetime.
    let f: fn() = unsafe { std::mem::transmute(ptr) };
    Some(f)
}

pub fn handle(req: &Request) -> Response {
    match req {
        Request::Ping => Response::Pong,
        Request::Show => {
            if let Some(f) = show_handler() {
                f();
            }
            Response::Shown
        }
    }
}
