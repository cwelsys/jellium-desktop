//! Window visibility state. Decisions come from `jfn_tray::policy`; this module
//! owns the flags and performs the window operation.

#![cfg(target_os = "linux")]

use std::sync::atomic::{AtomicBool, Ordering};

use jfn_platform_abi::get as plat;
use jfn_tray::policy::{CloseAction, VisibilityAction, on_close, on_video_mode};

static HIDDEN: AtomicBool = AtomicBool::new(false);
static RAISED_BY_PLAYBACK: AtomicBool = AtomicBool::new(false);
static TRAY_AVAILABLE: AtomicBool = AtomicBool::new(false);

pub fn hidden() -> bool {
    HIDDEN.load(Ordering::Acquire)
}

pub fn set_tray_available(available: bool) {
    TRAY_AVAILABLE.store(available, Ordering::Release);
}

pub fn tray_available() -> bool {
    TRAY_AVAILABLE.load(Ordering::Acquire)
}

/// Clears `raised_by_playback`: a window the user asked for stays up when
/// playback ends.
pub fn show() {
    RAISED_BY_PLAYBACK.store(false, Ordering::Release);
    if HIDDEN.swap(false, Ordering::AcqRel) {
        plat().window_set_visible(true);
    }
}

pub fn hide() {
    if !HIDDEN.swap(true, Ordering::AcqRel) {
        plat().window_set_visible(false);
    }
}

pub fn toggle() {
    if hidden() { show() } else { hide() }
}

/// True if the close was absorbed by hiding; false falls through to shutdown.
pub fn handle_close() -> bool {
    match on_close(
        TRAY_AVAILABLE.load(Ordering::Acquire),
        jfn_config::close_to_tray(),
    ) {
        CloseAction::Hide => {
            hide();
            true
        }
        CloseAction::Quit => false,
    }
}

pub fn handle_video_mode(active: bool) {
    let (action, raised) =
        on_video_mode(active, hidden(), RAISED_BY_PLAYBACK.load(Ordering::Acquire));
    match action {
        VisibilityAction::Show => {
            HIDDEN.store(false, Ordering::Release);
            plat().window_set_visible(true);
        }
        VisibilityAction::Hide => {
            HIDDEN.store(true, Ordering::Release);
            plat().window_set_visible(false);
        }
        VisibilityAction::Nothing => {}
    }
    RAISED_BY_PLAYBACK.store(raised, Ordering::Release);
}
