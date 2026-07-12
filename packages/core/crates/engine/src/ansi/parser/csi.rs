#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CsiCommand {
    CursorMovement(CursorMovement),
    Erase(EraseMode),
    Scroll(ScrollDirection, u32),
    Sgr(Vec<SgrAttribute>),
    DeviceStatus(DeviceStatus),
    Mode(ModeAction, ModeType),
    TabStop(TabStopAction),
    DeleteLine(u32),
    InsertLine(u32),
    DeleteChar(u32),
    InsertChar(u32),
    EraseChar(u32),
    CursorPositionSave,
    CursorPositionRestore,
    AttributeReset,
    DeviceAttributes(Vec<u32>),
    SecondaryDeviceAttributes(Vec<u32>),
    TertiaryDeviceAttributes(String),
    KittyKeyEvent {
        keycode: u32,
        modifiers: u32,
        event_type: KittyEventType,
        associated_text: Option<String>,
    },
    KittyEnhancementLevel {
        level: u8,
        action: ModeAction,
    },
    KittyKeyboardQuery(Vec<u32>),
    Unknown(u8, Vec<u32>),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KittyEventType {
    Press,
    Repeat,
    Release,
    Unknown,
}

impl KittyEventType {
    pub fn from_flag(flag: u32) -> Self {
        match flag {
            1 => Self::Press,
            2 => Self::Repeat,
            3 => Self::Release,
            _ => Self::Unknown,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CursorMovement {
    Up(u32),
    Down(u32),
    Forward(u32),
    Backward(u32),
    NextLine(u32),
    PreviousLine(u32),
    ColumnAbsolute(u32),
    Position(u32, u32),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EraseMode {
    CursorToEnd,
    CursorToBeginning,
    Entire,
    CursorToEndLines,
    CursorToBeginningLines,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScrollDirection {
    Up,
    Down,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeviceStatus {
    ReportCursorPosition,
    DeviceAttributes,
    DeviceAttributes2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ModeAction {
    Set,
    Reset,
    Save,
    Restore,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ModeType {
    Normal(u32),
    Private(u32),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TabStopAction {
    Set,
    Clear,
    ClearAll,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SgrAttribute {
    Reset,
    Bold,
    Dim,
    Italic,
    Underline,
    Blink,
    Inverse,
    Hidden,
    Strikethrough,
    Foreground(ForegroundColor),
    Background(BackgroundColor),
    UnderlineColor(UnderlineColor),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ForegroundColor {
    Default,
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    BrightBlack,
    BrightRed,
    BrightGreen,
    BrightYellow,
    BrightBlue,
    BrightMagenta,
    BrightCyan,
    BrightWhite,
    Extended(u8),
    Rgb(u8, u8, u8),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BackgroundColor {
    Default,
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    BrightBlack,
    BrightRed,
    BrightGreen,
    BrightYellow,
    BrightBlue,
    BrightMagenta,
    BrightCyan,
    BrightWhite,
    Extended(u8),
    Rgb(u8, u8, u8),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnderlineColor {
    Default,
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    BrightBlack,
    BrightRed,
    BrightGreen,
    BrightYellow,
    BrightBlue,
    BrightMagenta,
    BrightCyan,
    BrightWhite,
    Extended(u8),
    Rgb(u8, u8, u8),
}

impl CsiCommand {
    pub fn parse(final_byte: u8, params: &[u32], intermediate: &[u8]) -> Option<Self> {
        match final_byte {
            b'A' => Some(Self::CursorMovement(CursorMovement::Up(
                params.first().copied().unwrap_or(1),
            ))),
            b'B' => Some(Self::CursorMovement(CursorMovement::Down(
                params.first().copied().unwrap_or(1),
            ))),
            b'C' => Some(Self::CursorMovement(CursorMovement::Forward(
                params.first().copied().unwrap_or(1),
            ))),
            b'D' => Some(Self::CursorMovement(CursorMovement::Backward(
                params.first().copied().unwrap_or(1),
            ))),
            b'E' => Some(Self::CursorMovement(CursorMovement::NextLine(
                params.first().copied().unwrap_or(1),
            ))),
            b'F' => Some(Self::CursorMovement(CursorMovement::PreviousLine(
                params.first().copied().unwrap_or(1),
            ))),
            b'G' => Some(Self::CursorMovement(CursorMovement::ColumnAbsolute(
                params.first().copied().unwrap_or(1),
            ))),
            b'H' | b'f' => {
                let row = params.first().copied().unwrap_or(1);
                let col = params.get(1).copied().unwrap_or(1);
                Some(Self::CursorMovement(CursorMovement::Position(row, col)))
            }
            b'J' => {
                let mode = match params.first().copied().unwrap_or(0) {
                    0 => EraseMode::CursorToEnd,
                    1 => EraseMode::CursorToBeginning,
                    2 => EraseMode::Entire,
                    3 => EraseMode::CursorToEndLines,
                    _ => EraseMode::CursorToEnd,
                };
                Some(Self::Erase(mode))
            }
            b'K' => {
                let mode = match params.first().copied().unwrap_or(0) {
                    0 => EraseMode::CursorToEnd,
                    1 => EraseMode::CursorToBeginning,
                    2 => EraseMode::Entire,
                    _ => EraseMode::CursorToEnd,
                };
                Some(Self::Erase(mode))
            }
            b'L' => Some(Self::InsertLine(params.first().copied().unwrap_or(1))),
            b'M' => Some(Self::DeleteLine(params.first().copied().unwrap_or(1))),
            b'P' => Some(Self::DeleteChar(params.first().copied().unwrap_or(1))),
            b'@' => Some(Self::InsertChar(params.first().copied().unwrap_or(1))),
            b'X' => Some(Self::EraseChar(params.first().copied().unwrap_or(1))),
            b'S' => Some(Self::Scroll(
                ScrollDirection::Up,
                params.first().copied().unwrap_or(1),
            )),
            b'T' => Some(Self::Scroll(
                ScrollDirection::Down,
                params.first().copied().unwrap_or(1),
            )),
            b'n' => {
                if params.first() == Some(&6) {
                    Some(Self::DeviceStatus(DeviceStatus::ReportCursorPosition))
                } else if params.first() == Some(&0) {
                    Some(Self::DeviceStatus(DeviceStatus::DeviceAttributes))
                } else {
                    None
                }
            }
            b'c' => {
                if intermediate.first() == Some(&b'?') {
                    Some(Self::DeviceAttributes(params.to_vec()))
                } else if intermediate.first() == Some(&b'>') {
                    Some(Self::SecondaryDeviceAttributes(params.to_vec()))
                } else {
                    Some(Self::DeviceStatus(DeviceStatus::DeviceAttributes))
                }
            }
            b'}' | b'|' => {
                if intermediate.first() == Some(&b'!') {
                    Some(Self::TertiaryDeviceAttributes(String::new()))
                } else {
                    None
                }
            }
            b'h' => {
                if intermediate.first() == Some(&b'?') {
                    let mode = params.first().copied().unwrap_or(0);
                    if mode == 27127 {
                        let level = params.get(1).copied().unwrap_or(1) as u8;
                        Some(Self::KittyEnhancementLevel {
                            level,
                            action: ModeAction::Set,
                        })
                    } else {
                        Some(Self::Mode(ModeAction::Set, ModeType::Private(mode)))
                    }
                } else {
                    let mode = params.first().copied().unwrap_or(0);
                    Some(Self::Mode(ModeAction::Set, ModeType::Normal(mode)))
                }
            }
            b'l' => {
                if intermediate.first() == Some(&b'?') {
                    let mode = params.first().copied().unwrap_or(0);
                    if mode == 27127 {
                        let level = params.get(1).copied().unwrap_or(1) as u8;
                        Some(Self::KittyEnhancementLevel {
                            level,
                            action: ModeAction::Reset,
                        })
                    } else {
                        Some(Self::Mode(ModeAction::Reset, ModeType::Private(mode)))
                    }
                } else {
                    let mode = params.first().copied().unwrap_or(0);
                    Some(Self::Mode(ModeAction::Reset, ModeType::Normal(mode)))
                }
            }
            b's' => Some(Self::CursorPositionSave),
            b'u' => {
                if intermediate.first() == Some(&b'?') {
                    Some(Self::KittyKeyboardQuery(params.to_vec()))
                } else if params.is_empty()
                    || params.first() == Some(&0)
                    || params.first() == Some(&1)
                {
                    Some(Self::CursorPositionRestore)
                } else {
                    let keycode = params[0];
                    let modifiers = params.get(1).copied().unwrap_or(0);
                    let event_type_value = params.get(2).copied().unwrap_or(1);
                    let event_type = KittyEventType::from_flag(event_type_value);
                    let associated_text = None;
                    Some(Self::KittyKeyEvent {
                        keycode,
                        modifiers,
                        event_type,
                        associated_text,
                    })
                }
            }
            b'm' => Some(Self::Sgr(parse_sgr(params))),
            b'g' => {
                let action = match params.first().copied().unwrap_or(0) {
                    0 => TabStopAction::Set,
                    2 => TabStopAction::Clear,
                    3 => TabStopAction::ClearAll,
                    _ => TabStopAction::Set,
                };
                Some(Self::TabStop(action))
            }
            _ => Some(Self::Unknown(final_byte, params.to_vec())),
        }
    }
}

fn parse_sgr(params: &[u32]) -> Vec<SgrAttribute> {
    let mut attrs = Vec::new();
    let mut i = 0;

    while i < params.len() {
        match params[i] {
            0 => attrs.push(SgrAttribute::Reset),
            1 => attrs.push(SgrAttribute::Bold),
            2 => attrs.push(SgrAttribute::Dim),
            3 => attrs.push(SgrAttribute::Italic),
            4 => attrs.push(SgrAttribute::Underline),
            5 => attrs.push(SgrAttribute::Blink),
            7 => attrs.push(SgrAttribute::Inverse),
            8 => attrs.push(SgrAttribute::Hidden),
            9 => attrs.push(SgrAttribute::Strikethrough),
            30..=37 => {
                let color = match params[i] - 30 {
                    0 => ForegroundColor::Black,
                    1 => ForegroundColor::Red,
                    2 => ForegroundColor::Green,
                    3 => ForegroundColor::Yellow,
                    4 => ForegroundColor::Blue,
                    5 => ForegroundColor::Magenta,
                    6 => ForegroundColor::Cyan,
                    7 => ForegroundColor::White,
                    _ => ForegroundColor::Black,
                };
                attrs.push(SgrAttribute::Foreground(color));
            }
            38 => {
                if let Some(color) = parse_extended_color(params, &mut i) {
                    attrs.push(SgrAttribute::Foreground(color));
                }
            }
            39 => attrs.push(SgrAttribute::Foreground(ForegroundColor::Default)),
            40..=47 => {
                let color = match params[i] - 40 {
                    0 => BackgroundColor::Black,
                    1 => BackgroundColor::Red,
                    2 => BackgroundColor::Green,
                    3 => BackgroundColor::Yellow,
                    4 => BackgroundColor::Blue,
                    5 => BackgroundColor::Magenta,
                    6 => BackgroundColor::Cyan,
                    7 => BackgroundColor::White,
                    _ => BackgroundColor::Black,
                };
                attrs.push(SgrAttribute::Background(color));
            }
            48 => {
                if let Some(color) = parse_extended_bg_color(params, &mut i) {
                    attrs.push(SgrAttribute::Background(color));
                }
            }
            49 => attrs.push(SgrAttribute::Background(BackgroundColor::Default)),
            90..=97 => {
                let color = match params[i] - 90 {
                    0 => ForegroundColor::BrightBlack,
                    1 => ForegroundColor::BrightRed,
                    2 => ForegroundColor::BrightGreen,
                    3 => ForegroundColor::BrightYellow,
                    4 => ForegroundColor::BrightBlue,
                    5 => ForegroundColor::BrightMagenta,
                    6 => ForegroundColor::BrightCyan,
                    7 => ForegroundColor::BrightWhite,
                    _ => ForegroundColor::BrightBlack,
                };
                attrs.push(SgrAttribute::Foreground(color));
            }
            100..=107 => {
                let color = match params[i] - 100 {
                    0 => BackgroundColor::BrightBlack,
                    1 => BackgroundColor::BrightRed,
                    2 => BackgroundColor::BrightGreen,
                    3 => BackgroundColor::BrightYellow,
                    4 => BackgroundColor::BrightBlue,
                    5 => BackgroundColor::BrightMagenta,
                    6 => BackgroundColor::BrightCyan,
                    7 => BackgroundColor::BrightWhite,
                    _ => BackgroundColor::BrightBlack,
                };
                attrs.push(SgrAttribute::Background(color));
            }
            _ => {}
        }
        i += 1;
    }

    attrs
}

fn parse_extended_color(params: &[u32], i: &mut usize) -> Option<ForegroundColor> {
    if *i + 1 >= params.len() {
        return None;
    }

    match params[*i + 1] {
        5 => {
            if *i + 2 < params.len() {
                *i += 2;
                Some(ForegroundColor::Extended(params[*i] as u8))
            } else {
                None
            }
        }
        2 => {
            if *i + 4 < params.len() {
                *i += 4;
                Some(ForegroundColor::Rgb(
                    params[*i - 2] as u8,
                    params[*i - 1] as u8,
                    params[*i] as u8,
                ))
            } else {
                None
            }
        }
        _ => None,
    }
}

fn parse_extended_bg_color(params: &[u32], i: &mut usize) -> Option<BackgroundColor> {
    if *i + 1 >= params.len() {
        return None;
    }

    match params[*i + 1] {
        5 => {
            if *i + 2 < params.len() {
                *i += 2;
                Some(BackgroundColor::Extended(params[*i] as u8))
            } else {
                None
            }
        }
        2 => {
            if *i + 4 < params.len() {
                *i += 4;
                Some(BackgroundColor::Rgb(
                    params[*i - 2] as u8,
                    params[*i - 1] as u8,
                    params[*i] as u8,
                ))
            } else {
                None
            }
        }
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn csi_cursor_up() {
        let cmd = CsiCommand::parse(b'A', &[5], &[]);
        assert_eq!(cmd, Some(CsiCommand::CursorMovement(CursorMovement::Up(5))));
    }

    #[test]
    fn csi_cursor_down() {
        let cmd = CsiCommand::parse(b'B', &[3], &[]);
        assert_eq!(
            cmd,
            Some(CsiCommand::CursorMovement(CursorMovement::Down(3)))
        );
    }

    #[test]
    fn csi_cursor_position() {
        let cmd = CsiCommand::parse(b'H', &[10, 20], &[]);
        assert_eq!(
            cmd,
            Some(CsiCommand::CursorMovement(CursorMovement::Position(10, 20)))
        );
    }

    #[test]
    fn csi_erase_display() {
        let cmd = CsiCommand::parse(b'J', &[2], &[]);
        assert_eq!(cmd, Some(CsiCommand::Erase(EraseMode::Entire)));
    }

    #[test]
    fn csi_erase_line() {
        let cmd = CsiCommand::parse(b'K', &[0], &[]);
        assert_eq!(cmd, Some(CsiCommand::Erase(EraseMode::CursorToEnd)));
    }

    #[test]
    fn csi_sgr_bold() {
        let cmd = CsiCommand::parse(b'm', &[1], &[]);
        assert!(matches!(cmd, Some(CsiCommand::Sgr(attrs)) if attrs.contains(&SgrAttribute::Bold)));
    }

    #[test]
    fn csi_sgr_reset() {
        let cmd = CsiCommand::parse(b'm', &[0], &[]);
        assert!(
            matches!(cmd, Some(CsiCommand::Sgr(attrs)) if attrs.contains(&SgrAttribute::Reset))
        );
    }

    #[test]
    fn csi_device_status() {
        let cmd = CsiCommand::parse(b'n', &[6], &[]);
        assert_eq!(
            cmd,
            Some(CsiCommand::DeviceStatus(DeviceStatus::ReportCursorPosition))
        );
    }
}
