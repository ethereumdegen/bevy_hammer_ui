use bevy_hammer_ui::ui_builder::UiBuilder;
use bevy_hammer_ui::ui_builder::UiBuilderExt;
use bevy::{
    ecs::system::{EntityCommand, EntityCommands},
    prelude::*,
};

 
use bevy_hammer_ui::style::{UiStyle, UiStyleExt};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run();
}

pub fn setup(mut commands: Commands) {
    let root_node = commands
        .spawn(Node ::default())
        .style()
        //style stuff
        .id();

    let container_node = commands
        .ui_builder(root_node)
        .container(Node ::default(), |inner| {
            inner
                .spawn(Node ::default())
                .style()
                .width(Val::Px(50.0));
        })
        .style()
        .width(Val::Px(100.0))
        .id();
}

// implement these yourself !! Lets you use a function call to spawn a child + supports a function callback for nesting coolness

pub trait UiContainerExt {
    fn container(
        &mut self,
        bundle: impl Bundle,
        spawn_children: impl FnOnce(&mut UiBuilder<Entity>),
    ) -> UiBuilder<Entity>;
}

/*
impl UiContainerExt for EntityCommands<'_> {
    fn container(
        &mut self,
        bundle: impl Bundle,
        spawn_children: impl FnOnce(&mut EntityCommands),
    ) -> EntityCommands {
        let mut new_builder = self.spawn(bundle);
        spawn_children(&mut new_builder); //calls the callback

        new_builder
    }
}
*/


impl UiContainerExt for UiBuilder<'_, Entity> {
    fn container(
        &mut self,
        bundle: impl Bundle,
        spawn_children: impl FnOnce(&mut UiBuilder<Entity>),
    ) -> UiBuilder<Entity> {
        let mut new_builder = self.spawn(bundle);
        spawn_children(&mut new_builder);

        new_builder
    }
}


// implement these yourself ! (sickle ui used macros...)

struct SetUiStyleWidth(Val);

impl EntityCommand for SetUiStyleWidth {
    fn apply(self, entity: Entity, world: &mut World) {
        let Some(mut style_comp) = world.get_mut::<Node>(entity) else {
            return;
        };

        style_comp.width = self.0;
    }
}

pub trait SetUiStyleWidthExt<'a> {
    fn width(&'a mut self, val: Val) -> &mut UiStyle<'a>;
}

impl<'a> SetUiStyleWidthExt<'a> for UiStyle<'a> {
    fn width(&'a mut self, val: Val) -> &mut UiStyle<'a> {
        self.entity_commands().queue(SetUiStyleWidth(val));
        self
    }
}
