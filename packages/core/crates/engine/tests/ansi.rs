use bettertui_engine::ansi::*;

#[test]
fn encoder_new() {
    let enc = AnsiEncoder::new();
    assert!(enc.finish().is_empty());
}

#[test]
fn encoder_encode_empty() {
    let mut enc = AnsiEncoder::new();
    let fb = bettertui_engine::framebuffer::FrameBuffer::new(5, 5);
    enc.encode(&fb, &[]);
    let out = enc.finish();
    let s = String::from_utf8_lossy(out);
    assert!(s.contains("\x1b[?25l"));
    assert!(s.contains("\x1b[?25h"));
}

#[test]
fn encoder_move_to() {
    let mut enc = AnsiEncoder::new();
    enc.move_to(0, 0);
    let out = enc.finish();
    assert!(out.windows(4).any(|w| w == b"1;1H"));
}

#[test]
fn encoder_move_to_offset() {
    let mut enc = AnsiEncoder::new();
    enc.move_to(9, 4);
    let out = enc.finish();
    let s = String::from_utf8_lossy(out);
    assert!(s.contains("5;10H"));
}

#[test]
fn encoder_push_char() {
    let mut enc = AnsiEncoder::new();
    enc.push_char('A');
    let out = enc.finish();
    assert_eq!(out, b"A");
}

#[test]
fn encoder_sgr_bold() {
    let mut enc = AnsiEncoder::new();
    enc.begin_sgr();
    enc.push_param(1);
    enc.end_sgr();
    enc.push_char('X');
    let out = enc.finish();
    let s = String::from_utf8_lossy(out);
    assert!(s.starts_with("\x1b["));
    assert!(s.contains('1'));
    assert!(s.ends_with('X'));
}

#[test]
fn encoder_full_cell() {
    use bettertui_engine::framebuffer::Cell;
    use bettertui_engine::framebuffer::CellAttributes;
    use bettertui_engine::tree::NamedColor;
    let mut enc = AnsiEncoder::new();
    let cell = Cell::new('Z')
        .with_fg(bettertui_engine::tree::Color::Named(NamedColor::Red))
        .with_bg(bettertui_engine::tree::Color::Named(NamedColor::Blue))
        .with_attrs(CellAttributes::BOLD);
    let mut last_fg = None;
    let mut last_bg = None;
    let mut last_attrs = None;
    enc.encode_cell(&cell, &mut last_fg, &mut last_bg, &mut last_attrs);
    let out = enc.finish();
    let s = String::from_utf8_lossy(out);
    assert!(s.contains("31"));
    assert!(s.contains("44"));
    assert!(s.contains("1"));
    assert!(s.ends_with('Z'));
}

#[test]
fn encoder_style_coalescing() {
    use bettertui_engine::framebuffer::Cell;
    use bettertui_engine::tree::NamedColor;
    let mut enc = AnsiEncoder::new();
    let cell = Cell::new('A').with_fg(bettertui_engine::tree::Color::Named(NamedColor::Red));
    let mut last_fg = Some(bettertui_engine::tree::Color::Named(NamedColor::Red));
    let mut last_bg = None;
    let mut last_attrs = None;
    enc.encode_cell(&cell, &mut last_fg, &mut last_bg, &mut last_attrs);
    let len1 = enc.finish().len();
    let mut enc2 = AnsiEncoder::new();
    let cell2 = Cell::new('B').with_fg(bettertui_engine::tree::Color::Named(NamedColor::Red));
    let mut last_fg2 = Some(bettertui_engine::tree::Color::Named(NamedColor::Red));
    enc2.encode_cell(&cell2, &mut last_fg2, &mut last_bg, &mut last_attrs);
    let len2 = enc2.finish().len();
    assert!(len1 > len2);
}

#[test]
fn encoder_hide_show_cursor() {
    let mut enc = AnsiEncoder::new();
    enc.hide_cursor();
    enc.show_cursor();
    let out = enc.finish();
    let s = String::from_utf8_lossy(out);
    assert!(s.contains("\x1b[?25l"));
    assert!(s.contains("\x1b[?25h"));
}

#[test]
fn encoder_reset_sgr() {
    let mut enc = AnsiEncoder::new();
    enc.reset_sgr();
    let out = enc.finish();
    assert_eq!(out, b"\x1b[0m");
}

#[test]
fn encoder_into_vec() {
    let mut enc = AnsiEncoder::new();
    enc.push_char('X');
    let v = enc.into_vec();
    assert_eq!(v, b"X");
}

