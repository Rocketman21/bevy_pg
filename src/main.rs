use bevy::{input::mouse::MouseMotion, prelude::*};

fn main() {
	App::new()
		.insert_resource(Msaa { samples: 4 })
		.insert_resource(WASDMovementSettings { targert_index: 0 })
		.add_plugins(DefaultPlugins)
		// .add_plugin(ClawMachinePlugin)
		// .add_system(text_update_system.system())
		.add_startup_system(setup)
		.add_system(wasd_movement_system)
		.add_system(rotate_with_mouse_system)
		.add_system(game_state_control)
		.add_state(GameState::Play)
		.run();
}

// struct ClawMachinePlugin;
#[derive(Component)]
struct MouseRotation;

#[derive(Component)]
struct WASDMovement;
struct WASDMovementSettings {
	targert_index: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum GameState {
	Play,
	Pause,
}

// impl Plugin for ClawMachinePlugin {
// 	fn build(&self, app: &mut AppBuilder) {
// 		app
// 			.add_startup_system(setup.system())
// 			.add_system(wasd_movement_system.system())
// 			.add_system(rotate_with_mouse_system.system());
// 	}
// }

fn create_glass(
	mesh_resource: &mut ResMut<Assets<Mesh>>,
	materials_resource: &mut ResMut<Assets<StandardMaterial>>,
	mesh: Mesh,
	transform: Transform,
) -> PbrBundle {
	PbrBundle {
		mesh: mesh_resource.add(mesh),
		material: materials_resource.add(StandardMaterial {
			base_color: Color::rgba(1.0, 1.0, 1.0, 0.159),
			metallic: 0.717,
			perceptual_roughness: 0.095,
			alpha_mode: AlphaMode::Blend,
			..Default::default()
		}),
		// visible: Visibility {
		// 	..Default::default()
		// },
		transform,
		..Default::default()
	}
}

fn setup(
	mut windows: ResMut<Windows>,
	mut commands: Commands,
	asset_server: Res<AssetServer>,
	mut meshes: ResMut<Assets<Mesh>>,
	mut materials: ResMut<Assets<StandardMaterial>>,
) {
	toggle_cursor(windows.get_primary_mut().unwrap(), false);

	commands.spawn_scene(asset_server.load("claw_machine_transparent.glb#Scene0"));

	// Glass
	// commands.spawn_bundle(create_glass(
	// 	&mut meshes,
	// 	&mut materials,
	// 	Mesh::from(shape::Box::new(1.7, 2.1, 1.7)),
	// 	Transform::from_xyz(0.0, 2.6, 0.0),
	// ));

	commands
		.spawn_bundle((
			Transform::from_xyz(0.0, 1.5, 0.0),
			GlobalTransform::identity(),
		))
		.with_children(|parent| {
			parent.spawn_bundle(create_glass(
				&mut meshes,
				&mut materials,
				Mesh::from(shape::Quad::default()),
				Transform::from_xyz(0.0, 0.0, 0.0),
			));
		})
		.insert(WASDMovement);

	commands
		.spawn_bundle((
			Transform::from_xyz(0.0, 2.0, 0.0),
			GlobalTransform::identity(),
		))
		.with_children(|parent| {
			parent.spawn_scene(asset_server.load("claw.glb#Scene0"));
		});

	commands.spawn_bundle(PointLightBundle {
		transform: Transform::from_translation(Vec3::new(4.0, 5.0, 4.0)),
		..Default::default()
	});

	commands
		.spawn_bundle(PerspectiveCameraBundle {
			transform: Transform::from_xyz(-2.0, 5.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
			..Default::default()
		})
		.insert(WASDMovement)
		.insert(MouseRotation);
	
	commands.spawn_bundle(TextBundle {
		style: Style {
			align_self: AlignSelf::FlexEnd,
			position_type: PositionType::Absolute,
			position: Rect {
				top: Val::Px(5.0),
				left: Val::Px(15.0),
				..Default::default()
			},
			..Default::default()
		},
		text: Text::with_section(
			"FPS:",
			TextStyle {
				font: asset_server.load("fonts/Alata-Regular.ttf"),
				font_size: 60.0,
				color: Color::WHITE,
			},
			Default::default()
		),
		..Default::default()
	});
}

fn wasd_movement_system(
	time: Res<Time>,
	keyboard_input: Res<Input<KeyCode>>,
	mut query: Query<(&WASDMovement, &mut Transform)>,
	mut settings: ResMut<WASDMovementSettings>,
) {
	let query_iter = query.iter_mut();
	let query_len = query_iter.size_hint();

	for (index, (_, mut transform)) in query_iter.enumerate() {
		if index == settings.targert_index {
			const SPEED: f32 = 10.0;
			let distance = SPEED * time.delta_seconds();
			let translation = &mut transform.translation;
	
			if keyboard_input.pressed(KeyCode::W) {
				translation.z -= distance;
			}
			if keyboard_input.pressed(KeyCode::S) {
				translation.z += distance;
			}
			if keyboard_input.pressed(KeyCode::A) {
				translation.x -= distance;
			}
			if keyboard_input.pressed(KeyCode::D) {
				translation.x += distance;
			}
			if keyboard_input.pressed(KeyCode::LShift) {
				translation.y -= distance;
			}
			if keyboard_input.pressed(KeyCode::Space) {
				translation.y += distance;
			}
		}
	}

	if keyboard_input.just_pressed(KeyCode::Tab) {
		if let Some(length) = query_len.1 {
			if settings.targert_index < length - 1 {
				settings.targert_index += 1;
			} else {
				settings.targert_index = 0;
			}
		}
	}
}

fn toggle_cursor(window: &mut Window, is_enabled: bool) {
	window.set_cursor_visibility(!is_enabled);
	window.set_cursor_lock_mode(is_enabled);
}

fn game_state_control(
	keyboard_input: Res<Input<KeyCode>>,
	mut windows: ResMut<Windows>,
	mut state: ResMut<State<GameState>>,
) {
	if keyboard_input.just_pressed(KeyCode::Escape) {
		let window = windows.get_primary_mut().unwrap();

		match state.current() {
			GameState::Play => {
				if let Ok(()) = state.set(GameState::Pause) {
					toggle_cursor(window, false);
				}
				println!("pausing");
			}
			GameState::Pause => {
				if let Ok(()) = state.set(GameState::Play) {
					toggle_cursor(window, true);
				}
				println!("playing");
			}
		}
	}
}

fn rotate_with_mouse_system(
	mut mouse_motion_events: EventReader<MouseMotion>,
	mut query: Query<(&MouseRotation, &mut Transform)>,
	windows: ResMut<Windows>,
	state: ResMut<State<GameState>>,
) {
	const SENSITIVITY: f32 = 0.0001;
	let window = windows.get_primary().unwrap();

	for (_, mut transform) in query.iter_mut() {
		for event in mouse_motion_events.iter() {
			if let GameState::Play = state.current() {
				transform.rotate(
					Quat::from_rotation_y((-event.delta.x * SENSITIVITY * window.width()).to_radians())
					* Quat::from_rotation_x((-event.delta.y * SENSITIVITY * window.height()).to_radians())
				);
			}
		}
	}
}

// fn text_update_system(diagnostics: Res<Diagnostics>, mut query: Query<Mut<Text>>) {
// 	for mut text in &mut query.iter() {
// 			if let Some(fps) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
// 					if let Some(average) = fps.average() {
// 							text.value = format!("FPS: {:.2}", average);
// 					}
// 			}
// 	}
// }