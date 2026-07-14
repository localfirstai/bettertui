use super::ast::{InlineNode, ListItem, MarkdownNode, TaskItem};

pub struct Parser<'a> {
    lines: Vec<&'a str>,
    pos: usize,
}

impl<'a> Parser<'a> {
    pub fn new(input: &'a str) -> Self {
        let lines = input.lines().collect();
        Self { lines, pos: 0 }
    }

    pub fn parse(&mut self) -> Vec<MarkdownNode> {
        let mut nodes = Vec::new();
        while self.pos < self.lines.len() {
            if let Some(node) = self.parse_block() {
                nodes.push(node);
            }
        }
        nodes
    }

    fn current_line(&self) -> Option<&'a str> {
        self.lines.get(self.pos).copied()
    }

    fn advance(&mut self) {
        self.pos += 1;
    }

    fn skip_blank_lines(&mut self) {
        while self.pos < self.lines.len() && self.current_line().unwrap_or("").trim().is_empty() {
            self.advance();
        }
    }

    fn parse_block(&mut self) -> Option<MarkdownNode> {
        self.skip_blank_lines();
        let line = self.current_line()?;
        let trimmed = line.trim();

        if trimmed.is_empty() {
            self.advance();
            return None;
        }

        if trimmed.starts_with("# ") {
            self.parse_heading(1)
        } else if trimmed.starts_with("## ") {
            self.parse_heading(2)
        } else if trimmed.starts_with("### ") {
            self.parse_heading(3)
        } else if trimmed.starts_with("#### ") {
            self.parse_heading(4)
        } else if trimmed.starts_with("##### ") {
            self.parse_heading(5)
        } else if trimmed.starts_with("###### ") {
            self.parse_heading(6)
        } else if trimmed.starts_with("```") {
            self.parse_code_block()
        } else if trimmed.starts_with("> ") {
            self.parse_blockquote()
        } else if trimmed.starts_with("- [") || trimmed.starts_with("* [") {
            self.parse_task_list()
        } else if trimmed.starts_with("- ") || trimmed.starts_with("* ") {
            self.parse_list(false)
        } else if trimmed.chars().next().is_some_and(|c| c.is_ascii_digit())
            && trimmed.contains(". ")
        {
            self.parse_list(true)
        } else if trimmed == "---" || trimmed == "***" || trimmed == "___" {
            self.advance();
            Some(MarkdownNode::HorizontalRule)
        } else if trimmed.starts_with("|") {
            self.parse_table()
        } else {
            self.parse_paragraph()
        }
    }

    fn parse_heading(&mut self, level: u8) -> Option<MarkdownNode> {
        let line = self.current_line()?;
        let content = line.trim_start_matches('#');
        let content = content.trim();
        self.advance();
        Some(MarkdownNode::Heading {
            level,
            content: parse_inline(content),
        })
    }

    fn parse_paragraph(&mut self) -> Option<MarkdownNode> {
        let mut text_lines = Vec::new();
        while self.pos < self.lines.len() {
            let line = self.current_line()?;
            let trimmed = line.trim();
            if trimmed.is_empty() || self.is_block_start(trimmed) {
                break;
            }
            text_lines.push(trimmed);
            self.advance();
        }
        let text = text_lines.join(" ");
        Some(MarkdownNode::Paragraph(parse_inline(&text)))
    }

    fn is_block_start(&self, trimmed: &str) -> bool {
        trimmed.starts_with("# ")
            || trimmed.starts_with("## ")
            || trimmed.starts_with("### ")
            || trimmed.starts_with("#### ")
            || trimmed.starts_with("##### ")
            || trimmed.starts_with("###### ")
            || trimmed.starts_with("```")
            || trimmed.starts_with("> ")
            || trimmed.starts_with("- [")
            || trimmed.starts_with("* [")
            || trimmed.starts_with("- ")
            || trimmed.starts_with("* ")
            || trimmed == "---"
            || trimmed == "***"
            || trimmed == "___"
            || trimmed.starts_with("|")
    }

    fn parse_code_block(&mut self) -> Option<MarkdownNode> {
        let line = self.current_line()?;
        let language = if line.len() > 3 {
            let lang = line[3..].trim();
            if lang.is_empty() {
                None
            } else {
                Some(Box::from(lang))
            }
        } else {
            None
        };
        self.advance();

        let mut code_lines = Vec::new();
        while self.pos < self.lines.len() {
            let line = self.current_line()?;
            if line.trim().starts_with("```") {
                self.advance();
                break;
            }
            code_lines.push(line);
            self.advance();
        }

        let code = code_lines.join("\n");
        Some(MarkdownNode::CodeBlock {
            language,
            code: Box::from(code),
        })
    }

    fn parse_blockquote(&mut self) -> Option<MarkdownNode> {
        let mut content_lines = Vec::new();
        while self.pos < self.lines.len() {
            let line = self.current_line()?;
            let trimmed = line.trim();
            if trimmed.starts_with("> ") {
                content_lines.push(trimmed.strip_prefix("> ").unwrap_or(trimmed));
                self.advance();
            } else {
                break;
            }
        }
        let joined = content_lines.join("\n");
        let mut inner_parser = Parser::new(&joined);
        Some(MarkdownNode::Blockquote(inner_parser.parse()))
    }

    fn parse_list(&mut self, ordered: bool) -> Option<MarkdownNode> {
        let mut items = Vec::new();
        while self.pos < self.lines.len() {
            let line = self.current_line()?;
            let trimmed = line.trim();

            let item_start = if ordered {
                trimmed.find(". ").map(|i| i + 2)
            } else if trimmed.starts_with("- ") || trimmed.starts_with("* ") {
                Some(2)
            } else {
                None
            };

            if let Some(start) = item_start {
                let content = &trimmed[start..];
                let children = Vec::new();
                let mut content_nodes = parse_inline(content);
                self.advance();

                while self.pos < self.lines.len() {
                    let next = self.current_line()?;
                    let next_trimmed = next.trim();
                    if next_trimmed.starts_with("  ") && !next_trimmed.trim().is_empty() {
                        let child_text = next_trimmed.trim_start();
                        content_nodes.extend(parse_inline(child_text));
                        self.advance();
                    } else {
                        break;
                    }
                }

                items.push(ListItem {
                    content: content_nodes,
                    children,
                });
            } else {
                break;
            }
        }
        Some(MarkdownNode::List { ordered, items })
    }

    fn parse_task_list(&mut self) -> Option<MarkdownNode> {
        let mut items = Vec::new();
        while self.pos < self.lines.len() {
            let line = self.current_line()?;
            let trimmed = line.trim();

            let checked = if trimmed.starts_with("- [x]") || trimmed.starts_with("- [X]") {
                true
            } else if trimmed.starts_with("- [ ]") {
                false
            } else if trimmed.starts_with("* [x]") || trimmed.starts_with("* [X]") {
                true
            } else if trimmed.starts_with("* [ ]") {
                false
            } else {
                break;
            };

            let content_start = if trimmed.starts_with("- [") || trimmed.starts_with("* [") {
                let bracket_pos = trimmed.find(']').unwrap_or(2) + 1;
                trimmed[bracket_pos..].trim()
            } else {
                break;
            };

            items.push(TaskItem {
                checked,
                content: parse_inline(content_start),
            });
            self.advance();
        }
        Some(MarkdownNode::TaskList(items))
    }

    fn parse_table(&mut self) -> Option<MarkdownNode> {
        let header_line = self.current_line()?;
        let headers = parse_table_row(header_line);
        self.advance();

        if self.pos < self.lines.len() && self.current_line().unwrap_or("").contains("---") {
            self.advance();
        }

        let mut rows = Vec::new();
        while self.pos < self.lines.len() {
            let line = self.current_line()?;
            let trimmed = line.trim();
            if trimmed.is_empty() || !trimmed.starts_with("|") {
                break;
            }
            let cells = parse_table_row(trimmed);
            rows.push(cells);
            self.advance();
        }

        Some(MarkdownNode::Table { headers, rows })
    }
}

