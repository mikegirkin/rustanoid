use std::{alloc::GlobalAlloc, collections::HashSet};

use allegro::KeyCode;

use crate::geometry::*;

#[derive(Debug, Clone)]
pub struct Ball {
    pub position: Circle,
    pub movement_vector: FVector2d
}

#[derive(Debug, Clone)]
pub enum BrickVariety {
    Standard { color: i32 },
    Steel
}

impl BrickVariety {
    fn standard(color: i32) -> BrickVariety {
        BrickVariety::Standard { color }
    }
}

#[derive(Debug, Clone)]
pub struct Brick {
    pub position: Rectangle,
    pub variety: BrickVariety,
}

impl Brick {
    fn new(x1: f32, y1: f32, x2: f32, y2: f32, variety: BrickVariety) -> Brick {
        Brick {
            position: Rectangle::make_by_coords(x1, y1, x2, y2),
            variety: variety,
        }
    }
}

#[derive(Debug)]
pub struct Paddle {
    pub position: Rectangle,
    pub vector: FVector2d
}

impl Paddle {
    fn set_vector(&mut self, vector: FVector2d) {
        self.vector = vector
    }

    fn advance(&mut self, time_delta: f32, left_limit: f32, right_limit: f32) {
        let new_position = self.position.advance(&self.vector.clone().mul_scalar(time_delta));
        let limited_new_position = if new_position.left() < left_limit {
            new_position.with_left_at(left_limit)
        } else if new_position.right() > right_limit {
            new_position.with_right_at(right_limit)
        } else {
            new_position
        };
        self.position.mutable_set(limited_new_position);
    }
}

#[derive(Debug)]
pub struct BallCollision {
    pub ball: Ball,
    pub brick_index: usize,
    pub collision: Collision
}

impl BallCollision {
    pub fn new(ball: Ball, brick_index: usize, collision: Collision) -> BallCollision {
        BallCollision {
            ball,
            brick_index,
            collision,
        }
    }
}

#[derive(Debug)]
pub struct KeyboardState {
    pub move_left: bool,
    pub move_right: bool,
    pub fire: bool,
}

