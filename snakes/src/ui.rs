use bevy::prelude::*;

// ui plugin: only displays text.
// set via events.

pub struct UIPlugin;

impl Plugin for UIPlugin {
	fn build(&self, app: &mut App) {
		app.add_event::<UIEvent>();
		app.add_systems(Startup, init_ui_elements);
		app.add_systems(Update, read_ui_events);
	}
}

#[derive(Event)]
pub struct UIEvent
{
	pub id: &'static str,
	pub text: String,
}

#[derive(Component, Copy, Clone)]
#[require(Node)]
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

fn init_ui_elements(mut commands: Commands) {
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
			top: Val::Percent(10.0),
            ..default()
        },
	)).with_children(|builder| {
		builder.spawn((
			UIElement::new("header"),
			Text::new("header"),
		));
	}).id();

	let sub_header = commands.spawn((
		Node {
            align_items: AlignItems::Center,
			justify_content: JustifyContent::SpaceEvenly,
			top: Val::Percent(80.0),
            ..default()
        },
	)).with_children(|builder| {
		builder.spawn((
			UIElement::new("sub_header"),
			Text::new("sub_header"),
		));
	}).id();

	commands.entity(container).add_child(header);
	commands.entity(container).add_child(sub_header);
}

fn read_ui_events(
	mut ui_events: EventReader<UIEvent>,
	mut query: Query<(&UIElement, &mut Text)>,
) {
	for e in ui_events.read() {
		for (element, mut text) in &mut query {
			if e.id == element.id {
				text.clear();
				text.0 = e.text.clone();
			}
		}
	} 
}