fn parse_table_row(line: &str) -> Vec<Vec<InlineNode>> {
    line.trim()
        .trim_start_matches('|')
        .trim_end_matches('|')
        .split('|')
        .map(|cell| parse_inline(cell.trim()))
        .collect()
}

pub fn parse_inline(text: &str) -> Vec<InlineNode> {
    let mut nodes = Vec::new();
    let mut chars = text.chars().peekable();
    let mut current_text = String::new();

    while let Some(ch) = chars.next() {
        match ch {
            '*' | '_' => {
                if !current_text.is_empty() {
                    nodes.push(InlineNode::Text(Box::from(std::mem::take(
                        &mut current_text,
                    ))));
                }
                let next = chars.peek().copied();
                if next == Some(ch) {
                    chars.next();
                    let bold_text = collect_until(&mut chars, &[ch, ch]);
                    nodes.push(InlineNode::Bold(parse_inline(&bold_text)));
                } else {
                    let italic_text = collect_until(&mut chars, &[ch]);
                    nodes.push(InlineNode::Italic(parse_inline(&italic_text)));
                }
            }
            '~' => {
                if !current_text.is_empty() {
                    nodes.push(InlineNode::Text(Box::from(std::mem::take(
                        &mut current_text,
                    ))));
                }
                if chars.peek() == Some(&'~') {
                    chars.next();
                    let strike_text = collect_until(&mut chars, &['~', '~']);
                    nodes.push(InlineNode::Strikethrough(parse_inline(&strike_text)));
                } else {
                    current_text.push('~');
                }
            }
            '`' => {
                if !current_text.is_empty() {
                    nodes.push(InlineNode::Text(Box::from(std::mem::take(
                        &mut current_text,
                    ))));
                }
                let code_text = collect_until(&mut chars, &['`']);
                nodes.push(InlineNode::Code(Box::from(code_text)));
            }
            '[' => {
                if !current_text.is_empty() {
                    nodes.push(InlineNode::Text(Box::from(std::mem::take(
                        &mut current_text,
                    ))));
                }
                let link_text = collect_until(&mut chars, &[']']);
                if chars.peek() == Some(&'(') {
                    chars.next();
                    let url = collect_until(&mut chars, &[')']);
                    nodes.push(InlineNode::Link {
                        text: Box::from(link_text),
                        url: Box::from(url),
                    });
                } else {
                    current_text.push('[');
                    current_text.push_str(&link_text);
                }
            }
            '\\' => {
                if let Some(escaped) = chars.next() {
                    current_text.push(escaped);
                }
            }
            _ => {
                current_text.push(ch);
            }
        }
    }

    if !current_text.is_empty() {
        nodes.push(InlineNode::Text(Box::from(current_text)));
    }

    nodes
}

