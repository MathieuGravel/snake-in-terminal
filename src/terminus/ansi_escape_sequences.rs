use std::fmt::{Debug, Display, Formatter, Write};

const ESCAPE: char = 0x1B_u8 as char;

#[derive(Debug, Clone, Copy)]
pub enum ControlCharacters {
    /// Make bell sound
    Bell,
    Backspace,
    Tab,
    LineFeed,
    FormFeed,
    CarriageReturn,
    Escape,
}

impl Display for ControlCharacters {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_char(match self {
            Self::Bell => 0x07_u8,
            Self::Backspace => 0x08_u8,
            Self::Tab => 0x09_u8,
            Self::LineFeed => 0x0A_u8,
            Self::FormFeed => 0x0C_u8,
            Self::CarriageReturn => 0x0D_u8,
            Self::Escape => ESCAPE as u8,
        } as char)
    }
}

/// Control Sequence Introducer
pub enum CSI {
    /// Move the cursor up by n.
    ///
    /// Stop at the edge of the screen.
    CursorUp(u16),
    /// Move the cursor down by n.
    ///
    /// Stop at the edge of the screen.
    CursorDown(u16),
    /// Move the cursor forward by n.
    ///
    /// Stop at the edge of the screen.
    CursorForward(u16),
    /// Move the cursor back by n.
    ///
    /// Stop at the edge of the screen.
    CursorBack(u16),
    /// Move the cursor at the start of the n next line.
    ///
    /// Stop at the edge of the screen.
    CursorNextLine(u16),
    /// Move the cursor at the start of the n previous line.
    ///
    /// Stop at the edge of the screen.
    CursorPreviousLine(u16),
    /// Move the cursor horizontally to position n.
    ///
    /// Stop at the edge of the screen.
    CursorHorizontalAbsolute(u16),
    /// Move the cursor to position n, m.
    ///
    /// where n=1 and m=1 is the top left corner.
    ///
    /// Stop at the edge of the screen.
    CursorPosition(u16, u16),
    /// Erase text in display.
    ///
    /// When use with option All, it clear the window but keep previous history.
    /// If you want to delete the screen without keeping the previous print in the screen, move the cursor to 1, 1 and delete with option: CursorToEnd.
    EraseInDisplay(EraseOption),
    /// Erase text in line.
    EraseInLine(EraseOption),
    /// Scroll the text up but does not change cursor position.
    ScrollUp(u16),
    /// Scroll the text down but does not change cursor position.
    ScrollDown(u16),
    /// Set the position of the cursor without exceeding the window bound.
    HorizontalVerticalPosition(u16, u16),
    /// Set Graphic Rendition property.
    SelectGraphicRendition(SGR),
    AuxPortOn,
    AuxPortOff,
    DeviceStatusReport,
    /// Save the current cursor position.
    SaveCursorPosition,
    /// Restore the saved cursor position.
    ///
    /// If nothing saved, some terminal used position 1, 1 or the end of text.
    RestoreCursorPosition,
    HideCursor,
    ShowCursor,
}

impl CSI {
    fn write_parameter(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::CursorUp(n)
            | Self::CursorDown(n)
            | Self::CursorForward(n)
            | Self::CursorBack(n)
            | Self::CursorNextLine(n)
            | Self::CursorPreviousLine(n)
            | Self::CursorHorizontalAbsolute(n)
            | Self::ScrollUp(n)
            | Self::ScrollDown(n) => f.write_str(&n.to_string()),
            Self::CursorPosition(n, m) | Self::HorizontalVerticalPosition(n, m) => {
                f.write_str(&format!("{};{}", n, m))
            }
            Self::EraseInDisplay(eo) | Self::EraseInLine(eo) => {
                f.write_str((*eo as u8).to_string().as_str())
            }
            Self::SelectGraphicRendition(srg) => srg.fmt(f),
            Self::AuxPortOn => f.write_char('5'),
            Self::AuxPortOff => f.write_char('4'),
            Self::DeviceStatusReport => f.write_char('6'),
            CSI::SaveCursorPosition | CSI::RestoreCursorPosition => Ok(()),
            CSI::HideCursor | CSI::ShowCursor => f.write_str("?25"),
        }
    }

    fn write_letter_flag(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_char(match self {
            Self::CursorUp(_) => 'A',
            Self::CursorDown(_) => 'B',
            Self::CursorForward(_) => 'C',
            Self::CursorBack(_) => 'D',
            Self::CursorNextLine(_) => 'E',
            Self::CursorPreviousLine(_) => 'F',
            Self::CursorHorizontalAbsolute(_) => 'G',
            Self::CursorPosition(_, _) => 'H',
            Self::EraseInDisplay(_) => 'J',
            Self::EraseInLine(_) => 'K',
            Self::ScrollUp(_) => 'S',
            Self::ScrollDown(_) => 'T',
            Self::HorizontalVerticalPosition(_, _) => 'f',
            Self::SelectGraphicRendition(_) => 'm',
            Self::AuxPortOn | Self::AuxPortOff => 'i',
            Self::DeviceStatusReport => 'n',
            CSI::SaveCursorPosition => 's',
            CSI::RestoreCursorPosition => 'u',
            CSI::HideCursor => 'l',
            CSI::ShowCursor => 'h',
        })
    }
}