impl KeyboardState {
    pub fn from_key_set(set: &HashSet<KeyCode>) -> KeyboardState {
        KeyboardState {
            move_left: set.contains(&KeyCode::A),
            move_right: set.contains(&KeyCode::D),
            fire: set.contains(&KeyCode::Space),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum TimeState {
    Stopped,
    Running { last_update_time_sec: f64 },
    GameOver,
}

#[derive(Debug)]
pub struct GameState {
    pub balls: Vec<Ball>,
    pub bricks: Vec<Brick>,
    pub field: Rectangle,
    pub paddle: Paddle,
    pub time_state: TimeState,
    pub lives_left: i32,
}

impl GameState {
    const PADDLE_SPEED: f32 = 200.0;

    pub fn make_initial() -> GameState {
        GameState {
            balls: GameState::initial_balls_state(),
            bricks: GameState::initial_bricks_state(),
            field: Rectangle::make_by_coords(10.0, 10.0, 410.0, 460.0),
            paddle: GameState::initial_paddle_state(),
            time_state: TimeState::Stopped,
            lives_left: 3,
        }
    }

    fn initial_paddle_state() -> Paddle {
        Paddle {
            position: Rectangle::make_by_coords(50.0, 30.0, 100.0, 40.0),
            vector: FVector2d::new(0.0, 0.0),
        }
    }

    fn initial_balls_state() -> Vec<Ball> {
        vec![
            Ball {
                position: Circle::new(FPoint::new(70.0, 44.0), 4.0),
                movement_vector: FVector2d::new(-15.0, 120.0),
            }
        ]
    }

    fn initial_bricks_state() -> Vec<Brick> {
        vec!(
            Brick::new(10.0, 400.0, 50.0, 420.0, BrickVariety::standard(1)),
            Brick::new(50.0, 400.0, 90.0, 420.0, BrickVariety::standard(2)),
            Brick::new(10.0, 420.0, 50.0, 440.0, BrickVariety::standard(3)),
        )
    }

    pub fn tick(&mut self, timestamp_sec: f64, keyboard_state: KeyboardState) {
        self.paddle.set_vector(FVector2d::zero());
        if keyboard_state.move_left {
            self.paddle.set_vector(FVector2d::new(-GameState::PADDLE_SPEED, 0.0));
        }
        if keyboard_state.move_right {
            self.paddle.set_vector(FVector2d::new(GameState::PADDLE_SPEED, 0.0));
        }
        if keyboard_state.fire && self.time_state == TimeState::Stopped {
            self.time_state = TimeState::Running { last_update_time_sec: timestamp_sec }
        }

        match self.time_state {
            TimeState::Running { last_update_time_sec } => {
                self.execute_movement(timestamp_sec, last_update_time_sec);
            },
            TimeState::Stopped | TimeState::GameOver => (),
        }
    }

    fn execute_movement(&mut self, current_timestamp_sec: f64, last_update_time_sec: f64) {
        let time_delta = (current_timestamp_sec - last_update_time_sec) as f32;
        self.paddle.advance(time_delta, self.field.left(), self.field.right());
        self.handle_collisions(time_delta);
        self.handle_losing_ball();
        self.handle_game_over();

        match self.time_state {
            TimeState::Running{..} =>
                self.time_state = TimeState::Running {
                    last_update_time_sec: current_timestamp_sec
                },
            _ => (),
        }
    }

    fn handle_collisions(&mut self, time_delta: f32) -> () {
        for i in 0..self.balls.len() {
            let adjusted_vector = self.balls[i].movement_vector.clone().mul_scalar(time_delta);
            self.balls[i].position.center = self.balls[i].position.center.add(adjusted_vector);
            let paddle_collision = self.has_ball_collided_with_paddle(&self.balls[i]).map(|collision| {
                (collision, self.paddle.vector.clone())
            });
            let brick_collision = self.has_ball_collided_with_bricks(&self.balls[i]).map(|collision| {
                self.bricks.swap_remove(collision.brick_index);
                (collision.collision, FVector2d::zero())
            });
            let wall_collision = self.has_ball_coollided_with_wall(&self.balls[i]).map(|collision| {
                (collision, FVector2d::zero())
            });
            let collistion_opt = paddle_collision.or(brick_collision).or(wall_collision);
            match collistion_opt {
                Some((collision, other_object_vector)) => {
                    let new_movement_vector = new_vector_after_circle_collision(&self.balls[i].movement_vector, &collision, &other_object_vector);
                    self.balls[i].movement_vector = new_movement_vector;
                },
                None => {},
            }
        }
    }

    fn handle_losing_ball(&mut self) -> () {
        self.balls.retain_mut(|ball| {
           !GameState::has_ball_left_screen(ball)
        });
        if self.balls.len() == 0 {
            self.lives_left -= 1;
            self.paddle = GameState::initial_paddle_state();
            self.balls = GameState::initial_balls_state();
            self.time_state = TimeState::Stopped;
        }
    }

    fn handle_game_over(&mut self) -> () {
        if self.lives_left == 0 {
            self.time_state = TimeState::GameOver;
        }
    }

    fn has_ball_collided_with_bricks(&self, ball: &Ball) -> Option<BallCollision> {
        for (index, brick) in self.bricks.iter().enumerate() {
            let collision_opt = circle_rectangle_collision(&ball.position, &brick.position);
            match collision_opt {
                Some(collision) => {
                    return Some(BallCollision::new(ball.clone(), index, collision));
                }
                None => {}
            }
        }
        None
    }

    fn has_ball_coollided_with_wall(&self, ball: &Ball) -> Option<Collision> {
        if (ball.position.center.x - ball.position.radius) < self.field.left() {
            let collistion_point = FPoint::new(self.field.left(), ball.position.center.y);
            let normal = FVector2d::between(&collistion_point, &ball.position.center);
            Some(Collision {
                point: collistion_point,
                normal,
            })
        } else if (ball.position.center.x + ball.position.radius) > self.field.right() {
            let collistion_point = FPoint::new(self.field.right(), ball.position.center.y);
            let normal = FVector2d::between(&collistion_point, &ball.position.center);
            Some(Collision {
                point: collistion_point,
                normal,
            })
        } else if (ball.position.center.y + ball.position.radius) > self.field.top() {
            let collistion_point = FPoint::new(ball.position.center.x, self.field.top());
            let normal = FVector2d::between(&collistion_point, &ball.position.center);
            Some(Collision {
                point: collistion_point,
                normal,
            })
        } else {
            None
        }
    }

    fn has_ball_collided_with_paddle(&self, ball: &Ball) -> Option<Collision> {
        circle_rectangle_collision(&ball.position, &self.paddle.position).map(|collision| {
            let paddle_width = self.paddle.position.right() - self.paddle.position.left();
            let x_coord_relative_paddle_center = (collision.point.x - &self.paddle.position.left()) - (paddle_width / 2.0);
            let paddle_horisontal_vector = FVector2d::new(self.paddle.position.right() - self.paddle.position.left(), 0.0);
            let angle_rotation = if collision.point.y == self.paddle.position.top() && collision.normal.dot_product(&paddle_horisontal_vector).abs() < f32::EPSILON  {
                std::f32::consts::FRAC_PI_8 * (x_coord_relative_paddle_center / paddle_width)
            } else {
                0.0
            };
            Collision {
                point: collision.point,
                normal: collision.normal.rotate_clockwise(angle_rotation)
            }
        })
    }

    fn has_ball_left_screen(ball: &Ball) -> bool {
        ball.position.center.y < 0.0
    }
}
