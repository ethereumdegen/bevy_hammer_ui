//! Chainable setters for the standard [`Node`] fields and the everyday UI components, so a game
//! does not hand-write the same twenty `EntityCommand`s before it can style anything.
//!
//! ```rust,ignore
//! use bevy_hammer_ui::prelude::*;
//!
//! commands
//!     .entity(panel)
//!     .style()
//!     .width(Val::Px(240.0))
//!     .flex_direction(FlexDirection::Column)
//!     .row_gap(Val::Px(4.0))
//!     .background_color(Color::srgb(0.1, 0.1, 0.14));
//! ```
//!
//! Every setter queues a command that edits the component in place when commands are applied. A
//! setter aimed at an entity without a [`Node`] leaves it alone rather than panicking; the
//! component setters insert (or replace) their component.

use bevy::prelude::*;

use crate::style::UiStyle;

macro_rules! node_setters {
    ($( $(#[$meta:meta])* $name:ident : $ty:ty ),* $(,)?) => {
        /// Setters for every commonly styled [`Node`] field, on a [`UiStyle`].
        pub trait UiNodeStyleExt {
            $( $(#[$meta])* fn $name(&mut self, value: $ty) -> &mut Self; )*
        }

        impl UiNodeStyleExt for UiStyle<'_> {
            $(
                fn $name(&mut self, value: $ty) -> &mut Self {
                    self.entity_commands().queue(move |mut entity: EntityWorldMut| {
                        if let Some(mut node) = entity.get_mut::<Node>() {
                            node.$name = value;
                        }
                    });
                    self
                }
            )*
        }
    };
}

node_setters! {
    /// [`Node::display`]
    display: Display,
    /// [`Node::position_type`]
    position_type: PositionType,
    /// [`Node::overflow`]
    overflow: Overflow,
    /// [`Node::left`]
    left: Val,
    /// [`Node::right`]
    right: Val,
    /// [`Node::top`]
    top: Val,
    /// [`Node::bottom`]
    bottom: Val,
    /// [`Node::width`]
    width: Val,
    /// [`Node::height`]
    height: Val,
    /// [`Node::min_width`]
    min_width: Val,
    /// [`Node::min_height`]
    min_height: Val,
    /// [`Node::max_width`]
    max_width: Val,
    /// [`Node::max_height`]
    max_height: Val,
    /// [`Node::aspect_ratio`]
    aspect_ratio: Option<f32>,
    /// [`Node::align_items`]
    align_items: AlignItems,
    /// [`Node::justify_items`]
    justify_items: JustifyItems,
    /// [`Node::align_self`]
    align_self: AlignSelf,
    /// [`Node::justify_self`]
    justify_self: JustifySelf,
    /// [`Node::align_content`]
    align_content: AlignContent,
    /// [`Node::justify_content`]
    justify_content: JustifyContent,
    /// [`Node::margin`]
    margin: UiRect,
    /// [`Node::padding`]
    padding: UiRect,
    /// [`Node::border`]
    border: UiRect,
    /// [`Node::flex_direction`]
    flex_direction: FlexDirection,
    /// [`Node::flex_wrap`]
    flex_wrap: FlexWrap,
    /// [`Node::flex_grow`]
    flex_grow: f32,
    /// [`Node::flex_shrink`]
    flex_shrink: f32,
    /// [`Node::flex_basis`]
    flex_basis: Val,
    /// [`Node::row_gap`]
    row_gap: Val,
    /// [`Node::column_gap`]
    column_gap: Val,
}

/// Shorthands and component setters that are not a single [`Node`] field.
pub trait UiStyleShorthandExt {
    /// Width and height at once.
    fn size(&mut self, width: Val, height: Val) -> &mut Self;
    /// [`PositionType::Absolute`] with a left and top offset.
    fn absolute_at(&mut self, left: Val, top: Val) -> &mut Self;
    /// Inserts (or replaces) a [`BackgroundColor`].
    fn background_color(&mut self, color: Color) -> &mut Self;
    /// Inserts (or replaces) a [`BorderColor`] on all four sides.
    fn border_color(&mut self, color: Color) -> &mut Self;
    /// Inserts (or replaces) the [`Visibility`].
    fn visibility(&mut self, visibility: Visibility) -> &mut Self;
    /// Inserts (or replaces) the [`ZIndex`].
    fn z_index(&mut self, z_index: i32) -> &mut Self;
}

impl UiStyleShorthandExt for UiStyle<'_> {
    fn size(&mut self, width: Val, height: Val) -> &mut Self {
        self.width(width).height(height)
    }

    fn absolute_at(&mut self, left: Val, top: Val) -> &mut Self {
        self.position_type(PositionType::Absolute)
            .left(left)
            .top(top)
    }

    fn background_color(&mut self, color: Color) -> &mut Self {
        self.entity_commands().insert(BackgroundColor(color));
        self
    }

    fn border_color(&mut self, color: Color) -> &mut Self {
        self.entity_commands().insert(BorderColor::all(color));
        self
    }

    fn visibility(&mut self, visibility: Visibility) -> &mut Self {
        self.entity_commands().insert(visibility);
        self
    }

    fn z_index(&mut self, z_index: i32) -> &mut Self {
        self.entity_commands().insert(ZIndex(z_index));
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::style::UiStyleExt;
    use bevy::ecs::world::CommandQueue;

    #[test]
    fn setters_edit_the_node_and_insert_components() {
        let mut world = World::new();
        let panel = world.spawn(Node::default()).id();
        let mut queue = CommandQueue::default();
        {
            let mut commands = Commands::new(&mut queue, &world);
            let mut entity = commands.entity(panel);
            entity
                .style()
                .size(Val::Px(240.0), Val::Px(64.0))
                .flex_direction(FlexDirection::Column)
                .flex_grow(2.0)
                .padding(UiRect::all(Val::Px(4.0)))
                .background_color(Color::BLACK)
                .z_index(3);
        }
        queue.apply(&mut world);

        let node = world.get::<Node>(panel).unwrap();
        assert_eq!(node.width, Val::Px(240.0));
        assert_eq!(node.height, Val::Px(64.0));
        assert_eq!(node.flex_direction, FlexDirection::Column);
        assert_eq!(node.flex_grow, 2.0);
        assert_eq!(node.padding, UiRect::all(Val::Px(4.0)));
        assert_eq!(world.get::<BackgroundColor>(panel).unwrap().0, Color::BLACK);
        assert_eq!(world.get::<ZIndex>(panel).unwrap().0, 3);
    }

    #[test]
    fn a_setter_on_an_entity_without_a_node_does_nothing() {
        let mut world = World::new();
        let bare = world.spawn_empty().id();
        let mut queue = CommandQueue::default();
        {
            let mut commands = Commands::new(&mut queue, &world);
            commands.entity(bare).style().width(Val::Px(10.0));
        }
        queue.apply(&mut world);
        assert!(world.get::<Node>(bare).is_none());
    }
}