impl Display for CSI {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_char(ESCAPE)?;
        f.write_char('[')?;
        self.write_parameter(f)?;
        self.write_letter_flag(f)
    }
}

impl Debug for CSI {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("esc")?;
        f.write_char('[')?;
        self.write_parameter(f)?;
        self.write_letter_flag(f)
    }
}

/// Select Graphic Rendition
pub enum SGR {
    /// Reset all Graphic Rendition.
    Reset,
    /// Render text in Bold.
    Bold,
    /// Render text with decreased intensity.
    Dim,
    /// Render text in Italic.
    Italic,
    /// Render text with underline.
    Underline,
    SlowBlink,
    RapidBlink,
    /// Swap foreground color with background color.
    Invert,
    /// The text will not be rendered.
    Hide,
    /// Render text with strike.
    Strike,
    DefaultFont,
    SetAlternativeFont(u8),
    Fraktur,
    /// Render text with double underline.
    DoublyUnderlined,
    NormalIntensity,
    NeitherItalicNorBlackletter,
    NotUnderlined,
    NotBlinking,
    ProportionalSpacing,
    NotReversed,
    Reveal,
    NotCrossedOut,
    SetForegroundColor(TerminalColor),
    SetForegroundColorRGB {
        r: u8,
        g: u8,
        b: u8,
    },
    DefaultForegroundColor,
    SetBackgroundColor(TerminalColor),
    SetBackgroundColorRGB {
        r: u8,
        g: u8,
        b: u8,
    },
    DefaultBackgroundColor,
    DisableProportionalSpacing,
    Framed,
    Encircled,
    Overlined,
    NeitherFramedNorEncircled,
    NotOverlined,
    SetUnderlineColor,
    DefaultUnderlineColor,
    // ...
    SetBrightForegroundColor(TerminalColor),
    SetBrightBackgroundColor(TerminalColor),
}

impl Display for SGR {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Reset => f.write_char('0'),
            Self::Bold => f.write_char('1'),
            Self::Dim => f.write_char('2'),
            Self::Italic => f.write_char('3'),
            Self::Underline => f.write_char('4'),
            Self::SlowBlink => f.write_char('5'),
            Self::RapidBlink => f.write_char('6'),
            Self::Invert => f.write_char('7'),
            Self::Hide => f.write_char('8'),
            Self::Strike => f.write_char('9'),
            Self::DefaultFont => f.write_str("10"),
            Self::SetAlternativeFont(font) => f.write_str(&format!("1{}", font)),
            Self::Fraktur => f.write_str("20"),
            Self::DoublyUnderlined => f.write_str("21"),
            Self::NormalIntensity => f.write_str("22"),
            Self::NeitherItalicNorBlackletter => f.write_str("23"),
            Self::NotUnderlined => f.write_str("24"),
            Self::NotBlinking => f.write_str("25"),
            Self::ProportionalSpacing => f.write_str("26"),
            Self::NotReversed => f.write_str("27"),
            Self::Reveal => f.write_str("28"),
            Self::NotCrossedOut => f.write_str("29"),
            Self::SetForegroundColor(terminal_color) => {
                f.write_str(&format!("3{}", *terminal_color as u8))
            }
            Self::SetForegroundColorRGB { r, g, b } => {
                f.write_str(&format!("38;2;{};{};{}", r, g, b))
            }
            Self::DefaultForegroundColor => f.write_str("39"),
            Self::SetBackgroundColor(terminal_color) => {
                f.write_str(&format!("4{}", *terminal_color as u8))
            }
            Self::SetBackgroundColorRGB { r, g, b } => {
                f.write_str(&format!("48;2;{};{};{}", r, g, b))
            }
            Self::DefaultBackgroundColor => f.write_str("49"),
            Self::DisableProportionalSpacing => f.write_str("50"),
            Self::Framed => f.write_str("51"),
            Self::Encircled => f.write_str("52"),
            Self::Overlined => f.write_str("53"),
            Self::NeitherFramedNorEncircled => f.write_str("54"),
            Self::NotOverlined => f.write_str("55"),
            Self::SetUnderlineColor => f.write_str("58"),
            Self::DefaultUnderlineColor => f.write_str("59"),
            Self::SetBrightForegroundColor(terminal_color) => {
                f.write_str(&format!("9{}", *terminal_color as u8))
            }
            Self::SetBrightBackgroundColor(terminal_color) => {
                f.write_str(&format!("10{}", *terminal_color as u8))
            }
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum TerminalColor {
    Black = 0,
    Red = 1,
    Green = 2,
    Yellow = 3,
    Blue = 4,
    Magenta = 5,
    Cyan = 6,
    White = 7,
}

#[derive(Debug, Clone, Copy)]
pub enum EraseOption {
    CursorToEnd = 0,
    CursorToBeginning = 1,
    All = 2,
}
