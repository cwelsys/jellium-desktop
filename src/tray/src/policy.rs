//! Pure visibility policy. No I/O, no D-Bus, no window handles: the
//! decisions live here so they are testable without a compositor or a bus.

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CloseAction {
    Hide,
    Quit,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VisibilityAction {
    Show,
    Hide,
    Nothing,
}

/// What hiding does to playback that is already running.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PlaybackAction {
    Pause,
    Nothing,
}

/// Without a registered tray item there is no way back to a hidden window, so
/// close quits whatever the setting says.
pub fn on_close(tray_available: bool, close_to_tray: bool) -> CloseAction {
    if tray_available && close_to_tray {
        CloseAction::Hide
    } else {
        CloseAction::Quit
    }
}

/// `raised_by_playback` distinguishes a window playback raised from one the
/// user opened; only the former is hidden again when playback ends. Returns the
/// action and the new flag value.
pub fn on_video_mode(
    active: bool,
    hidden: bool,
    raised_by_playback: bool,
) -> (VisibilityAction, bool) {
    match (active, hidden) {
        (true, true) => (VisibilityAction::Show, true),
        (true, false) => (VisibilityAction::Nothing, raised_by_playback),
        (false, _) if raised_by_playback => (VisibilityAction::Hide, false),
        (false, _) => (VisibilityAction::Nothing, false),
    }
}

/// An unmapped window shows nothing, so video playing on into it wastes a
/// decode and leaves the user's place behind. Audio is the opposite: playing
/// on with no window is the point of the tray, so it is never paused. The rule
/// does not care who started the video — a cast is paused like anything else.
pub fn on_hide(video_playing: bool) -> PlaybackAction {
    if video_playing {
        PlaybackAction::Pause
    } else {
        PlaybackAction::Nothing
    }
}

/// Video resuming from any source — MPRIS, a media key, the JS UI — has to put
/// the window back, or playback would run on with nothing on screen. Showing
/// the window is not the mirror of this: it deliberately leaves playback
/// paused, so looking at the window never starts audio.
pub fn on_video_resumed(hidden: bool) -> VisibilityAction {
    if hidden {
        VisibilityAction::Show
    } else {
        VisibilityAction::Nothing
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn close_hides_only_with_tray_and_setting() {
        assert_eq!(on_close(true, true), CloseAction::Hide);
        assert_eq!(on_close(true, false), CloseAction::Quit);
        assert_eq!(on_close(false, true), CloseAction::Quit);
        assert_eq!(on_close(false, false), CloseAction::Quit);
    }

    #[test]
    fn video_start_raises_only_when_hidden() {
        assert_eq!(
            on_video_mode(true, true, false),
            (VisibilityAction::Show, true)
        );
        assert_eq!(
            on_video_mode(true, false, false),
            (VisibilityAction::Nothing, false)
        );
    }

    #[test]
    fn video_end_hides_only_what_playback_raised() {
        assert_eq!(
            on_video_mode(false, false, true),
            (VisibilityAction::Hide, false)
        );
        assert_eq!(
            on_video_mode(false, false, false),
            (VisibilityAction::Nothing, false)
        );
    }

    #[test]
    fn hide_pauses_video_and_leaves_audio_alone() {
        assert_eq!(on_hide(true), PlaybackAction::Pause);
        assert_eq!(on_hide(false), PlaybackAction::Nothing);
    }

    #[test]
    fn video_resume_shows_only_a_hidden_window() {
        assert_eq!(on_video_resumed(true), VisibilityAction::Show);
        assert_eq!(on_video_resumed(false), VisibilityAction::Nothing);
    }

    #[test]
    fn video_start_while_visible_preserves_the_flag() {
        assert_eq!(
            on_video_mode(true, false, true),
            (VisibilityAction::Nothing, true)
        );
    }
}
