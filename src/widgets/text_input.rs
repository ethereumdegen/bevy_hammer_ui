use bevy::prelude::*;
use bevy::text::{EditableText, TextCursorStyle};

use crate::ui_builder::{UiBuilder, UiBuilderGetId};

/// Visual and editing configuration for a single-line native Bevy text input.
pub struct TextInputConfig {
    pub initial_value: String,
    pub font_size: f32,
    pub visible_width: Option<f32>,
    pub max_characters: Option<usize>,
    pub text_color: Color,
    pub background_color: Color,
    pub border_color: Color,
    pub node: Node,
}

impl Default for TextInputConfig {
    fn default() -> Self {
        Self {
            initial_value: String::new(),
            font_size: 16.0,
            visible_width: None,
            max_characters: None,
            text_color: Color::WHITE,
            background_color: Color::BLACK,
            border_color: Color::WHITE,
            node: Node {
                border: UiRect::all(Val::Px(2.0)),
                padding: UiRect::all(Val::Px(4.0)),
                ..default()
            },
        }
    }
}

/// Identifies text inputs built by [`UiTextInputExt`].
#[derive(Component, Debug, Default, Reflect)]
#[reflect(Component)]
pub struct HammerTextInput;

/// Spawns a styled, single-line [`EditableText`] using Bevy's native editing systems.
pub trait UiTextInputExt {
    fn text_input(&mut self, config: TextInputConfig) -> UiBuilder<'_, Entity>;
}

impl<T: UiBuilderGetId> UiTextInputExt for UiBuilder<'_, T> {
    fn text_input(&mut self, config: TextInputConfig) -> UiBuilder<'_, Entity> {
        self.spawn((
            config.node,
            BorderColor::all(config.border_color),
            BackgroundColor(config.background_color),
            EditableText {
                visible_width: config.visible_width,
                max_characters: config.max_characters,
                allow_newlines: false,
                ..EditableText::new(config.initial_value)
            },
            TextLayout::no_wrap(),
            TextFont::from_font_size(config.font_size),
            TextColor(config.text_color),
            TextCursorStyle::default(),
            HammerTextInput,
        ))
    }
}
