struct Game {
    board: Board,
    players: Vec<Box<dyn Player>>,
    level: i32,
    phase: Phase,
}

struct Board {}

trait Player {
    fn do_something(self);
}

struct NoOpPlayer {}

impl Player for NoOpPlayer {
    fn do_something(self) {}
}

enum Phase {
    StartOfLevel,
    CreepsRoaming,
    EndOfLevel,
}

impl Game {
    fn new() -> Self {
        Self {
            board: Board {},
            players: vec![Box::new(NoOpPlayer {})],
            level: 0,
            phase: Phase::StartOfLevel,
        }
    }
    fn run(&mut self) {}
    fn print_result(&mut self) {}
}

fn main() {
    let mut game = Game::new();
    game.run();
    game.print_result();
}
