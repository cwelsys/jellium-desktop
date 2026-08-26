//! System-tray presence (StatusNotifierItem) and the window-visibility
//! policy it drives.

#![cfg(target_os = "linux")]

mod item;
pub mod policy;

use parking_lot::Mutex;

/// Function pointers rather than a trait so this crate takes no dependency on
/// the binary crate.
#[derive(Clone, Copy)]
pub struct Callbacks {
    pub toggle: fn(),
    pub show: fn(),
    pub quit: fn(),
    pub available: fn(bool),
}

static HANDLE: Mutex<Option<ksni::Handle<item::JelliumTray>>> = Mutex::new(None);

/// No-op if already running. A false through `callbacks.available` means no
/// StatusNotifierWatcher is on the bus and the caller must keep the window
/// reachable.
pub fn start(callbacks: Callbacks) {
    // Held across the spawn round-trip deliberately: check-and-spawn must be atomic.
    let mut slot = HANDLE.lock();
    if slot.is_some() {
        return;
    }
    let tray = item::JelliumTray { callbacks };
    match item::spawn(tray) {
        Ok(handle) => {
            *slot = Some(handle);
            (callbacks.available)(true);
            tracing::info!(target: "Tray", "status notifier item registered");
        }
        Err(e) => {
            (callbacks.available)(false);
            tracing::warn!(target: "Tray", "no system tray: {e}");
        }
    }
}

pub fn stop() {
    let Some(handle) = HANDLE.lock().take() else {
        return;
    };
    async_io::block_on(handle.shutdown());
}
