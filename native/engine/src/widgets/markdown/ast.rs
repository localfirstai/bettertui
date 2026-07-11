#[derive(Debug, Clone, PartialEq)]
pub enum MarkdownNode {
    Heading {
        level: u8,
        content: Vec<InlineNode>,
    },
    Paragraph(Vec<InlineNode>),
    List {
        ordered: bool,
        items: Vec<ListItem>,
    },
    Blockquote(Vec<MarkdownNode>),
    CodeBlock {
        language: Option<Box<str>>,
        code: Box<str>,
    },
    Table {
        headers: Vec<Vec<InlineNode>>,
        rows: Vec<Vec<Vec<InlineNode>>>,
    },
    HorizontalRule,
    TaskList(Vec<TaskItem>),
}

#[derive(Debug, Clone, PartialEq)]
pub struct ListItem {
    pub content: Vec<InlineNode>,
    pub children: Vec<MarkdownNode>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TaskItem {
    pub checked: bool,
    pub content: Vec<InlineNode>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum InlineNode {
    Text(Box<str>),
    Code(Box<str>),
    Bold(Vec<InlineNode>),
    Italic(Vec<InlineNode>),
    BoldItalic(Vec<InlineNode>),
    Link { text: Box<str>, url: Box<str> },
    Strikethrough(Vec<InlineNode>),
}

impl InlineNode {
    pub fn text_content(&self) -> &str {
        match self {
            InlineNode::Text(s) => s,
            InlineNode::Code(s) => s,
            InlineNode::Bold(children)
            | InlineNode::Italic(children)
            | InlineNode::BoldItalic(children)
            | InlineNode::Strikethrough(children) => {
                children.first().map(|c| c.text_content()).unwrap_or("")
            }
            InlineNode::Link { text, .. } => text,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn inline_node_text_content() {
        let node = InlineNode::Text("hello".into());
        assert_eq!(node.text_content(), "hello");
    }

    #[test]
    fn inline_node_bold_text_content() {
        let node = InlineNode::Bold(vec![InlineNode::Text("bold".into())]);
        assert_eq!(node.text_content(), "bold");
    }

    #[test]
    fn inline_node_link_text_content() {
        let node = InlineNode::Link {
            text: "click".into(),
            url: "https://example.com".into(),
        };
        assert_eq!(node.text_content(), "click");
    }

    #[test]
    fn markdown_node_variants() {
        let heading = MarkdownNode::Heading {
            level: 1,
            content: vec![InlineNode::Text("Title".into())],
        };
        assert!(matches!(heading, MarkdownNode::Heading { level: 1, .. }));

        let hr = MarkdownNode::HorizontalRule;
        assert!(matches!(hr, MarkdownNode::HorizontalRule));
    }

    #[test]
    fn list_item_creation() {
        let item = ListItem {
            content: vec![InlineNode::Text("item".into())],
            children: vec![],
        };
        assert_eq!(item.content.len(), 1);
        assert!(item.children.is_empty());
    }

    #[test]
    fn task_item_creation() {
        let item = TaskItem {
            checked: true,
            content: vec![InlineNode::Text("done".into())],
        };
        assert!(item.checked);
    }
}
