/// Cardinal Directions
#[derive(PartialEq, Eq, Debug)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

/// This robot struct is pretty simple, it holds the position and direction of the robot.
pub struct Robot {
    x: i32,
    y: i32,
    d: Direction,
}

impl Robot {
    /// Robot constructor takes in the x and y coordinate and also the direction it is facing.
    pub fn new(x: i32, y: i32, d: Direction) -> Self {
        Self{x, y, d}
    }

    #[must_use]
    pub fn turn_right(mut self) -> Self {
        match self.d {
            Direction::North => {
                self.d = Direction::East;
            }
            Direction::East => {
                self.d = Direction::South;
            }
            Direction::South => {
                self.d = Direction::West;
            }
            Direction::West => {
                self.d = Direction::North;
            }
        }
        self
    }

    #[must_use]
    pub fn turn_left(mut self) -> Self {
        match self.d {
            Direction::North => {
                self.d = Direction::West;
            }
            Direction::East => {
                self.d = Direction::North;
            }
            Direction::South => {
                self.d = Direction::East;
            }
            Direction::West => {
                self.d = Direction::South;
            }
        }
        self
    }

    #[must_use]
    pub fn advance(mut self) -> Self {
        match self.d {
            Direction::North => {
                self.y += 1;
            }
            Direction::East => {
                self.x += 1;
            }
            Direction::South => {
                self.y -= 1;
            }
            Direction::West => {
                self.x -= 1;
            }
        }
        self
    }

    /// This function handles input passed in. It capitalizes all characters and matches the letter to an action.
    #[must_use]
    pub fn instructions(mut self, instructions: &str) -> Self {
        // Input: Capitalize everything, no numbers, Allow only left, right and advance as chars.
        for char in instructions.to_uppercase().chars() {
            self = match char {
                'A' => self.advance(),
                'L' => self.turn_left(),
                'R' => self.turn_right(),
                _ => self,
            };
        }
        self
    }

    pub fn position(&self) -> (i32, i32) {
        (self.x, self.y)
    }

    pub fn direction(&self) -> &Direction {
        &self.d
    }
}

/// Left it up to user to play with the robot
fn main () {
    // Made the choice of not creating a mutable robot. All of the methods for the robot struct will handle that
}

#[test]
fn create_robot_at_origin_facing_north() {
    let robot = Robot::new(0, 0, Direction::North);
    assert_eq!(robot.position(), (0, 0));
    assert_eq!(robot.direction(), &Direction::North);
}

#[test]
fn create_robot_at_position_facing_south() {
    let robot = Robot::new(-1, -1, Direction::South);
    assert_eq!(robot.position(), (-1, -1));
    assert_eq!(robot.direction(), &Direction::South);
}

#[test]
fn turn_right_from_north() {
    let robot = Robot::new(0, 0, Direction::North);
    let robot = robot.turn_right();
    assert_eq!(robot.direction(), &Direction::East);
}

#[test]
fn turn_right_from_east() {
    let robot = Robot::new(0, 0, Direction::East);
    let robot = robot.turn_right();
    assert_eq!(robot.direction(), &Direction::South);
}

#[test]
fn turn_right_from_south() {
    let robot = Robot::new(0, 0, Direction::South);
    let robot = robot.turn_right();
    assert_eq!(robot.direction(), &Direction::West);
}

#[test]
fn turn_right_from_west() {
    let robot = Robot::new(0, 0, Direction::West);
    let robot = robot.turn_right();
    assert_eq!(robot.direction(), &Direction::North);
}

#[test]
fn turn_left_from_north() {
    let robot = Robot::new(0, 0, Direction::North);
    let robot = robot.turn_left();
    assert_eq!(robot.direction(), &Direction::West);
}

#[test]
fn turn_left_from_west() {
    let robot = Robot::new(0, 0, Direction::West);
    let robot = robot.turn_left();
    assert_eq!(robot.direction(), &Direction::South);
}

#[test]
fn turn_left_from_south() {
    let robot = Robot::new(0, 0, Direction::South);
    let robot = robot.turn_left();
    assert_eq!(robot.direction(), &Direction::East);
}

#[test]
fn turn_left_from_east() {
    let robot = Robot::new(0, 0, Direction::East);
    let robot = robot.turn_left();
    assert_eq!(robot.direction(), &Direction::North);
}

#[test]
fn advance_facing_north() {
    let robot = Robot::new(0, 0, Direction::North);
    let robot = robot.advance();
    assert_eq!(robot.position(), (0, 1));
}

#[test]
fn advance_facing_east() {
    let robot = Robot::new(0, 0, Direction::East);
    let robot = robot.advance();
    assert_eq!(robot.position(), (1, 0));
}

#[test]
fn advance_facing_south() {
    let robot = Robot::new(0, 0, Direction::South);
    let robot = robot.advance();
    assert_eq!(robot.position(), (0, -1));
}

#[test]
fn advance_facing_west() {
    let robot = Robot::new(0, 0, Direction::West);
    let robot = robot.advance();
    assert_eq!(robot.position(), (-1, 0));
}

#[test]
fn instructions_move_robot_forward() {
    let robot = Robot::new(0, 0, Direction::North);
    let robot = robot.instructions("A");
    assert_eq!(robot.position(), (0, 1));
}

#[test]
fn instructions_turn_robot_left() {
    let robot = Robot::new(0, 0, Direction::North);
    let robot = robot.instructions("L");
    assert_eq!(robot.direction(), &Direction::West);
}

#[test]
fn instructions_turn_robot_right() {
    let robot = Robot::new(0, 0, Direction::North);
    let robot = robot.instructions("R");
    assert_eq!(robot.direction(), &Direction::East);
}

#[test]
fn instructions_move_and_turn_robot() {
    let robot = Robot::new(0, 0, Direction::North);
    let robot = robot.instructions("LAA");
    assert_eq!(robot.position(), (-2, 0));
    assert_eq!(robot.direction(), &Direction::West);
}

#[test]
fn instructions_can_be_lowercase() {
    let robot = Robot::new(0, 0, Direction::North);
    let robot = robot.instructions("laa");
    assert_eq!(robot.position(), (-2, 0));
    assert_eq!(robot.direction(), &Direction::West);
}
