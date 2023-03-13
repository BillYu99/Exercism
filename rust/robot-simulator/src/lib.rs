// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Copy, Clone)]
pub struct Robot {
    direction: Direction,
    x_coor: i32,
    y_coor: i32,
}

impl Robot {
    pub fn new(x: i32, y: i32, d: Direction) -> Self {
        Self {
            direction: d,
            x_coor: x,
            y_coor: y,
        }
    }

    #[must_use]
    pub fn turn_right(self) -> Self {
        use Direction::*;

        let facing = match self.direction {
            North => East,
            East => South,
            South => West,
            West => North,
        };

        Self {
            direction: facing,
            x_coor: self.x_coor,
            y_coor: self.y_coor,
        }
    }

    #[must_use]
    pub fn turn_left(self) -> Self {
        use Direction::*;

        let facing = match self.direction {
            North => West,
            East => North,
            South => East,
            West => South,
        };

        Self {
            direction: facing,
            x_coor: self.x_coor,
            y_coor: self.y_coor,
        }
    }

    #[must_use]
    pub fn advance(self) -> Self {
        use Direction::*;

        let robot = match self.direction {
            North => Self {
                direction: self.direction,
                x_coor: self.x_coor,
                y_coor: self.y_coor + 1,
            },
            East => Self {
                direction: self.direction,
                x_coor: self.x_coor + 1,
                y_coor: self.y_coor,
            },
            South => Self {
                direction: self.direction,
                x_coor: self.x_coor,
                y_coor: self.y_coor - 1,
            },
            West => Self {
                direction: self.direction,
                x_coor: self.x_coor - 1,
                y_coor: self.y_coor,
            },
        };
        robot
    }

    #[must_use]
    pub fn instructions(&self, instructions: &str) -> Self {
        let mut robot = self.clone();

        for s in instructions.chars() {
            if s == 'L' {
                robot = robot.turn_left();
            } else if s == 'R' {
                robot = robot.turn_right();
            } else {
                robot = robot.advance();
            }
        }
        robot
    }

    pub fn position(&self) -> (i32, i32) {
        let position = (self.x_coor, self.y_coor);
        position
    }

    pub fn direction(&self) -> &Direction {
        &self.direction
    }
}
