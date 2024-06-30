pub struct Spawn {
    pub quantity: u32,
    pub health: f32,
    pub speed: f32,
    pub bounty: f32, // bounty for the whole spawn. per creep this is bounty / quantity
}

pub struct Spawner {
    spawn: Spawn,
}
impl Spawner {
    pub fn new(spawn: Spawn) -> Self {
        Spawner { spawn }
    }

    pub fn tick() {}
}
