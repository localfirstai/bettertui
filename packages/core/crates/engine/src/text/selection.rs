#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SelectionRange {
    pub start: usize,
    pub end: usize,
}

impl SelectionRange {
    pub fn new(start: usize, end: usize) -> Self {
        Self {
            start: start.min(end),
            end: start.max(end),
        }
    }

    pub fn contains(&self, pos: usize) -> bool {
        pos >= self.start && pos < self.end
    }

    pub fn len(&self) -> usize {
        self.end - self.start
    }

    pub fn is_empty(&self) -> bool {
        self.start == self.end
    }
}

#[derive(Debug, Clone)]
pub struct Selection {
    active: Option<SelectionRange>,
    anchor: Option<usize>,
    selecting: bool,
}

impl Default for Selection {
    fn default() -> Self {
        Self::new()
    }
}

impl Selection {
    pub fn new() -> Self {
        Self {
            active: None,
            anchor: None,
            selecting: false,
        }
    }

    pub fn start(&mut self, anchor: usize) {
        self.anchor = Some(anchor);
        self.selecting = true;
        self.active = Some(SelectionRange::new(anchor, anchor));
    }

    pub fn update(&mut self, current: usize) {
        if let Some(anchor) = self.anchor {
            self.active = Some(SelectionRange::new(anchor, current));
        }
    }

    pub fn end(&mut self) {
        self.selecting = false;
        if let Some(range) = self.active
            && range.is_empty()
        {
            self.active = None;
            self.anchor = None;
        }
    }

    pub fn clear(&mut self) {
        self.active = None;
        self.anchor = None;
        self.selecting = false;
    }

    pub fn is_selecting(&self) -> bool {
        self.selecting
    }

    pub fn active(&self) -> Option<SelectionRange> {
        self.active
    }

    pub fn has_selection(&self) -> bool {
        self.active.is_some() && !self.active.unwrap().is_empty()
    }

    pub fn select_all(&mut self, len: usize) {
        self.active = Some(SelectionRange::new(0, len));
        self.anchor = Some(0);
    }

    pub fn select_line(&mut self, start: usize, end: usize) {
        self.active = Some(SelectionRange::new(start, end));
        self.anchor = Some(start);
    }

    pub fn selected_text(&self, text: &str) -> Option<String> {
        self.active
            .map(|range| text[range.start..range.end].to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn selection_range_new() {
        let range = SelectionRange::new(5, 10);
        assert_eq!(range.start, 5);
        assert_eq!(range.end, 10);
    }

    #[test]
    fn selection_range_new_reversed() {
        let range = SelectionRange::new(10, 5);
        assert_eq!(range.start, 5);
        assert_eq!(range.end, 10);
    }

    #[test]
    fn selection_range_contains() {
        let range = SelectionRange::new(5, 10);
        assert!(range.contains(7));
        assert!(!range.contains(4));
        assert!(!range.contains(10));
    }

    #[test]
    fn selection_range_len() {
        let range = SelectionRange::new(5, 10);
        assert_eq!(range.len(), 5);
    }

    #[test]
    fn selection_range_is_empty() {
        let range = SelectionRange::new(5, 5);
        assert!(range.is_empty());
    }

    #[test]
    fn selection_new() {
        let selection = Selection::new();
        assert!(!selection.has_selection());
    }

    #[test]
    fn selection_default() {
        let selection = Selection::default();
        assert!(!selection.has_selection());
    }

    #[test]
    fn selection_start_update_end() {
        let mut selection = Selection::new();
        selection.start(5);
        assert!(selection.is_selecting());
        selection.update(10);
        assert!(selection.has_selection());
        selection.end();
        assert!(!selection.is_selecting());
        assert!(selection.has_selection());
    }

    #[test]
    fn selection_clear() {
        let mut selection = Selection::new();
        selection.start(5);
        selection.update(10);
        selection.clear();
        assert!(!selection.has_selection());
    }

    #[test]
    fn selection_select_all() {
        let mut selection = Selection::new();
        selection.select_all(100);
        assert!(selection.has_selection());
        assert_eq!(selection.active().unwrap().start, 0);
        assert_eq!(selection.active().unwrap().end, 100);
    }

    #[test]
    fn selection_selected_text() {
        let mut selection = Selection::new();
        selection.start(0);
        selection.update(5);
        assert_eq!(
            selection.selected_text("hello world"),
            Some("hello".to_string())
        );
    }
}
