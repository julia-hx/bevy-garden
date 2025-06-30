use bevy::prelude::*;

// ui plugin: only displays text.
// set via events.

pub struct UIPlugin;

impl Plugin for UIPlugin {
	fn build(&self, app: &mut App) {
		app.add_systems(Startup, init_ui_fields);
	}
}

#[derive(Component, Copy, Clone)]
//#[require(Node)]
struct UIElement {
	id: &'static str,
}

impl UIElement {
	fn new(id: &'static str) -> Self {
		Self {
			id,
		}
	}
}

fn init_ui_fields(mut commands: Commands) {
	commands.spawn((
		UIElement::new("stage"),
		Text::new("stage 0"),
		Node {
            position_type: PositionType::Absolute,
            top: Val::Px(10.0),
            left: Val::Px(10.0),
            ..default()
        },

	));

	commands.spawn((
		Node {
            position_type: PositionType::Absolute,
            top: Val::Px(10.0),
            right: Val::Px(10.0),
            ..default()
        },
	)).with_children(|builder| {
		builder.spawn((
			UIElement::new("score"),
			Text::new("0 of 0"),
		));
	});

	let container = commands.spawn((
		Node {
			width: Val::Percent(100.0),
			height:Val::Percent(100.0),
			flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
			..default()	
		},
	)).id();

	let header = commands.spawn((
		Node {
            align_items: AlignItems::Center,
			justify_content: JustifyContent::SpaceEvenly,
			top: Val::Percent(25.0),
            ..default()
        },
	)).with_children(|builder| {
		builder.spawn((
			UIElement::new("header"),
			Text::new("ready?"),
		));
	}).id();

	let sub_header = commands.spawn((
		Node {
            align_items: AlignItems::Center,
			justify_content: JustifyContent::SpaceEvenly,
			top: Val::Percent(28.0),
            ..default()
        },
	)).with_children(|builder| {
		builder.spawn((
			UIElement::new("sub_header"),
			Text::new("well?"),
		));
	}).id();

	commands.entity(container).add_child(header);
	commands.entity(container).add_child(sub_header);
}
