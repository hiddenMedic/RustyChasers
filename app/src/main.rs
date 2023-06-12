use tokio;
use lib_simulation::{SimulationConfig, IndividualConfig, Model};
use std::f32::consts::PI;

fn setup_configs() -> (SimulationConfig, IndividualConfig, IndividualConfig){
    let sim_conf:SimulationConfig = SimulationConfig {
        generation_length: 2500,
        nhervors: 1,
        nchasers: 0,
        nplants: 30,
        nworlds: 1000,
        safe_evolve: true,
        respawn_plants: false,
        mutation_probability: 0.005,
        mutation_magnitude: 0.3,
        parallelized: true
    };
    let hervor_conf =IndividualConfig {
        speed_min: 0.001,
        speed_max: 0.005,
        speed_accel: 0.2,
        rotation_accel: PI / 32.0,
        fov_angle: PI + PI / 4.0,
        fov_range: 0.25,
        eye_cells: 9,
        training_model: Model::CLOSEST
    }; 
    let chaser_conf = IndividualConfig {
        speed_min: 0.001,
        speed_max: 0.004,
        speed_accel: 0.2,
        rotation_accel: PI / 32.0,
        fov_angle: PI + PI / 4.0,
        fov_range: 0.25,
        eye_cells: 9,
        training_model: Model::CLOSEST
    };

    (sim_conf, hervor_conf, chaser_conf)
}

#[tokio::main]
async fn main() {
    std::env::set_var("RUST_BACKTRACE", "1");

    let (sim_conf, hervor_conf, chaser_conf) = setup_configs();
    lib_graphics::create_window(sim_conf, hervor_conf, chaser_conf);

    loop {
        std::thread::sleep(std::time::Duration::new(10000000000000, 0));
    }
}
 