#[test]
fn encoder_region() {
    use bettertui_engine::dirty_diff::DirtyRegion;
    use bettertui_engine::framebuffer::Cell;
    let mut enc = AnsiEncoder::new();
    let mut fb = bettertui_engine::framebuffer::FrameBuffer::new(5, 3);
    fb.set(1, 1, Cell::new('H'));
    let region = DirtyRegion::new(0, 0, 5, 3);
    enc.encode_region(&fb, &region);
    let out = enc.finish();
    let s = String::from_utf8_lossy(out);
    assert!(s.contains('H'));
}

fn test_commands() -> Vec<PaletteCommand> {
    vec![
        PaletteCommand::new("Save File").with_category("File"),
        PaletteCommand::new("Open File").with_category("File"),
        PaletteCommand::new("Find and Replace").with_category("Edit"),
        PaletteCommand::new("Toggle Terminal").with_category("View"),
    ]
}

#[test]
fn fuzzy_match_exact() {
    let (score, matches) = fuzzy_score("save", "Save File").unwrap();
    assert!(score > 0);
    assert_eq!(matches.len(), 4);
}

#[test]
fn fuzzy_match_partial() {
    let result = fuzzy_score("sv", "Save File");
    assert!(result.is_some());
}

#[test]
fn fuzzy_no_match() {
    let result = fuzzy_score("xyz", "Save File");
    assert!(result.is_none());
}

#[test]
fn fuzzy_empty_query() {
    let result = fuzzy_score("", "anything");
    assert!(result.is_some());
}

#[test]
fn palette_add_and_query() {
    let mut palette = CommandPalette::new();
    for cmd in test_commands() {
        palette.add(cmd);
    }
    assert_eq!(palette.command_count(), 4);
    palette.set_query("save");
    assert_eq!(palette.result_count(), 1);
    assert_eq!(palette.selected().unwrap().command.label, "Save File");
}

#[test]
fn palette_navigation() {
    let mut palette = CommandPalette::new();
    for cmd in test_commands() {
        palette.add(cmd);
    }
    palette.set_query("file");
    assert!(palette.result_count() >= 2);
    palette.select_next();
    assert!(palette.selected_index() > 0);
    palette.select_previous();
    assert_eq!(palette.selected_index(), 0);
}

#[test]
fn palette_first_last() {
    let mut palette = CommandPalette::new();
    for cmd in test_commands() {
        palette.add(cmd);
    }
    palette.set_query("");
    palette.select_last();
    assert_eq!(palette.selected_index(), palette.result_count() - 1);
    palette.select_first();
    assert_eq!(palette.selected_index(), 0);
}

#[test]
fn palette_empty_query_shows_all() {
    let mut palette = CommandPalette::new();
    for cmd in test_commands() {
        palette.add(cmd);
    }
    palette.set_query("");
    assert_eq!(palette.result_count(), 4);
}

#[test]
fn palette_disabled_commands_filtered() {
    let mut palette = CommandPalette::new();
    palette.add(PaletteCommand::new("Enabled"));
    palette.add(PaletteCommand::new("Disabled").with_category("test"));
    palette.commands_mut()[1].enabled = false;
    palette.set_query("");
    assert_eq!(palette.result_count(), 1);
}

#[test]
fn palette_clear() {
    let mut palette = CommandPalette::new();
    palette.add(PaletteCommand::new("test"));
    palette.clear();
    assert_eq!(palette.command_count(), 0);
    assert_eq!(palette.result_count(), 0);
}

#[test]
fn palette_shortcut() {
    let cmd = PaletteCommand::new("Save").with_shortcut("Ctrl+S");
    assert_eq!(cmd.shortcut.as_deref(), Some("Ctrl+S"));
}

#[test]
fn parser_new() {
    let parser = AnsiParser::new();
    assert!(parser.events().is_empty());
}

#[test]
fn parser_default() {
    let parser = AnsiParser::default();
    assert!(parser.events().is_empty());
}

#[test]
fn parser_feed_text() {
    let mut parser = AnsiParser::new();
    parser.feed(b"hello");
    assert_eq!(parser.events().len(), 5);
}

#[test]
fn parser_feed_escape() {
    let mut parser = AnsiParser::new();
    parser.feed(b"\x1b[A");
    assert_eq!(parser.events().len(), 1);
}

#[test]
fn parser_feed_csi() {
    let mut parser = AnsiParser::new();
    parser.feed(b"\x1b[1;2H");
    assert_eq!(parser.events().len(), 1);
}

