//! Text widgets: text display, input, code, and rich text rendering.

pub mod badge_widget;
pub mod code_widget;
pub mod heading_widget;
pub mod input_widget;
pub mod label_widget;
pub mod markdown;
pub mod prompt_composer;
pub mod text_widget;
pub mod textarea_widget;

pub use badge_widget::{BadgeVariant, BadgeWidget};
pub use code_widget::CodeWidget;
pub use heading_widget::{HeadingLevel, HeadingWidget};
pub use input_widget::InputWidget;
pub use label_widget::LabelWidget;
pub use markdown::{InlineNode, MarkdownNode, MarkdownRenderer, Parser as MarkdownParser};
pub use prompt_composer::{ComposerState, PromptComposer};
pub use text_widget::TextWidget;
pub use textarea_widget::TextareaWidget;
