#[derive(Debug, PartialEq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}

#[derive(Default)]
pub struct BowlingGame {
    frames: Vec<u16>,
    current_pins: u16,
    is_done: bool,
    needs_fill_ball: bool,
}

fn compute_score((frame_num, window): (usize, &[u16])) -> u16 {
    let total: u16 = window.iter().take(2).sum();

    let is_strike = window[0] == 10 || frame_num == 9;
    let is_valid_consecutive_strike = is_strike && window[2] == 10 && frame_num < 8;
    let is_spare = total == 10;

    let bonus = if is_valid_consecutive_strike {
        3
    } else if is_strike {
        2
    } else if is_spare {
        1
    } else {
        0
    };

    total + window.iter().skip(2).take(bonus).sum::<u16>()
}

impl BowlingGame {
    pub fn new() -> Self {
        BowlingGame {
            frames: vec![],
            current_pins: 10,
            is_done: false,
            needs_fill_ball: false,
        }
    }

    fn is_open_frame(&self) -> bool {
        (self.frames.len() - 1) % 2 == 0
    }

    fn is_final_frame(&self) -> bool {
        self.frames.len() >= 19
    }

    fn is_strike(&self) -> bool {
        self.is_open_frame() && self.current_pins == 0
    }

    fn is_game_over(&self) -> bool {
        !self.needs_fill_ball && self.frames.len() > 19 || self.frames.len() == 21
    }

    fn needs_reset(&self) -> bool {
        // in non-final off-frame
        (!self.is_open_frame() && !self.is_final_frame())
            // fill ball, keeps filling pins!
            || (self.is_open_frame() || self.is_final_frame()) && self.current_pins == 0
    }

    pub fn roll(&mut self, pins: u16) -> Result<(), Error> {
        if pins > self.current_pins {
            return Err(Error::NotEnoughPinsLeft);
        } else if self.is_done {
            return Err(Error::GameComplete);
        }

        self.frames.push(pins);
        self.current_pins -= pins;

        if self.is_strike() && !self.is_final_frame() {
            self.frames.push(0);
        }

        if self.is_final_frame() && self.current_pins == 0 {
            self.needs_fill_ball = true;
        }

        if self.needs_reset() {
            self.current_pins = 10;
        }

        if self.is_game_over() {
            self.is_done = true;
        }

        Ok(())
    }

    pub fn score(&self) -> Option<u16> {
        let padded_frames = [self.frames.as_slice(), vec![0, 0, 0, 0].as_slice()].concat();

        if !self.is_done {
            return None;
        }

        Some(
            padded_frames
                .windows(6)
                .step_by(2)
                .enumerate()
                .map(compute_score)
                .sum(),
        )
    }
}
