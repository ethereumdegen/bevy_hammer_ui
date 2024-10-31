
use bevy::prelude::*;

 

pub trait Spawn {
    fn spawn<T: Bundle>(&mut self, bundle: T) -> EntityCommands;
}

impl Spawn for Commands<'_, '_> {
    fn spawn<T: Bundle>(&mut self, bundle: T) -> EntityCommands
    {
        Commands::spawn(self, bundle)
    }
}

impl Spawn for ChildBuilder<'_> {
    fn spawn<T: Bundle>(&mut self, bundle: T) -> EntityCommands
    {
        ChildBuild::spawn(self, bundle)
    }
}