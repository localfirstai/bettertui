/// Type aliases for widget callbacks.
pub type AsyncCallback = Box<dyn Fn() + Send + Sync>;
pub type ChangeCallback = Box<dyn Fn(&str) + Send + Sync>;
pub type SubmitCallback = Box<dyn Fn(&str) + Send + Sync>;
pub type IndexChangeCallback = Box<dyn Fn(usize) + Send + Sync>;
