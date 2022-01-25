use bevy::{prelude::*, core::FixedTimestep};

const TIMESTEP_PER_SECOND: f64 = 2.0;

#[derive(Default)]
pub struct FixedTimestempTestPlugin;

impl Plugin for FixedTimestempTestPlugin {
	fn build(&self, app: &mut App) {
		app
			.insert_resource(TimeTracker { seconds: 0. })
			.add_system_set(
				SystemSet::new()
					.with_run_criteria(FixedTimestep::steps_per_second(TIMESTEP_PER_SECOND))
					.with_system(timestep)
			);
	}
}

struct TimeTracker {
	seconds: f64,
}

fn timestep(time: Res<Time>, mut prev_time: ResMut<TimeTracker>) {
	// delta_1 - разница во времени между итерациями главного цикла движка,
	// и она на момент bevy 0.6 неразрывно связана с fps.
	//
	// delta_2 - разница во времени между вызовами этой системы (timestep).
	// Это значение практически не зависит от fps. Подобную технику можно
	// применять например для автоматной очереди в шутерах
	println!("delta_1: {}, delta_2: {}", time.delta_seconds(), time.seconds_since_startup() - prev_time.seconds);
	prev_time.seconds = time.seconds_since_startup();
}