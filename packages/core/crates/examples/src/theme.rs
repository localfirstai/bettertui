#[allow(dead_code)]
pub struct Theme {
    pub title_color: &'static str,
    pub border_color: &'static str,
    pub focused_border_color: &'static str,
    pub input_text_color: &'static str,
    pub input_placeholder_color: &'static str,
    pub input_cursor_color: &'static str,
    pub select_text_color: &'static str,
    pub select_selected_bg: &'static str,
    pub select_selected_text_color: &'static str,
    pub select_description_color: &'static str,
    pub select_category_color: &'static str,
    pub instructions_color: &'static str,
}

pub const DARK: Theme = Theme {
    title_color: "\x1b[38;2;240;248;255m",
    border_color: "\x1b[38;2;71;85;105m",
    focused_border_color: "\x1b[38;2;96;165;250m",
    input_text_color: "\x1b[38;2;226;232;240m",
    input_placeholder_color: "\x1b[38;2;148;163;184m",
    input_cursor_color: "\x1b[38;2;96;165;250m",
    select_text_color: "\x1b[38;2;226;232;240m",
    select_selected_bg: "\x1b[48;2;30;58;95m",
    select_selected_text_color: "\x1b[38;2;56;189;248m",
    select_description_color: "\x1b[38;2;100;116;139m",
    select_category_color: "\x1b[38;2;148;163;184m\x1b[1m",
    instructions_color: "\x1b[38;2;148;163;184m",
};

#[allow(dead_code)]
pub const LIGHT: Theme = Theme {
    title_color: "\x1b[38;2;15;23;42m",
    border_color: "\x1b[38;2;203;213;225m",
    focused_border_color: "\x1b[38;2;37;99;235m",
    input_text_color: "\x1b[38;2;15;23;42m",
    input_placeholder_color: "\x1b[38;2;100;116;139m",
    input_cursor_color: "\x1b[38;2;37;99;235m",
    select_text_color: "\x1b[38;2;15;23;42m",
    select_selected_bg: "\x1b[48;2;219;234;254m",
    select_selected_text_color: "\x1b[38;2;29;78;216m",
    select_description_color: "\x1b[38;2;71;85;105m",
    select_category_color: "\x1b[38;2;71;85;105m\x1b[1m",
    instructions_color: "\x1b[38;2;71;85;105m",
};
