//! Theme-color sink. Resets ThemeColor video mode on terminal playback
//! events. Active-true setVideoMode fires from the web_browser path
//! on metadata arrival; that's not mpv-derived and stays out of the
//! playback event stream, so it enters through
//! [`jfn_playback_theme_video_mode_changed`] rather than [`deliver`].
//!
//! Both edges must reach the installed handler: it drives more than the theme
//! colour — on Linux the active-true edge is what raises a tray-hidden window
//! for an incoming cast.

use parking_lot::Mutex;
use std::sync::OnceLock;

use crate::types::{PlaybackEvent, PlaybackEventKind};

type SetCb = extern "C" fn(bool);

fn cb_slot() -> &'static Mutex<Option<SetCb>> {
    static SLOT: OnceLock<Mutex<Option<SetCb>>> = OnceLock::new();
    SLOT.get_or_init(|| Mutex::new(None))
}

/// Install the ThemeColor::setVideoMode setter. `cb == None` disables.
pub fn jfn_playback_set_theme_video_mode_handler(cb: Option<SetCb>) {
    *cb_slot().lock() = cb;
}

/// Report a video-mode edge that did not come from mpv. The web path owns the
/// active-true edge, which [`deliver`] never sees.
pub fn jfn_playback_theme_video_mode_changed(active: bool) {
    if let Some(cb) = *cb_slot().lock() {
        cb(active);
    }
}

pub(crate) fn deliver(ev: &PlaybackEvent) {
    match ev.kind {
        PlaybackEventKind::Finished | PlaybackEventKind::Canceled | PlaybackEventKind::Error => {
            if let Some(cb) = *cb_slot().lock() {
                cb(false);
            }
        }
        _ => {}
    }
}
