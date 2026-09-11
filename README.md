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

#### Native text input

`UiTextInputExt` wraps Bevy's native `EditableText` primitive in a styled Hammer UI widget:

```rust
use bevy_hammer_ui::widgets::text_input::{TextInputConfig, UiTextInputExt};

commands
    .ui_builder(parent)
    .text_input(TextInputConfig {
        initial_value: "Captain".into(),
        max_characters: Some(64),
        ..default()
    });
```

The application must include Bevy's standard UI widget plugins, as `DefaultPlugins` does.

#### Built-in style setters (0.19.2)

Every commonly styled `Node` field has a chainable setter, plus a few component setters, so a game
no longer writes its own `SetWidth`, `SetFlexGrow`, `SetAlignSelf`... commands:

```rust
use bevy_hammer_ui::prelude::*;

commands
    .entity(panel)
    .style()
    .size(Val::Px(240.0), Val::Px(64.0))
    .flex_direction(FlexDirection::Column)
    .row_gap(Val::Px(4.0))
    .padding(UiRect::all(Val::Px(6.0)))
    .background_color(Color::srgb(0.1, 0.1, 0.14))
    .border_color(Color::srgb(0.35, 0.38, 0.5));
```

`UiNodeStyleExt` covers display, position type, overflow, the four offsets, size and its min/max,
aspect ratio, all six alignment properties, margin, padding, border, the flex properties and both
gaps. `UiStyleShorthandExt` adds `size`, `absolute_at`, `background_color`, `border_color`,
`visibility` and `z_index`. Custom setters still work exactly as before, as extension traits on
`UiStyle`.

