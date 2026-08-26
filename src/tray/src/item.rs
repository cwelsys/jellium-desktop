//! StatusNotifierItem. ksni runs the D-Bus connection on its own thread; this
//! module builds the item and forwards clicks to the registered callbacks.

use ksni::menu::StandardItem;
use ksni::{Handle, MenuItem, Status, ToolTip, TrayMethods};

use crate::Callbacks;

const ICON_NAME: &str = "net.nullsum.JelliumDesktop";
const TITLE: &str = "Jellium Desktop";

pub(crate) struct JelliumTray {
    pub(crate) callbacks: Callbacks,
}

impl ksni::Tray for JelliumTray {
    fn id(&self) -> String {
        ICON_NAME.into()
    }

    fn title(&self) -> String {
        TITLE.into()
    }

    fn icon_name(&self) -> String {
        ICON_NAME.into()
    }

    fn status(&self) -> Status {
        Status::Active
    }

    fn tool_tip(&self) -> ToolTip {
        ToolTip {
            icon_name: ICON_NAME.into(),
            title: TITLE.into(),
            description: String::new(),
            icon_pixmap: Vec::new(),
        }
    }

    /// Left click.
    fn activate(&mut self, _x: i32, _y: i32) {
        (self.callbacks.toggle)();
    }

    fn menu(&self) -> Vec<MenuItem<Self>> {
        vec![
            StandardItem {
                label: "Open Jellium Desktop".into(),
                activate: Box::new(|this: &mut Self| (this.callbacks.show)()),
                ..Default::default()
            }
            .into(),
            MenuItem::Separator,
            StandardItem {
                label: "Quit".into(),
                activate: Box::new(|this: &mut Self| (this.callbacks.quit)()),
                ..Default::default()
            }
            .into(),
        ]
    }
}

/// `TrayMethods::spawn` is async (ksni's own executor drives the D-Bus
/// connection on a background thread regardless); this crate runs no async
/// runtime of its own, so block on it here as `jfn_mpris` does for zbus calls.
pub(crate) fn spawn(tray: JelliumTray) -> Result<Handle<JelliumTray>, ksni::Error> {
    async_io::block_on(tray.spawn())
}
