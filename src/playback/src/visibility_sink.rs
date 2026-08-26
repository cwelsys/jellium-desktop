//! Window-visibility sink. Video and window visibility interact in two
//! directions, and both need mpv's own view of playback rather than a JS
//! round-trip: hiding pauses video (the query here answers whether there is
//! any to pause), and video resuming from anywhere — MPRIS, a media key, the
//! JS UI — raises a window that was hidden.
//!
//! Audio is deliberately excluded from both. Playing on with no window is what
//! the tray exists for.

use parking_lot::Mutex;
use std::sync::OnceLock;
use std::sync::atomic::{AtomicBool, Ordering};

use crate::types::{MediaType, PlaybackEvent, PlaybackEventKind, PlaybackPhase};

static VIDEO_PLAYING: AtomicBool = AtomicBool::new(false);

type ResumedCb = extern "C" fn();

fn cb_slot() -> &'static Mutex<Option<ResumedCb>> {
    static SLOT: OnceLock<Mutex<Option<ResumedCb>>> = OnceLock::new();
    SLOT.get_or_init(|| Mutex::new(None))
}

/// Install the handler fired when video starts or resumes. `cb == None`
/// disables it.
pub fn jfn_playback_set_video_resumed_handler(cb: Option<ResumedCb>) {
    *cb_slot().lock() = cb;
}

/// True when mpv is playing video right now. False for audio and for paused
/// video, so hiding never interrupts music.
pub fn video_playing() -> bool {
    VIDEO_PLAYING.load(Ordering::Acquire)
}

pub(crate) fn deliver(ev: &PlaybackEvent) {
    match ev.kind {
        PlaybackEventKind::Started
        | PlaybackEventKind::Paused
        | PlaybackEventKind::Finished
        | PlaybackEventKind::Canceled
        | PlaybackEventKind::Error
        | PlaybackEventKind::MediaTypeChanged => {
            // Video only. Unknown media type is treated as not-video: raising
            // the window on a guess is worse than not raising it.
            let playing = ev.snapshot.phase == PlaybackPhase::Playing
                && ev.snapshot.media_type == MediaType::Video;
            // Rising edge only: a repeated Playing snapshot must not re-fire,
            // or every position tick past a filter would re-raise the window.
            if !VIDEO_PLAYING.swap(playing, Ordering::AcqRel)
                && playing
                && let Some(cb) = *cb_slot().lock()
            {
                cb();
            }
        }
        _ => {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn event(kind: PlaybackEventKind, phase: PlaybackPhase, media: MediaType) -> PlaybackEvent {
        let mut ev = PlaybackEvent::new(kind);
        ev.snapshot.phase = phase;
        ev.snapshot.media_type = media;
        ev
    }

    /// The static is process-global, so these run as one sequence rather than
    /// as separate #[test] fns racing each other.
    #[test]
    fn tracks_video_and_ignores_audio() {
        VIDEO_PLAYING.store(false, Ordering::Release);

        deliver(&event(
            PlaybackEventKind::Started,
            PlaybackPhase::Playing,
            MediaType::Audio,
        ));
        assert!(!video_playing(), "audio must never count as video playing");

        deliver(&event(
            PlaybackEventKind::Started,
            PlaybackPhase::Playing,
            MediaType::Video,
        ));
        assert!(video_playing());

        deliver(&event(
            PlaybackEventKind::Paused,
            PlaybackPhase::Paused,
            MediaType::Video,
        ));
        assert!(!video_playing(), "paused video is not playing");

        deliver(&event(
            PlaybackEventKind::Finished,
            PlaybackPhase::Stopped,
            MediaType::Video,
        ));
        assert!(!video_playing());
    }
}
