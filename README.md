### Bevy Hammer UI

A barebones UI framework for bevy, using UiBuilder pattern to retain Command context.

Inspired by SickleUi  (basically the most important 2 files from sickle ui)


####  Use vanilla bevy UI but with a bit more power: 


```

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


```


#### Define your own custom widgets and style-commands


```

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

```


```


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

```
