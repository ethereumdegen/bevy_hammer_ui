use bevy::{ecs::system::EntityCommands, prelude::*};

pub trait Spawn {
    fn spawn<T: Bundle>(&mut self, bundle: T) -> EntityCommands;
}

impl Spawn for Commands<'_, '_> {
    fn spawn<T: Bundle>(&mut self, bundle: T) -> EntityCommands {
        Commands::spawn(self, bundle)
    }
}



impl  Spawn for EntityCommands<'_>  {
    fn spawn<T: Bundle>(&mut self, bundle: T) -> EntityCommands  {

        let parent_id = self.id();
        let mut new_entity_id;

        // Spawn the new entity as a child of the current entity
        self.commands().entity(parent_id).with_children(|parent| {
            new_entity_id = parent.spawn(bundle).id();
        });

        // Return `EntityCommands` for the newly spawned entity
        self.commands().entity(new_entity_id)
    }
}



/*
impl Spawn for EntityCommands<'_> {
    fn spawn<T: Bundle>(&mut self, bundle: T) -> EntityCommands<'_> {
        let parent_id = self.id();

        let child = self.commands().spawn(bundle).set_parent(parent_id);

        self.reborrow()
    }
}
*/