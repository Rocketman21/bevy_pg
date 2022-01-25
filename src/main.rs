use bevy::{
	input::mouse::MouseMotion, prelude::*,
	diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin},
	gltf::Gltf,
};
use fixed_timestep_test::FixedTimestempTestPlugin;

mod fixed_timestep_test;

fn main() {
	App::new()
		.insert_resource(Msaa { samples: 4 })
		.insert_resource(WASDMovementSettings { target_index: 0 })
		.init_resource::<GltfHandleStorage>()
		.add_plugins(DefaultPlugins)
		.add_plugin(FrameTimeDiagnosticsPlugin::default())
		// .add_plugin(FixedTimestempTestPlugin::default())
		.add_system(text_update_system)
		.add_startup_system(load_gltf)
		.add_system(spawn_gltf)
		.add_startup_system(setup)
		.add_system(wasd_movement_system)
		.add_system(rotate_with_mouse_system)
		.add_system(game_state_control_system)
		.add_state(GameState::Play)
		.run();
}

// struct ClawMachinePlugin;
#[derive(Component)]
struct MouseRotation;

#[derive(Component)]
struct WASDMovement;
struct WASDMovementSettings {
	target_index: usize,
}
#[derive(Default)]
struct GltfHandleStorage(Handle<Gltf>);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum GameState {
	Play,
	Pause,
}

fn load_gltf(
	mut commands: Commands,
	asset_server: Res<AssetServer>,
) {
	commands.insert_resource(GltfHandleStorage(asset_server.load("claw_machine_bevy.glb")));
}

fn spawn_gltf(
	mut asset_events: EventReader<AssetEvent<Gltf>>,
	assets: Res<Assets<Gltf>>,
	gltf_handle_storage: Res<GltfHandleStorage>,
	mut commands: Commands,
) {
	asset_events.iter().for_each(|event| {
		if let AssetEvent::Created { handle } = event {
			if handle == &gltf_handle_storage.0 {
				let gltf = assets.get(handle).unwrap();

				commands
					.spawn_bundle((
						Transform::from_xyz(0.0, 0.0, 0.0),
						GlobalTransform::identity(),
					))
					.with_children(|parent| {
						parent.spawn_scene(gltf.named_scenes["claw_machine"].clone());
						parent.spawn_scene(gltf.named_scenes["claw"].clone()); //.insert(WASDMovement)
					});
			}
		}
	});
}

fn setup(
	mut windows: ResMut<Windows>,
	mut commands: Commands,
	asset_server: Res<AssetServer>,
) {
	toggle_cursor(windows.get_primary_mut().unwrap(), false);

	// if let Some(gltf) = assets_gltf.get(&gltf_handle_storage.0) {
	// 	commands.spawn_scene(gltf.scenes[0].clone());

	// 	println!("handle {:?}", gltf.scenes[0].clone());
	
	

	// commands
	// 	.spawn_bundle((
	// 		Transform::from_xyz(0.0, 2.0, 0.0),
	// 		GlobalTransform::identity(),
	// 	))
	// 	.with_children(|parent| {
	// 		parent.spawn_scene(asset_server.load("claw.glb#Scene0"));
	// 	});

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

	commands.spawn_bundle(UiCameraBundle::default());
	
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
				font_size: 20.0,
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
		if index == settings.target_index {
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
			if settings.target_index < length - 1 {
				settings.target_index += 1;
			} else {
				settings.target_index = 0;
			}
		}
	}
}

fn toggle_cursor(window: &mut Window, is_enabled: bool) {
	window.set_cursor_visibility(!is_enabled);
	window.set_cursor_lock_mode(is_enabled);
}

fn game_state_control_system(
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

fn text_update_system(diagnostics: Res<Diagnostics>, mut query: Query<&mut Text>) {
	for mut text in query.iter_mut() {
			if let Some(fps) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
					if let Some(average) = fps.average() {
							text.sections[0].value = format!("FPS: {:.0}", average);
					}
			}
	}
}
