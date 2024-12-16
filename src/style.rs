use crate::ui_builder::UiBuilder;
use crate::ui_builder::UiBuilderGetId;
use bevy::{ecs::system::EntityCommands, prelude::*};

//wraps entitycommands into a prop so they can be operated on by World-access systems
pub struct UiStyle<'a> {
    commands: EntityCommands<'a>,
}

impl UiStyle<'_> {
    /// Returns the Entity that is the target of all styling commands
    pub fn id(&self) -> Entity {
        self.commands.id()
    }

    /// Returns the underlying EntityCommands via reborrow
    pub fn entity_commands(&mut self) -> EntityCommands {
        self.commands.reborrow()
    }
}

pub trait UiStyleExt {
    /// Styling commands for UI Nodes
    ///
    /// `sickle_ui` exposes functions for all standard bevy styleable attributes.
    /// Manual extension can be done for custom styling needs via extension traits:
    ///
    /// ```rust
    /// pub trait SetMyPropExt {
    ///     fn my_prop(&mut self, value: f32) -> &mut Self;
    /// }
    ///
    /// impl SetMyPropExt for UiStyle<'_> {
    ///     fn my_prop(&mut self, value: f32) -> &mut Self {
    ///         // SetMyProp is assumed to be an EntityCommand
    ///         // Alternatively a closure can be supplied as per a standard bevy command
    ///         // NOTE: All built-in commands structs are public and can be re-used in extensions
    ///         self.entity_commands().add(SetMyProp {
    ///             value
    ///         });
    ///         self
    ///     }
    /// }
    /// ```
    fn style(&mut self) -> UiStyle;
}

// ?
/*
impl UiStyleExt for Commands<'_, '_> {
    fn style(&mut self, entity: Entity) -> UiStyle {
        UiStyle {
            commands: self.entity(entity),
        }
    }
}
*/


impl UiStyleExt for EntityCommands<'_> {
    fn style(&mut self) -> UiStyle {
        UiStyle {
            commands: self.reborrow(),
        }
    }
}


impl<T: UiBuilderGetId> UiStyleExt for UiBuilder<'_, T> {
    fn style(&mut self) -> UiStyle {

     
        UiStyle {
            commands: self.entity_commands(),
        }
    }
}
