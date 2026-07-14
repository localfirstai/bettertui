use std::collections::HashMap;

use super::Widget;

pub type WidgetFactory = Box<dyn Fn() -> Box<dyn Widget> + Send + Sync>;

pub struct WidgetRegistry {
    factories: HashMap<Box<str>, WidgetFactory>,
}

impl Default for WidgetRegistry {
    fn default() -> Self {
        Self::new()
    }
}

impl WidgetRegistry {
    pub fn new() -> Self {
        Self {
            factories: HashMap::new(),
        }
    }

    pub fn register(
        &mut self,
        kind: &'static str,
        factory: impl Fn() -> Box<dyn Widget> + Send + Sync + 'static,
    ) {
        self.factories.insert(Box::from(kind), Box::new(factory));
    }

    pub fn create(&self, kind: &str) -> Option<Box<dyn Widget>> {
        self.factories.get(kind).map(|f| f())
    }

    pub fn has(&self, kind: &str) -> bool {
        self.factories.contains_key(kind)
    }

    pub fn kinds(&self) -> Vec<&str> {
        self.factories.keys().map(|s| s.as_ref()).collect()
    }

    pub fn len(&self) -> usize {
        self.factories.len()
    }

    pub fn is_empty(&self) -> bool {
        self.factories.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct DummyWidget;

    impl crate::Widget for DummyWidget {
        fn kind(&self) -> &'static str {
            "Dummy"
        }

        fn create(&self, _ctx: &mut crate::context::WidgetContext) -> crate::WidgetId {
            crate::WidgetId(bettertui_engine::tree::NodeId::default())
        }
    }

    #[test]
    fn registry_new() {
        let registry = WidgetRegistry::new();
        assert!(registry.is_empty());
    }

    #[test]
    fn registry_register_and_create() {
        let mut registry = WidgetRegistry::new();
        registry.register("Dummy", || Box::new(DummyWidget));
        assert!(registry.has("Dummy"));
        assert_eq!(registry.len(), 1);

        let widget = registry.create("Dummy");
        assert!(widget.is_some());
        assert_eq!(widget.expect("Node missing from arena").kind(), "Dummy");
    }

    #[test]
    fn registry_create_unknown() {
        let registry = WidgetRegistry::new();
        assert!(registry.create("Unknown").is_none());
    }

    #[test]
    fn registry_kinds() {
        let mut registry = WidgetRegistry::new();
        registry.register("A", || Box::new(DummyWidget));
        registry.register("B", || Box::new(DummyWidget));

        let mut kinds = registry.kinds();
        kinds.sort();
        assert_eq!(kinds, vec!["A", "B"]);
    }
}