#[test]
fn parser_reset() {
    let mut parser = AnsiParser::new();
    parser.feed(b"hello");
    parser.reset();
    assert!(parser.events().is_empty());
}

#[test]
fn parser_poll_event() {
    let mut parser = AnsiParser::new();
    parser.feed(b"ab");
    assert!(parser.poll_event().is_some());
    assert!(parser.poll_event().is_some());
    assert!(parser.poll_event().is_none());
}

#[test]
fn parser_state_ground() {
    assert_ne!(ParserState::Ground, ParserState::Escape);
}

#[test]
fn parser_event_char() {
    let event = ParserEvent::Char(b'A');
    assert!(event.is_printable());
    assert!(!event.is_cursor_movement());
}

#[test]
fn parser_event_csi() {
    let event = ParserEvent::Csi(CsiCommand::CursorMovement(CursorMovement::Up(1)));
    assert!(event.is_cursor_movement());
}

#[test]
fn parser_event_sgr() {
    let event = ParserEvent::Csi(CsiCommand::Sgr(vec![SgrAttribute::Bold]));
    assert!(event.is_sgr());
}

#[test]
fn csi_cursor_up() {
    let cmd = CsiCommand::parse(b'A', &[5], &[]);
    assert_eq!(cmd, Some(CsiCommand::CursorMovement(CursorMovement::Up(5))));
}

#[test]
fn csi_cursor_down() {
    let cmd = CsiCommand::parse(b'B', &[3], &[]);
    assert_eq!(cmd, Some(CsiCommand::CursorMovement(CursorMovement::Down(3))));
}

#[test]
fn csi_cursor_position() {
    let cmd = CsiCommand::parse(b'H', &[10, 20], &[]);
    assert_eq!(cmd, Some(CsiCommand::CursorMovement(CursorMovement::Position(10, 20))));
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
    assert!(matches!(cmd, Some(CsiCommand::Sgr(attrs)) if attrs.contains(&SgrAttribute::Reset)));
}

#[test]
fn csi_device_status() {
    let cmd = CsiCommand::parse(b'n', &[6], &[]);
    assert_eq!(cmd, Some(CsiCommand::DeviceStatus(DeviceStatus::ReportCursorPosition)));
}

#[test]
fn sgr_state_new() {
    let state = SgrState::new();
    assert!(state.is_plain());
}

#[test]
fn sgr_state_default() {
    let state = SgrState::default();
    assert!(state.is_plain());
}

#[test]
fn sgr_state_apply_bold() {
    let mut state = SgrState::new();
    state.apply(&[SgrAttribute::Bold]);
    assert!(state.bold);
    assert!(!state.is_plain());
}

#[test]
fn sgr_state_apply_reset() {
    let mut state = SgrState::new();
    state.apply(&[SgrAttribute::Bold, SgrAttribute::Italic]);
    assert!(!state.is_plain());
    state.apply(&[SgrAttribute::Reset]);
    assert!(state.is_plain());
}

#[test]
fn sgr_state_apply_foreground() {
    let mut state = SgrState::new();
    state.apply(&[SgrAttribute::Foreground(ForegroundColor::Red)]);
    assert_eq!(state.foreground, ForegroundColor::Red);
}

#[test]
fn sgr_state_apply_background() {
    let mut state = SgrState::new();
    state.apply(&[SgrAttribute::Background(BackgroundColor::Blue)]);
    assert_eq!(state.background, BackgroundColor::Blue);
}

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
        Some(OscCommand::SetClipboard(ClipboardData { selection: ClipboardSelection::Clipboard, .. }))
    ));
}

