pub mod ui_builder;

pub mod style;

pub mod widgets;

pub mod style_commands;

/// Everything a UI module usually needs: the builder, `.style()` and the style setters.
pub mod prelude {
    pub use crate::style::{UiStyle, UiStyleExt};
    pub use crate::style_commands::{UiNodeStyleExt, UiStyleShorthandExt};
    pub use crate::ui_builder::{UiBuilder, UiBuilderExt, UiRoot};
}
