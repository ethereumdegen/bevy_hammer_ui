
use bevy::{ecs::system::EntityCommands, prelude::*};

 

pub trait Spawn {
    fn spawn<T: Bundle>(&mut self, bundle: T) -> EntityCommands;
}

 

impl Spawn for Commands<'_, '_> {
    fn spawn<T: Bundle>(&mut self, bundle: T) -> EntityCommands
    {
        Commands::spawn(self, bundle)
    }
}



impl Spawn for EntityCommands<'_> {
    
      fn spawn<T: Bundle>(&mut self, bundle: T) -> EntityCommands<'_> {
        let parent_id = self.id();

        let child = self.commands().spawn(bundle).set_parent(parent_id);
                
        self.reborrow()
    }

}
 