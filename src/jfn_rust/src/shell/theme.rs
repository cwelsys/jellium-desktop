//! The overlay's palette, ported from the former `web/overlay.css`.

use iced_core::Color;

/// Every opaque pixel the shell overlay puts on screen belongs to a widget:
/// the frame is always cleared fully transparent, so wherever no widget draws,
/// jellyfin-web shows through. The two colours a frame cannot derive for
/// itself are carried here.
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Theme {
    /// The titlebar strip's fill.
    pub chrome_background: Color,
    /// The open modal's backdrop, alpha included.
    pub backdrop: Color,
    pub palette: Palette,
}

impl Default for Theme {
    fn default() -> Self {
        Theme {
            chrome_background: BACKGROUND,
            backdrop: Color::TRANSPARENT,
            palette: Palette::DEFAULT,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Palette {
    pub card: Color,
    pub border: Color,
    pub field: Color,
    pub accent: Color,
    pub accent_fill: Color,
    pub accent_fill_hover: Color,
    pub on_accent: Color,
    pub text: Color,
    pub heading: Color,
    pub muted: Color,
    pub placeholder: Color,
    pub link: Color,
    pub menu: MenuColors,
}

impl Palette {
    pub const DEFAULT: Palette = Palette {
        card: CARD,
        border: CARD_BORDER,
        field: FIELD,
        accent: ACCENT,
        accent_fill: ACCENT_FILL,
        accent_fill_hover: rgb(0x00, 0x7e, 0xac),
        on_accent: Color::WHITE,
        text: TEXT,
        heading: Color::WHITE,
        muted: MUTED,
        placeholder: Color {
            r: 1.0,
            g: 1.0,
            b: 1.0,
            a: 0.5,
        },
        link: LINK,
        menu: MenuColors::DEFAULT,
    };

    pub const CATPPUCCIN_MOCHA: Palette = Palette {
        card: rgb(0x1e, 0x1e, 0x2e),
        border: rgb(0x45, 0x47, 0x5a),
        field: rgb(0x31, 0x32, 0x44),
        accent: rgb(0x89, 0xb4, 0xfa),
        accent_fill: rgb(0x89, 0xb4, 0xfa),
        accent_fill_hover: rgb(0xb4, 0xbe, 0xfe),
        on_accent: rgb(0x1e, 0x1e, 0x2e),
        text: rgb(0xcd, 0xd6, 0xf4),
        heading: rgb(0xcd, 0xd6, 0xf4),
        muted: rgb(0xa6, 0xad, 0xc8),
        placeholder: rgb(0x7f, 0x84, 0x9c),
        link: rgb(0x89, 0xb4, 0xfa),
        menu: MenuColors::CATPPUCCIN_MOCHA,
    };

    pub fn named(name: &str) -> Palette {
        match name {
            CATPPUCCIN_MOCHA_NAME => Palette::CATPPUCCIN_MOCHA,
            _ => Palette::DEFAULT,
        }
    }

    pub fn current() -> Palette {
        Palette::named(&jfn_config::theme())
    }
}

pub const CATPPUCCIN_MOCHA_NAME: &str = "catppuccin-mocha";

pub const THEME_NAMES: [&str; 2] = ["", CATPPUCCIN_MOCHA_NAME];

pub fn theme_label(name: &str) -> String {
    match name {
        CATPPUCCIN_MOCHA_NAME => "Catppuccin Mocha",
        _ => "Default",
    }
    .to_owned()
}

#[cfg(target_os = "linux")]
pub fn publish_menu_palette() {
    let menu = Palette::current().menu;
    jfn_linux_util::menu::set_palette(jfn_linux_util::menu::MenuPalette {
        background: menu.background,
        border: menu.border,
        hover: menu.hover,
        separator: menu.separator,
        text: menu.text,
        text_disabled: menu.text_disabled,
    });
}

#[cfg(not(target_os = "linux"))]
pub fn publish_menu_palette() {}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct MenuColors {
    pub background: [u8; 3],
    pub border: [u8; 3],
    pub hover: [u8; 3],
    pub separator: [u8; 3],
    pub text: [u8; 3],
    pub text_disabled: [u8; 3],
}

impl MenuColors {
    pub const DEFAULT: MenuColors = MenuColors {
        background: [0x2b, 0x2b, 0x2b],
        border: [0x55, 0x55, 0x55],
        hover: [0x3d, 0x3d, 0x3d],
        separator: [0x44, 0x44, 0x44],
        text: [0xe0, 0xe0, 0xe0],
        text_disabled: [0x88, 0x88, 0x88],
    };

    pub const CATPPUCCIN_MOCHA: MenuColors = MenuColors {
        background: [0x1e, 0x1e, 0x2e],
        border: [0x45, 0x47, 0x5a],
        hover: [0x31, 0x32, 0x44],
        separator: [0x31, 0x32, 0x44],
        text: [0xcd, 0xd6, 0xf4],
        text_disabled: [0x7f, 0x84, 0x9c],
    };
}

/// The buffered theme colour jellyfin-web last reported, [`BACKGROUND`] until
/// it unlocks. The titlebar and the modal backdrop both paint it.
pub fn chrome_background() -> Color {
    from_rgb(jfn_color::theme::jfn_theme_color_current())
}

pub(crate) fn from_rgb(value: u32) -> Color {
    rgb(
        ((value >> 16) & 0xff) as u8,
        ((value >> 8) & 0xff) as u8,
        (value & 0xff) as u8,
    )
}

const fn rgb(r: u8, g: u8, b: u8) -> Color {
    Color {
        r: r as f32 / 255.0,
        g: g as f32 / 255.0,
        b: b as f32 / 255.0,
        a: 1.0,
    }
}

/// `#101010`, the same background the app already paints everywhere else.
pub const BACKGROUND: Color = rgb(0x10, 0x10, 0x10);

/// The bundled family, named rather than left to `Family::SansSerif` — which
/// cosmic-text resolves against the system database. Every widget the shell
/// overlay builds takes it as [`iced_core::renderer::Settings::default_font`],
/// so it is the overlay's `Font::DEFAULT` in the only sense that governs what
/// is drawn.
pub const FONT: iced_core::Font = iced_core::Font::new("Noto Sans");

pub const TEXT_SIZE: f32 = 18.0;

pub const HELP_SIZE: f32 = 15.0;

pub const HEADING_SIZE: f32 = 24.0;

/// `#2b2b2b`, the about card's fill.
pub const CARD: Color = rgb(0x2b, 0x2b, 0x2b);

/// `#555555`, the about card's border.
pub const CARD_BORDER: Color = rgb(0x55, 0x55, 0x55);

/// `#00a4dc`.
pub const ACCENT: Color = rgb(0x00, 0xa4, 0xdc);

pub const ACCENT_FILL: Color = rgb(0x00, 0x77, 0xa3);

/// `#363636`.
pub const FIELD: Color = rgb(0x36, 0x36, 0x36);

/// `rgba(255, 255, 255, 0.8)`.
pub const TEXT: Color = Color {
    r: 1.0,
    g: 1.0,
    b: 1.0,
    a: 0.8,
};

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum TextClass {
    #[default]
    Default,
    Muted,
    Heading,
    Link,
}

impl iced_core::widget::text::Catalog for Theme {
    type Class<'a> = TextClass;

    fn default<'a>() -> Self::Class<'a> {
        TextClass::Default
    }

    fn style(&self, class: &Self::Class<'_>) -> iced_core::widget::text::Style {
        let p = &self.palette;
        iced_core::widget::text::Style {
            color: match class {
                TextClass::Default => None,
                TextClass::Muted => Some(p.muted),
                TextClass::Heading => Some(p.heading),
                TextClass::Link => Some(p.link),
            },
        }
    }
}

impl Theme {
    /// Field fill, a two-pixel border that is the accent while focused and
    /// the border colour otherwise, text value, accent-fill selection.
    pub fn field_style(&self, focused: bool) -> crate::shell::field::Style {
        let p = &self.palette;
        crate::shell::field::Style {
            background: iced_core::Background::Color(p.field),
            border: iced_core::Border {
                color: if focused { p.accent } else { p.border },
                width: 2.0,
                radius: 3.0.into(),
            },
            placeholder: p.placeholder,
            value: p.text,
            selection: p.accent_fill,
        }
    }

    /// The focus indicator shared by Settings actions, checkboxes, and selects.
    /// It overlays their existing state without replacing its fill or text style.
    pub fn control_focus_border(&self) -> iced_core::Border {
        iced_core::Border {
            color: self.palette.accent,
            width: 2.0,
            radius: 3.0.into(),
        }
    }
}

impl iced_widget::button::Catalog for Theme {
    type Class<'a> = ButtonClass;

    fn default<'a>() -> Self::Class<'a> {
        ButtonClass::Primary
    }

    fn style(
        &self,
        class: &Self::Class<'_>,
        status: iced_widget::button::Status,
    ) -> iced_widget::button::Style {
        use iced_widget::button::Status;
        let p = &self.palette;
        let base = iced_widget::button::Style {
            background: Some(iced_core::Background::Color(p.accent_fill)),
            text_color: p.on_accent,
            border: iced_core::Border {
                color: Color::TRANSPARENT,
                width: 0.0,
                radius: 3.0.into(),
            },
            shadow: iced_core::Shadow::default(),
            snap: true,
        };
        match (class, status) {
            (ButtonClass::Primary, Status::Hovered | Status::Pressed) => {
                iced_widget::button::Style {
                    background: Some(iced_core::Background::Color(p.accent_fill_hover)),
                    ..base
                }
            }
            (ButtonClass::Primary, Status::Disabled) => iced_widget::button::Style {
                background: Some(iced_core::Background::Color(p.field)),
                text_color: p.muted,
                ..base
            },
            (ButtonClass::Primary, Status::Active) => base,
            (ButtonClass::Chrome, Status::Hovered | Status::Pressed) => {
                iced_widget::button::Style {
                    background: Some(iced_core::Background::Color(Color {
                        a: 0.28,
                        ..rgb(0x7f, 0x7f, 0x7f)
                    })),
                    text_color: CHROME_TEXT,
                    border: iced_core::Border::default(),
                    ..base
                }
            }
            (ButtonClass::Close, Status::Hovered | Status::Pressed) => iced_widget::button::Style {
                background: Some(iced_core::Background::Color(rgb(0xc4, 0x2b, 0x1c))),
                text_color: Color::WHITE,
                border: iced_core::Border::default(),
                ..base
            },
            (ButtonClass::TabSelected, _) => iced_widget::button::Style {
                background: Some(iced_core::Background::Color(p.accent_fill)),
                text_color: p.on_accent,
                border: iced_core::Border::default(),
                ..base
            },
            (ButtonClass::Tab, Status::Hovered | Status::Pressed) => iced_widget::button::Style {
                background: Some(iced_core::Background::Color(p.field)),
                text_color: p.text,
                border: iced_core::Border::default(),
                ..base
            },
            (ButtonClass::Tab, _) => iced_widget::button::Style {
                background: None,
                text_color: p.text,
                border: iced_core::Border::default(),
                ..base
            },
            (ButtonClass::Chrome | ButtonClass::Close, _) => iced_widget::button::Style {
                background: None,
                text_color: CHROME_TEXT,
                border: iced_core::Border::default(),
                ..base
            },
        }
    }
}

impl iced_widget::checkbox::Catalog for Theme {
    type Class<'a> = ();

    fn default<'a>() -> Self::Class<'a> {}

    fn style(
        &self,
        _class: &Self::Class<'_>,
        status: iced_widget::checkbox::Status,
    ) -> iced_widget::checkbox::Style {
        use iced_widget::checkbox::Status;
        let (checked, highlighted) = match status {
            Status::Active { is_checked } => (is_checked, false),
            Status::Hovered { is_checked } => (is_checked, true),
            Status::Disabled { is_checked } => (is_checked, false),
        };
        let p = &self.palette;
        iced_widget::checkbox::Style {
            background: iced_core::Background::Color(if checked { p.accent_fill } else { p.field }),
            icon_color: p.on_accent,
            border: iced_core::Border {
                color: if highlighted { p.accent } else { p.border },
                width: 2.0,
                radius: 3.0.into(),
            },
            text_color: Some(p.text),
        }
    }
}

impl iced_widget::scrollable::Catalog for Theme {
    type Class<'a> = ();

    fn default<'a>() -> Self::Class<'a> {}

    fn style(
        &self,
        _class: &Self::Class<'_>,
        _status: iced_widget::scrollable::Status,
    ) -> iced_widget::scrollable::Style {
        let p = &self.palette;
        let rail = iced_widget::scrollable::Rail {
            background: Some(iced_core::Background::Color(p.field)),
            border: iced_core::Border::default(),
            scroller: iced_widget::scrollable::Scroller {
                background: iced_core::Background::Color(p.border),
                border: iced_core::Border {
                    radius: 3.0.into(),
                    ..iced_core::Border::default()
                },
            },
        };
        iced_widget::scrollable::Style {
            container: iced_widget::container::Style::default(),
            vertical_rail: rail,
            horizontal_rail: rail,
            gap: None,
            auto_scroll: iced_widget::scrollable::AutoScroll {
                background: iced_core::Background::Color(p.card),
                border: iced_core::Border::default(),
                shadow: iced_core::Shadow::default(),
                icon: p.text,
            },
        }
    }
}

impl iced_widget::overlay::menu::Catalog for Theme {
    type Class<'a> = ();

    fn default<'a>() -> <Self as iced_widget::overlay::menu::Catalog>::Class<'a> {}

    fn style(
        &self,
        _class: &<Self as iced_widget::overlay::menu::Catalog>::Class<'_>,
    ) -> iced_widget::overlay::menu::Style {
        let p = &self.palette;
        iced_widget::overlay::menu::Style {
            background: iced_core::Background::Color(p.field),
            border: iced_core::Border {
                color: p.border,
                width: 1.0,
                radius: 3.0.into(),
            },
            text_color: p.text,
            selected_text_color: p.on_accent,
            selected_background: iced_core::Background::Color(p.accent_fill),
            shadow: iced_core::Shadow::default(),
        }
    }
}

impl iced_widget::pick_list::Catalog for Theme {
    type Class<'a> = ();

    fn default<'a>() -> <Self as iced_widget::pick_list::Catalog>::Class<'a> {}

    fn style(
        &self,
        _class: &<Self as iced_widget::pick_list::Catalog>::Class<'_>,
        status: iced_widget::pick_list::Status,
    ) -> iced_widget::pick_list::Style {
        use iced_widget::pick_list::Status;
        let highlighted = matches!(status, Status::Hovered | Status::Opened { .. });
        let p = &self.palette;
        iced_widget::pick_list::Style {
            text_color: p.text,
            placeholder_color: p.placeholder,
            handle_color: p.text,
            background: iced_core::Background::Color(p.field),
            border: iced_core::Border {
                color: if highlighted { p.accent } else { p.border },
                width: 2.0,
                radius: 3.0.into(),
            },
        }
    }
}

impl iced_widget::container::Catalog for Theme {
    type Class<'a> = ContainerClass;

    fn default<'a>() -> Self::Class<'a> {
        ContainerClass::Transparent
    }

    fn style(&self, class: &Self::Class<'_>) -> iced_widget::container::Style {
        let fill = match class {
            ContainerClass::Transparent => None,
            ContainerClass::Backdrop => Some(self.backdrop),
            ContainerClass::Chrome => Some(self.chrome_background),
            ContainerClass::Card => Some(self.palette.card),
        };
        let background = fill.map(iced_core::Background::Color);
        let border = match class {
            ContainerClass::Card => iced_core::Border {
                color: self.palette.border,
                width: 1.0,
                radius: 8.0.into(),
            },
            _ => iced_core::Border::default(),
        };
        iced_widget::container::Style {
            text_color: Some(self.palette.text),
            background,
            border,
            ..iced_widget::container::Style::default()
        }
    }
}

/// `#8ab4f8`, the about panel's clickable paths.
pub const LINK: Color = rgb(0x8a, 0xb4, 0xf8);

/// `#aaa`, help text and the about panel's row labels.
pub const MUTED: Color = rgb(0xaa, 0xaa, 0xaa);

/// `#f0f0f0`, the titlebar's icon colour.
pub const CHROME_TEXT: Color = rgb(0xf0, 0xf0, 0xf0);

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum ButtonClass {
    #[default]
    Primary,
    Chrome,
    Close,
    Tab,
    TabSelected,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum ContainerClass {
    #[default]
    Transparent,
    /// The open modal's full-window field, filled with [`Theme::backdrop`].
    Backdrop,
    /// The titlebar strip, filled opaque with [`Theme::chrome_background`].
    Chrome,
    /// The about panel's body: [`CARD`], a one-pixel [`CARD_BORDER`],
    /// eight-pixel corners.
    Card,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn theme_names_pick_their_palette_and_unknown_falls_back() {
        assert_eq!(Palette::named(""), Palette::DEFAULT);
        assert_eq!(
            Palette::named(CATPPUCCIN_MOCHA_NAME),
            Palette::CATPPUCCIN_MOCHA
        );
        assert_eq!(Palette::named("solarized"), Palette::DEFAULT);
        assert_eq!(theme_label(""), "Default");
        assert_eq!(theme_label(CATPPUCCIN_MOCHA_NAME), "Catppuccin Mocha");
    }
}
