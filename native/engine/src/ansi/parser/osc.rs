#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OscCommand {
    SetClipboard(ClipboardData),
    SetHyperlink(Hyperlink),
    SetIconName(String),
    SetTitle(String),
    SetBackgroundColor(String),
    SetForegroundColor(String),
    SetCursorColor(String),
    SetMouseCursorShape(String),
    SetWorkingDirectory(String),
    InvalidUrl(String),
    Unknown(u32, Vec<u8>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ClipboardData {
    pub data: String,
    pub selection: ClipboardSelection,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ClipboardSelection {
    Clipboard,
    Primary,
    Secondary,
    Tertiary,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Hyperlink {
    pub id: Option<String>,
    pub uri: String,
}

impl OscCommand {
    pub fn parse(data: &[u8]) -> Option<Self> {
        let s = String::from_utf8_lossy(data);
        let parts: Vec<&str> = s.splitn(2, ';').collect();

        if parts.len() < 2 {
            return None;
        }

        let code: u32 = parts[0].parse().ok()?;
        let value = parts[1];

        match code {
            52 => {
                let clipboard_parts: Vec<&str> = value.splitn(2, ';').collect();
                if clipboard_parts.len() < 2 {
                    return None;
                }

                let selection = match clipboard_parts[0] {
                    "c" | "Clipboard" => ClipboardSelection::Clipboard,
                    "p" | "Primary" => ClipboardSelection::Primary,
                    "s" | "Secondary" => ClipboardSelection::Secondary,
                    "0" => ClipboardSelection::Clipboard,
                    "1" => ClipboardSelection::Primary,
                    "2" => ClipboardSelection::Secondary,
                    "3" => ClipboardSelection::Tertiary,
                    _ => ClipboardSelection::Clipboard,
                };

                let data = clipboard_parts[1].to_string();
                Some(Self::SetClipboard(ClipboardData { data, selection }))
            }
            8 => {
                let link_parts: Vec<&str> = value.splitn(2, ';').collect();
                let id = if link_parts[0].is_empty() {
                    None
                } else {
                    Some(link_parts[0].to_string())
                };
                let uri = link_parts.get(1).unwrap_or(&"").to_string();
                Some(Self::SetHyperlink(Hyperlink { id, uri }))
            }
            0 | 2 => Some(Self::SetTitle(value.to_string())),
            1 => Some(Self::SetIconName(value.to_string())),
            10 => Some(Self::SetForegroundColor(value.to_string())),
            11 => Some(Self::SetBackgroundColor(value.to_string())),
            12 => Some(Self::SetCursorColor(value.to_string())),
            13 => Some(Self::SetMouseCursorShape(value.to_string())),
            7 => Some(Self::SetWorkingDirectory(value.to_string())),
            _ => Some(Self::Unknown(code, data.to_vec())),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn osc_set_title() {
        let cmd = OscCommand::parse(b"2;My Terminal");
        assert_eq!(cmd, Some(OscCommand::SetTitle("My Terminal".to_string())));
    }

    #[test]
    fn osc_set_clipboard() {
        let cmd = OscCommand::parse(b"52;c;SGVsbG8=");
        assert!(matches!(
            cmd,
            Some(OscCommand::SetClipboard(ClipboardData {
                selection: ClipboardSelection::Clipboard,
                ..
            }))
        ));
    }

    #[test]
    fn osc_set_hyperlink() {
        let cmd = OscCommand::parse(b"8;;https://example.com");
        assert_eq!(
            cmd,
            Some(OscCommand::SetHyperlink(Hyperlink {
                id: None,
                uri: "https://example.com".to_string(),
            }))
        );
    }

    #[test]
    fn osc_set_hyperlink_with_id() {
        let cmd = OscCommand::parse(b"8;id=link1;https://example.com");
        assert_eq!(
            cmd,
            Some(OscCommand::SetHyperlink(Hyperlink {
                id: Some("id=link1".to_string()),
                uri: "https://example.com".to_string(),
            }))
        );
    }
}
