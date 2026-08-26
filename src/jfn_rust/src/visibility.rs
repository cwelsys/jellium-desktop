//! Window visibility state. Decisions come from `jfn_tray::policy`; this module
//! owns the flags and performs the window operation.

#![cfg(target_os = "linux")]

use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::time::Duration;

use jfn_platform_abi::get as plat;
use jfn_tray::policy::{
    CloseAction, PlaybackAction, VisibilityAction, on_close, on_hide, on_video_mode,
    on_video_resumed,
};

static HIDDEN: AtomicBool = AtomicBool::new(false);
static RAISED_BY_PLAYBACK: AtomicBool = AtomicBool::new(false);
static TRAY_AVAILABLE: AtomicBool = AtomicBool::new(false);

/// Playback reports its end per file, so a queue advancing to the next episode
/// arrives here identically to a queue running out: a video-mode false edge,
/// with the next file's true edge following a moment later. Hiding on the spot
/// makes the window flap at every episode boundary. Waiting costs a few
/// seconds of empty window at a real end and nothing the rest of the time.
///
/// `can_go_next` would answer this precisely, but it rides only on
/// `QueueCapsChanged` and reads false on every `Finished` event, so it would
/// have to be cached and trusted; a delay has the better failure mode.
const HIDE_GRACE: Duration = Duration::from_secs(5);

/// Bumped by every show and by each newly armed hide. A deferred hide fires
/// only while it still holds the current value, so anything that puts the
/// window up in the meantime cancels it.
static HIDE_GEN: AtomicU64 = AtomicU64::new(0);

fn cancel_pending_hide() {
    HIDE_GEN.fetch_add(1, Ordering::AcqRel);
}

/// `set_visible` posts a command to the Wayland event loop rather than
/// touching the surface, so firing from this thread is safe.
fn arm_hide() {
    let generation = HIDE_GEN.fetch_add(1, Ordering::AcqRel) + 1;
    std::thread::spawn(move || {
        std::thread::sleep(HIDE_GRACE);
        if HIDE_GEN.load(Ordering::Acquire) == generation {
            RAISED_BY_PLAYBACK.store(false, Ordering::Release);
            set_mapped(false);
        }
    });
}

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
    cancel_pending_hide();
    RAISED_BY_PLAYBACK.store(false, Ordering::Release);
    set_mapped(true);
}

/// Pauses first, so the last frame is not decoded into a surface that is about
/// to be unmapped.
pub fn hide() {
    cancel_pending_hide();
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
        cancel_pending_hide();
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
        VisibilityAction::Show => {
            cancel_pending_hide();
            set_mapped(true);
            RAISED_BY_PLAYBACK.store(raised, Ordering::Release);
        }
        // The flag stays set until the deferred hide actually fires: if the
        // next episode arrives first the window was never playback's to give
        // back, and the eventual real end still needs to find it true.
        VisibilityAction::Hide => arm_hide(),
        VisibilityAction::Nothing => RAISED_BY_PLAYBACK.store(raised, Ordering::Release),
    }
}
