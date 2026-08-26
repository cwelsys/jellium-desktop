//! Window visibility state. Decisions come from `jfn_tray::policy`; this module
//! owns the flags and performs the window operation.

#![cfg(target_os = "linux")]

use std::sync::atomic::{AtomicBool, Ordering};

use jfn_platform_abi::get as plat;
use jfn_tray::policy::{
    CloseAction, PlaybackAction, VisibilityAction, on_close, on_hide, on_video_mode,
    on_video_resumed,
};

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

/// Maps or unmaps the window, calling into the platform only when `HIDDEN`
/// actually changes so redundant show/hide calls never reach it.
fn set_mapped(visible: bool) {
    let new_hidden = !visible;
    if HIDDEN.swap(new_hidden, Ordering::AcqRel) != new_hidden {
        plat().window_set_visible(visible);
    }
}

/// Clears `raised_by_playback`: a window the user asked for stays up when
/// playback ends.
pub fn show() {
    RAISED_BY_PLAYBACK.store(false, Ordering::Release);
    set_mapped(true);
}

/// Pauses first, so the last frame is not decoded into a surface that is about
/// to be unmapped.
pub fn hide() {
    if on_hide(jfn_playback::visibility_sink::video_playing()) == PlaybackAction::Pause {
        jfn_playback::sink_core::execute(jfn_playback::sink_core::MediaCommand::Pause);
    }
    set_mapped(false);
}

/// Video started or resumed. Leaves `raised_by_playback` alone rather than
/// setting it: the flag already records whether the window was the user's or
/// playback's, and resuming does not change whose it is. Setting it here would
/// make a window the user had open before hiding vanish when playback ends.
pub fn handle_video_resumed() {
    if on_video_resumed(hidden()) == VisibilityAction::Show {
        set_mapped(true);
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
        VisibilityAction::Show => set_mapped(true),
        VisibilityAction::Hide => set_mapped(false),
        VisibilityAction::Nothing => {}
    }
    RAISED_BY_PLAYBACK.store(raised, Ordering::Release);
}
