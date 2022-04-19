use std::{
    collections::HashMap,
    fmt::{Debug, Display, Formatter},
    hash::{Hash, Hasher},
};

use crate::terminus::ansi_escape_sequences::{TerminalColor, CSI, SGR};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Color {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    Gray,
    BrightRed,
    BrightGreen,
    BrightYellow,
    BrightBlue,
    BrightMagenta,
    BrightCyan,
    BrightWhite,
    RGB(u8, u8, u8),
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum StyleProperty {
    /// Set foreground color.
    Color(Color),
    BackgroundColor(Color),
    Bold,
    Italic,
    Strike,
    Dim,
    Underline,
    DoublyUnderline,
    Blinking,
    Hidden,
    Invert,
}

impl StyleProperty {
    fn id(&self) -> u8 {
        match self {
            StyleProperty::Color(_) => 0,
            StyleProperty::BackgroundColor(_) => 1,
            StyleProperty::Bold => 2,
            StyleProperty::Italic => 3,
            StyleProperty::Strike => 4,
            StyleProperty::Dim => 5,
            StyleProperty::Underline | StyleProperty::DoublyUnderline => 6,
            StyleProperty::Blinking => 7,
            StyleProperty::Hidden => 8,
            StyleProperty::Invert => 9,
        }
    }
}

impl Hash for StyleProperty {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write_u8(self.id());
        state.finish();
    }
}

impl Display for StyleProperty {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(
            &CSI::SelectGraphicRendition(match self {
                StyleProperty::Color(color) => match color {
                    Color::Black => SGR::SetForegroundColor(TerminalColor::Black),
                    Color::Red => SGR::SetForegroundColor(TerminalColor::Red),
                    Color::Green => SGR::SetForegroundColor(TerminalColor::Green),
                    Color::Yellow => SGR::SetForegroundColor(TerminalColor::Yellow),
                    Color::Blue => SGR::SetForegroundColor(TerminalColor::Blue),
                    Color::Magenta => SGR::SetForegroundColor(TerminalColor::Magenta),
                    Color::Cyan => SGR::SetForegroundColor(TerminalColor::Cyan),
                    Color::White => SGR::SetForegroundColor(TerminalColor::White),
                    Color::Gray => SGR::SetBrightForegroundColor(TerminalColor::Black),
                    Color::BrightRed => SGR::SetBrightForegroundColor(TerminalColor::Red),
                    Color::BrightGreen => SGR::SetBrightForegroundColor(TerminalColor::Green),
                    Color::BrightYellow => SGR::SetBrightForegroundColor(TerminalColor::Yellow),
                    Color::BrightBlue => SGR::SetBrightForegroundColor(TerminalColor::Blue),
                    Color::BrightMagenta => SGR::SetBrightForegroundColor(TerminalColor::Magenta),
                    Color::BrightCyan => SGR::SetBrightForegroundColor(TerminalColor::Cyan),
                    Color::BrightWhite => SGR::SetBrightForegroundColor(TerminalColor::White),
                    Color::RGB(r, g, b) => SGR::SetForegroundColorRGB {
                        r: *r,
                        g: *g,
                        b: *b,
                    },
                },
                StyleProperty::BackgroundColor(color) => match color {
                    Color::Black => SGR::SetBackgroundColor(TerminalColor::Black),
                    Color::Red => SGR::SetBackgroundColor(TerminalColor::Red),
                    Color::Green => SGR::SetBackgroundColor(TerminalColor::Green),
                    Color::Yellow => SGR::SetBackgroundColor(TerminalColor::Yellow),
                    Color::Blue => SGR::SetBackgroundColor(TerminalColor::Blue),
                    Color::Magenta => SGR::SetBackgroundColor(TerminalColor::Magenta),
                    Color::Cyan => SGR::SetBackgroundColor(TerminalColor::Cyan),
                    Color::White => SGR::SetBackgroundColor(TerminalColor::White),
                    Color::Gray => SGR::SetBrightBackgroundColor(TerminalColor::Black),
                    Color::BrightRed => SGR::SetBrightBackgroundColor(TerminalColor::Red),
                    Color::BrightGreen => SGR::SetBrightBackgroundColor(TerminalColor::Green),
                    Color::BrightYellow => SGR::SetBrightBackgroundColor(TerminalColor::Yellow),
                    Color::BrightBlue => SGR::SetBrightBackgroundColor(TerminalColor::Blue),
                    Color::BrightMagenta => SGR::SetBrightBackgroundColor(TerminalColor::Magenta),
                    Color::BrightCyan => SGR::SetBrightBackgroundColor(TerminalColor::Cyan),
                    Color::BrightWhite => SGR::SetBrightBackgroundColor(TerminalColor::White),
                    Color::RGB(r, g, b) => SGR::SetBackgroundColorRGB {
                        r: *r,
                        g: *g,
                        b: *b,
                    },
                },
                StyleProperty::Bold => SGR::Bold,
                StyleProperty::Italic => SGR::Italic,
                StyleProperty::Strike => SGR::Strike,
                StyleProperty::Dim => SGR::Dim,
                StyleProperty::Underline => SGR::Underline,
                StyleProperty::DoublyUnderline => SGR::DoublyUnderlined,
                StyleProperty::Blinking => SGR::SlowBlink,
                StyleProperty::Hidden => SGR::Hide,
                StyleProperty::Invert => SGR::Invert,
            })
            .to_string(),
        )
    }
}

pub struct Style {
    properties: HashMap<u8, StyleProperty>,
}

impl Style {
    pub const RESET: &'static str = "\x1b[0m";

    pub fn new() -> Self {
        Self {
            properties: HashMap::new(),
        }
    }

    pub fn add_property(&mut self, property: StyleProperty) -> &mut Self {
        self.properties.insert(property.id(), property);
        self
    }

    pub fn add_properties(&mut self, properties: &[StyleProperty]) {
        for p in properties {
            self.properties.insert(p.id(), *p);
        }
    }

    pub fn remove_property(&mut self, property: StyleProperty) {
        self.properties.remove(&property.id());
    }

    pub fn remove_properties(&mut self, properties: &[StyleProperty]) {
        for p in properties {
            self.properties.remove(&p.id());
        }
    }

    pub fn prettify(&self, string: &str) -> String {
        let mut str = self.ansi_sequence();
        str += string;
        str += CSI::SelectGraphicRendition(SGR::Reset).to_string().as_str();
        str
    }

    pub fn prettifier(&self) -> impl Fn(&str) -> String {
        let ansi_seq = self.ansi_sequence();
        let reset = CSI::SelectGraphicRendition(SGR::Reset).to_string();
        return move |s: &str| format!("{ansi_seq}{s}{reset}");
    }

    pub fn ansi_sequence(&self) -> String {
        let mut sequence = String::new();
        for style_prop in self.properties.values() {
            sequence += style_prop.to_string().as_str();
        }
        sequence
    }

    pub fn is(&self, property: StyleProperty) -> bool {
        self.properties
            .get(&property.id())
            .map(|p| p == &property)
            .unwrap_or(false)
    }
}

impl<const N: usize> From<&[StyleProperty; N]> for Style {
    fn from(properties: &[StyleProperty; N]) -> Self {
        let mut style = Self::new();
        style.add_properties(properties);
        style
    }
}

impl<const N: usize> From<[StyleProperty; N]> for Style {
    fn from(properties: [StyleProperty; N]) -> Self {
        Self::from(&properties)
    }
}

impl From<&Style> for Style {
    fn from(style: &Style) -> Self {
        Self {
            properties: style.properties.clone(),
        }
    }
}