fn collect_until(chars: &mut std::iter::Peekable<std::str::Chars>, stop: &[char]) -> String {
    let mut result = String::new();
    let stop_len = stop.len();
    let mut matched = 0;

    for ch in chars.by_ref() {
        if matched < stop_len && ch == stop[matched] {
            matched += 1;
            if matched == stop_len {
                return result;
            }
        } else {
            for &c in stop.iter().take(matched) {
                result.push(c);
            }
            matched = 0;
            result.push(ch);
        }
    }

    for &c in stop.iter().take(matched) {
        result.push(c);
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_heading() {
        let mut parser = Parser::new("# Hello");
        let nodes = parser.parse();
        assert_eq!(nodes.len(), 1);
        match &nodes[0] {
            MarkdownNode::Heading { level, content } => {
                assert_eq!(*level, 1);
                assert_eq!(content.len(), 1);
                assert_eq!(content[0].text_content(), "Hello");
            }
            _ => panic!("Expected heading"),
        }
    }

    #[test]
    fn parse_paragraph() {
        let mut parser = Parser::new("Hello world");
        let nodes = parser.parse();
        assert_eq!(nodes.len(), 1);
        match &nodes[0] {
            MarkdownNode::Paragraph(content) => {
                assert_eq!(content.len(), 1);
                assert_eq!(content[0].text_content(), "Hello world");
            }
            _ => panic!("Expected paragraph"),
        }
    }

    #[test]
    fn parse_code_block() {
        let input = "```rust\nfn main() {}\n```";
        let mut parser = Parser::new(input);
        let nodes = parser.parse();
        assert_eq!(nodes.len(), 1);
        match &nodes[0] {
            MarkdownNode::CodeBlock { language, code } => {
                assert_eq!(language.as_deref(), Some("rust"));
                assert_eq!(code.as_ref(), "fn main() {}");
            }
            _ => panic!("Expected code block"),
        }
    }

    #[test]
    fn parse_inline_bold() {
        let nodes = parse_inline("**bold**");
        assert_eq!(nodes.len(), 1);
        assert!(matches!(nodes[0], InlineNode::Bold(_)));
    }

    #[test]
    fn parse_inline_italic() {
        let nodes = parse_inline("*italic*");
        assert_eq!(nodes.len(), 1);
        assert!(matches!(nodes[0], InlineNode::Italic(_)));
    }

    #[test]
    fn parse_inline_code() {
        let nodes = parse_inline("`code`");
        assert_eq!(nodes.len(), 1);
        assert!(matches!(nodes[0], InlineNode::Code(_)));
    }

    #[test]
    fn parse_inline_link() {
        let nodes = parse_inline("[text](url)");
        assert_eq!(nodes.len(), 1);
        match &nodes[0] {
            InlineNode::Link { text, url } => {
                assert_eq!(text.as_ref(), "text");
                assert_eq!(url.as_ref(), "url");
            }
            _ => panic!("Expected link"),
        }
    }

    #[test]
    fn parse_inline_strikethrough() {
        let nodes = parse_inline("~~strike~~");
        assert_eq!(nodes.len(), 1);
        assert!(matches!(nodes[0], InlineNode::Strikethrough(_)));
    }

    #[test]
    fn parse_list() {
        let input = "- item1\n- item2\n- item3";
        let mut parser = Parser::new(input);
        let nodes = parser.parse();
        assert_eq!(nodes.len(), 1);
        match &nodes[0] {
            MarkdownNode::List { ordered, items } => {
                assert!(!ordered);
                assert_eq!(items.len(), 3);
            }
            _ => panic!("Expected list"),
        }
    }

    #[test]
    fn parse_task_list() {
        let input = "- [x] done\n- [ ] not done";
        let mut parser = Parser::new(input);
        let nodes = parser.parse();
        assert_eq!(nodes.len(), 1);
        match &nodes[0] {
            MarkdownNode::TaskList(items) => {
                assert_eq!(items.len(), 2);
                assert!(items[0].checked);
                assert!(!items[1].checked);
            }
            _ => panic!("Expected task list"),
        }
    }

    #[test]
    fn parse_horizontal_rule() {
        let mut parser = Parser::new("---");
        let nodes = parser.parse();
        assert_eq!(nodes.len(), 1);
        assert!(matches!(nodes[0], MarkdownNode::HorizontalRule));
    }

    #[test]
    fn parse_blockquote() {
        let input = "> quoted text\n> more quoted";
        let mut parser = Parser::new(input);
        let nodes = parser.parse();
        assert_eq!(nodes.len(), 1);
        assert!(matches!(nodes[0], MarkdownNode::Blockquote(_)));
    }

    #[test]
    fn parse_table() {
        let input = "| Header |\n|--------|\n| Cell |";
        let mut parser = Parser::new(input);
        let nodes = parser.parse();
        assert_eq!(nodes.len(), 1);
        assert!(matches!(nodes[0], MarkdownNode::Table { .. }));
    }

    #[test]
    fn parse_mixed() {
        let input = "# Title\n\nHello world\n\n- item1\n- item2";
        let mut parser = Parser::new(input);
        let nodes = parser.parse();
        assert_eq!(nodes.len(), 3);
    }
}
