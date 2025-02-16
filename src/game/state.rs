use std::{fmt, num::NonZero};

#[derive(Default, Debug)]
pub struct GameState {
    player_1_points: u8,
    player_2_points: u8,
}

pub enum Player {
    Player1,
    Player2,
}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Player1 => f.write_str("Player 1"),
            Self::Player2 => f.write_str("Player 2"),
        }
    }
}

impl GameState {
    pub fn score(&mut self, player: &Player) {
        match player {
            Player::Player1 => self.player_1_points += 1,
            Player::Player2 => self.player_2_points += 1,
        }
    }

    pub fn winner(&self, winning_points: NonZero<u8>) -> Option<Player> {
        if self.player_1_points >= winning_points.into() {
            Some(Player::Player1)
        } else if self.player_2_points >= winning_points.into() {
            Some(Player::Player2)
        } else {
            None
        }
    }

    pub fn fmt_score(&self) -> String {
        format!("{}:{}", self.player_1_points, self.player_2_points)
    }
}
