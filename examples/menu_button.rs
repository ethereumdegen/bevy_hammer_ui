
use bevy::{ecs::system::{EntityCommand,EntityCommands}, prelude::*};


use bevy_hammer_ui::spawn::Spawn  ;
use bevy_hammer_ui::style::{UiStyle,UiStyleExt};



 

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run();
}



pub fn setup (
	    mut commands: Commands
	    ) {

	 let root_node = commands
	 .spawn(NodeBundle::default())
	 .style()
	 	//style stuff 
	 .id()

	 ;


	 let container_node = commands
        .entity(root_node)
        .container( NodeBundle::default() , |inner| {

        	inner
        	.spawn(NodeBundle::default()) 
        	.style()
        	.width(Val::Px(50.0) );

        } )	
        
        .style()
        .width(Val::Px(100.0)  )
        .id();


 


}




// implement these yourself !! Lets you use a function call to spawn a child + supports a function callback for nesting coolness 


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
        spawn_children: impl FnOnce(&mut  EntityCommands ),
    ) -> EntityCommands {

	 	let mut new_builder = self.spawn(bundle);
        spawn_children(&mut new_builder);  //calls the callback

        new_builder


	 }

	 
}




// implement these yourself ! (sickle ui used macros...)

struct SetUiStyleWidth (Val);

impl EntityCommand for SetUiStyleWidth {
    fn apply(self, entity: Entity, world: &mut World) {
        

        let Some(mut style_comp) = world.get_mut::<Style>(entity) else {
             
            return;
        };

       style_comp.width = self.0 ;
    }
}



pub trait SetUiStyleWidthExt<'a> {
    fn width(&'a mut self, val: Val) -> &mut UiStyle<'a>;
}

impl<'a> SetUiStyleWidthExt<'a> for UiStyle<'a> {
    fn width(&'a mut self, val: Val) -> &mut UiStyle<'a> {
        self.entity_commands().add(SetUiStyleWidth (val) );
        self
    }
}


 