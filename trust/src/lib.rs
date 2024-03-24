#![forbid(unsafe_code)]

////////////////////////////////////////////////////////////////////////////////

#[derive(Default, Clone, Copy, Debug, PartialEq, Eq)]
pub enum Step {
    #[default]
    Cheat,
    Cooperate,
}

pub trait Agent {
    fn next_step(&self) -> Step;
    fn opponent_step(&mut self, step: Step);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RoundOutcome {
    BothCooperated,
    LeftCheated,
    RightCheated,
    BothCheated,
}

pub struct Game {
    left: Box<dyn Agent>,
    right: Box<dyn Agent>,
    left_score: i32,
    right_score: i32,
}

impl Game {
    pub fn new(left: Box<dyn Agent>, right: Box<dyn Agent>) -> Self {
        Self {
            left,
            right,
            left_score: 0,
            right_score: 0,
        }
    }

    pub fn left_score(&self) -> i32 {
        self.left_score
    }

    pub fn right_score(&self) -> i32 {
        self.right_score
    }

    pub fn play_round(&mut self) -> RoundOutcome {
        let left_step = self.left.next_step();
        let right_step = self.right.next_step();

        let res = if left_step == Step::Cheat && right_step == Step::Cheat {
            RoundOutcome::BothCheated
        } else if left_step == Step::Cheat {
            self.left_score += 3;
            self.right_score -= 1;
            RoundOutcome::LeftCheated
        } else if right_step == Step::Cheat {
            self.right_score += 3;
            self.left_score -= 1;
            RoundOutcome::RightCheated
        } else {
            self.left_score += 2;
            self.right_score += 2;
            RoundOutcome::BothCooperated
        };

        self.left.opponent_step(right_step);
        self.right.opponent_step(left_step);

        res
    }
}

////////////////////////////////////////////////////////////////////////////////

#[derive(Default)]
pub struct CheatingAgent {}

impl Agent for CheatingAgent {
    fn next_step(&self) -> Step {
        Step::Cheat
    }

    fn opponent_step(&mut self, _: Step) {}
}

////////////////////////////////////////////////////////////////////////////////

#[derive(Default)]
pub struct CooperatingAgent {}

impl Agent for CooperatingAgent {
    fn next_step(&self) -> Step {
        Step::Cooperate
    }

    fn opponent_step(&mut self, _: Step) {}
}

////////////////////////////////////////////////////////////////////////////////

#[derive(Default)]
pub struct GrudgerAgent {
    is_cheat: bool,
}

impl Agent for GrudgerAgent {
    fn next_step(&self) -> Step {
        if self.is_cheat {
            return Step::Cheat;
        }

        Step::Cooperate
    }

    fn opponent_step(&mut self, step: Step) {
        match step {
            Step::Cheat => self.is_cheat = true,

            Step::Cooperate => {}
        }
    }
}

////////////////////////////////////////////////////////////////////////////////

pub struct CopycatAgent {
    last_opponent_step: Step,
    is_first: bool,
}

impl Default for CopycatAgent {
    fn default() -> Self {
        Self {
            last_opponent_step: Step::Cheat,
            is_first: true,
        }
    }
}

impl Agent for CopycatAgent {
    fn next_step(&self) -> Step {
        if self.is_first {
            return Step::Cooperate;
        }

        self.last_opponent_step
    }

    fn opponent_step(&mut self, step: Step) {
        if self.is_first {
            self.is_first = false
        }
        self.last_opponent_step = step
    }
}

////////////////////////////////////////////////////////////////////////////////

#[derive(Default)]
pub struct DetectiveAgent {
    step_number: i32,
    last_opponent_step: Step,
    opponent_is_cheat: bool,
}

impl Agent for DetectiveAgent {
    fn next_step(&self) -> Step {
        if matches!(self.step_number, 0 | 2 | 3) {
            return Step::Cooperate;
        }

        if self.step_number == 1 {
            return Step::Cheat;
        }

        if !self.opponent_is_cheat {
            return Step::Cheat;
        }

        self.last_opponent_step
    }

    fn opponent_step(&mut self, step: Step) {
        self.step_number += 1;
        self.last_opponent_step = step;
        if self.step_number <= 4 {
            match step {
                Step::Cheat => self.opponent_is_cheat = true,

                Step::Cooperate => {}
            }
        }
    }
}