#[test]
fn osc_set_hyperlink() {
    let cmd = OscCommand::parse(b"8;;https://example.com");
    assert_eq!(cmd, Some(OscCommand::SetHyperlink(Hyperlink { id: None, uri: "https://example.com".to_string() })));
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

#[test]
fn osc52_set_sequence_encodes_base64() {
    let seq = ClipboardData::set_sequence(ClipboardSelection::Clipboard, "Hello");
    let s = String::from_utf8_lossy(&seq);
    // "Hello" base64 == "SGVsbG8="
    assert_eq!(s, "\x1b]52;c;SGVsbG8=\x1b\\");
}

#[test]
fn osc52_query_sequence() {
    let seq = ClipboardData::query_sequence(ClipboardSelection::Primary);
    let s = String::from_utf8_lossy(&seq);
    assert_eq!(s, "\x1b]52;p;?\x1b\\");
}

#[test]
fn osc52_selection_params() {
    assert_eq!(ClipboardSelection::Clipboard.param(), 'c');
    assert_eq!(ClipboardSelection::Primary.param(), 'p');
    assert_eq!(ClipboardSelection::Secondary.param(), 's');
    assert_eq!(ClipboardSelection::Tertiary.param(), 'q');
}

#[test]
fn osc52_roundtrip_set_then_parse() {
    let seq = ClipboardData::set_sequence(ClipboardSelection::Clipboard, "round trip");
    // Strip the ESC ] prefix and ESC \ suffix to feed the OSC payload to parse().
    let s = String::from_utf8_lossy(&seq);
    let payload = s.trim_start_matches("\x1b]").trim_end_matches("\x1b\\");
    let cmd = OscCommand::parse(payload.as_bytes());
    match cmd {
        Some(OscCommand::SetClipboard(data)) => {
            assert_eq!(data.decoded(), Some("round trip".to_string()));
            assert!(!data.is_query());
        }
        other => panic!("expected SetClipboard, got {other:?}"),
    }
}

#[test]
fn osc52_query_marker_decodes_to_none() {
    let cmd = OscCommand::parse(b"52;c;?");
    match cmd {
        Some(OscCommand::SetClipboard(data)) => {
            assert!(data.is_query());
            assert_eq!(data.decoded(), None);
        }
        other => panic!("expected SetClipboard query, got {other:?}"),
    }
}

#[test]
fn osc52_invalid_base64_decodes_to_none() {
    let data = ClipboardData { data: "not!base64!".to_string(), selection: ClipboardSelection::Clipboard };
    assert_eq!(data.decoded(), None);
}

#[test]
fn kitty_csiu_colon_event_type_release() {
    // CSI 97 ; 1 : 3 u  ==  'a' with no mods, event_type=3 (Release).
    let mut parser = AnsiParser::new();
    parser.feed(b"\x1b[97;1:3u");
    // Drain parser events; the CSI command is surfaced via ParserEvent::Csi.
    let mut found = None;
    while let Some(ev) = parser.poll_event() {
        if let ParserEvent::Csi(CsiCommand::KittyKeyEvent { keycode, modifiers, event_type, .. }) = ev {
            found = Some((keycode, modifiers, event_type));
        }
    }
    assert_eq!(found, Some((97, 1, KittyEventType::Release)), "colon sub-param must decode event_type");
}

// ── OSC 4 palette ────────────────────────────────────────────────────────────

#[test]
fn osc4_parse_set_palette_color() {
    let cmd = OscCommand::parse(b"4;1;rgb:ff/00/00");
    match cmd {
        Some(OscCommand::SetPaletteColor { index, ref spec }) => {
            assert_eq!(index, 1);
            assert_eq!(spec, "rgb:ff/00/00");
        }
        other => panic!("expected SetPaletteColor, got {other:?}"),
    }
    assert_eq!(cmd.unwrap().palette_rgb(), Some((255, 0, 0)));
}

#[test]
fn osc4_palette_rgb_16bit_components() {
    let cmd = OscCommand::parse(b"4;2;rgb:ffff/8000/0000").unwrap();
    assert_eq!(cmd.palette_rgb(), Some((255, 128, 0)));
}

#[test]
fn osc4_query_marker_has_no_rgb() {
    let cmd = OscCommand::parse(b"4;5;?").unwrap();
    assert!(matches!(cmd, OscCommand::SetPaletteColor { index: 5, .. }));
    assert_eq!(cmd.palette_rgb(), None);
}

#[test]
fn osc4_query_and_set_builders() {
    assert_eq!(String::from_utf8_lossy(&OscCommand::palette_query(3)), "\x1b]4;3;?\x1b\\");
    let set = OscCommand::palette_set(4, 0x12, 0x34, 0x56);
    assert_eq!(String::from_utf8_lossy(&set), "\x1b]4;4;rgb:1212/3434/5656\x1b\\");
    // Round-trip: the set spec parses back to the same rgb.
    let s = String::from_utf8_lossy(&set);
    let payload = s.trim_start_matches("\x1b]").trim_end_matches("\x1b\\");
    assert_eq!(OscCommand::parse(payload.as_bytes()).unwrap().palette_rgb(), Some((0x12, 0x34, 0x56)));
}

// ── CPR (Cursor Position Report) ─────────────────────────────────────────────

#[test]
fn csi_cursor_position_report_parses() {
    let cmd = CsiCommand::parse(b'R', &[12, 34], &[]);
    assert_eq!(cmd, Some(CsiCommand::CursorPositionReport { row: 12, col: 34 }));
}
