#[derive(Clone)]
#[derive(Copy)]
struct Level {
    pub lines_to_next_level: u8,
    pub tick_interval: u128,
}

impl Level {
    pub fn new(lines_to_next_level: u8, frames_per_block_at_60: u16) -> Level {
        return Level { lines_to_next_level, tick_interval: 16667 as u128 * frames_per_block_at_60 as u128 }
    }
}

pub enum AddClearedLinesResult {
    LevelIncreased(Level),
    LevelStayedTheSame,
}

pub struct Levels {
    current_level_index: usize,
    accumulated_lines: u8,
    levels: Vec<Level>,
}

impl Levels {
    pub fn classic() -> Levels {
        return Levels {
            current_level_index: 0,
            accumulated_lines: 0,
            levels: Vec::from([
                Level::new( 10, 48),
                Level::new( 20, 43),
                Level::new( 30, 38),
                Level::new( 40, 33),
                Level::new( 50, 28),
                Level::new( 60, 23),
                Level::new( 70, 18),
                Level::new( 80, 13),
                Level::new( 90,  8),
                Level::new(100,  6),
                Level::new(100,  5),
                Level::new(100,  5),
                Level::new(100,  5),
                Level::new(100,  4),
                Level::new(100,  4),
                Level::new(100,  4),
                Level::new(110,  3),
                Level::new(120,  3),
                Level::new(130,  3),
                Level::new(140,  2),
                Level::new(150,  2),
                Level::new(160,  2),
                Level::new(170,  2),
                Level::new(180,  2),
                Level::new(190,  2),
                Level::new(200,  2),
                Level::new(200,  2),
                Level::new(200,  2),
                Level::new(200,  2),
                Level::new(200,  1),
            ]),
        };
    }

    pub fn add_cleared_lines(&mut self, cleared_lines: u8) -> AddClearedLinesResult {
        let mut level_increased = false;
        self.accumulated_lines += cleared_lines;
        let required_lines_to_next_level = self.levels[self.current_level_index].lines_to_next_level;
        if self.accumulated_lines >= required_lines_to_next_level && self.current_level_index < self.levels.len() {
            self.accumulated_lines -= required_lines_to_next_level;
            self.current_level_index += 1;
            return AddClearedLinesResult::LevelIncreased(self.levels[self.current_level_index]);
        }

        return AddClearedLinesResult::LevelStayedTheSame;
    }
}