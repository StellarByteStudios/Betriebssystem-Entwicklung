use usrlib::{gameengine::position::Position, print_setpos};

pub struct Score {
    player: usize,
    enemy: usize,
}

impl Score {
    pub fn new() -> Self {
        return Score {
            player: 0,
            enemy: 0,
        };
    }

    pub fn score_player(&mut self) {
        self.player += 1;
    }

    pub fn score_enemy(&mut self) {
        self.enemy += 1;
    }

    pub fn print_score(&self, pos: &Position) {
        print_setpos!(
            pos.get_x() as usize,
            pos.get_y() as usize,
            "Player = {} - {} = Enemy",
            self.player,
            self.enemy
        );
    }
}
