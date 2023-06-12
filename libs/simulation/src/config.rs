use serde;

#[derive(serde::Serialize, serde::Deserialize, PartialEq, Clone)]
pub enum Model{
    POSITIONAL, CELLULAR, CLOSEST
}

#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct IndividualConfig {
    pub speed_min: f32,
    pub speed_max: f32,
    pub speed_accel: f32,
    pub rotation_accel: f32,
    pub fov_range: f32,
    pub fov_angle: f32,
    pub eye_cells: usize,
    pub training_model: Model
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct SimulationConfig {
    pub generation_length: usize,
    pub nhervors: usize,
    pub nchasers: usize,
    pub nplants: usize,
    pub nworlds: usize,
    pub safe_evolve: bool,
    pub respawn_plants: bool,
    pub mutation_probability: f32,
    pub mutation_magnitude: f32,
    pub parallelized: bool,
}