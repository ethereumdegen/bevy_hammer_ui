use bevy::prelude::*;

pub trait UiContainerExt {
    fn container(
        &mut self,
        bundle: impl Bundle,
        spawn_children: impl FnOnce(&mut EntityCommands),
    ) -> EntityCommands;
}

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
