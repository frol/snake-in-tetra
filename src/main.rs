use rand::Rng;
use std::collections::VecDeque;
use tetra::audio::Sound;
use tetra::graphics::{self, Color, DrawParams, Texture};
use tetra::input::{self, Key};
use tetra::math::Vec2;
use tetra::{Context, ContextBuilder, State};

const FRAMES_PER_SECOND: f64 = 15.0;
const SPRITE_SIZE: i32 = 20;
const SCREEN_SIZE: i32 = 21;
const INITIAL_TAIL: usize = 5;

type Position = Vec2<i32>;
type Direction = Vec2<i32>;

const START_POSITION: Position = Position::new(10, 10);

struct Apple {
    position: Position,
    texture: Texture,
}

impl Apple {
    fn new(ctx: &mut Context) -> tetra::Result<Self> {
        Ok(Self {
            position: Position::new(3, 3),
            texture: Texture::new(ctx, "./resources/red.png")?,
        })
    }

    fn draw(&self, ctx: &mut Context) {
        graphics::draw(
            ctx,
            &self.texture,
            DrawParams::new()
                .position(Vec2::new(
                    (self.position.x * SPRITE_SIZE) as f32,
                    (self.position.y * SPRITE_SIZE) as f32,
                ))
                .scale(Vec2::new(
                    (SPRITE_SIZE as f32) * 0.95,
                    (SPRITE_SIZE as f32) * 0.95,
                )),
        );
    }
}

struct Snake {
    direction: Direction,
    trail: VecDeque<Position>,
    texture: Texture,
    die: Sound,
}

impl Snake {
    fn new(ctx: &mut Context) -> tetra::Result<Self> {
        Ok(Self {
            direction: Direction::zero(),
            trail: vec![START_POSITION; INITIAL_TAIL].into_iter().collect(),
            texture: Texture::new(ctx, "./resources/green.png")?,
            die: Sound::new("./resources/splat.mp3")?,
        })
    }

    fn check_collision(&self, position: Position) -> bool {
        for pos in &self.trail {
            if *pos == position {
                return true;
            }
        }
        false
    }

    fn update(&mut self, ctx: &mut Context) {
        let head = self.trail.back().unwrap();
        let position = Position::new(
            (head.x + SCREEN_SIZE + self.direction.x) % SCREEN_SIZE,
            (head.y + SCREEN_SIZE + self.direction.y) % SCREEN_SIZE,
        );
        if self.direction != Vec2::zero() && self.check_collision(position) {
            self.die.play(ctx).unwrap();
            self.trail = vec![START_POSITION; INITIAL_TAIL].into_iter().collect();
            self.direction = Vec2::zero();
            return;
        }

        self.trail.push_back(position);
        self.trail.pop_front();
    }

    fn draw(&self, ctx: &mut Context) {
        for element in &self.trail {
            graphics::draw(
                ctx,
                &self.texture,
                DrawParams::new()
                    .position(Vec2::new(
                        (element.x * SPRITE_SIZE) as f32,
                        (element.y * SPRITE_SIZE) as f32,
                    ))
                    .scale(Vec2::new(
                        (SPRITE_SIZE as f32) * 0.95,
                        (SPRITE_SIZE as f32) * 0.95,
                    )),
            );
        }
    }
}

struct SnakeGame {
    apple: Apple,
    snake: Snake,
    eat: Sound,
}

impl SnakeGame {
    fn new(ctx: &mut Context) -> tetra::Result<SnakeGame> {
        Ok(SnakeGame {
            apple: Apple::new(ctx)?,
            snake: Snake::new(ctx)?,
            eat: Sound::new("./resources/chomp.mp3")?,
        })
    }

    fn handle_input(&mut self, ctx: &mut Context) {
        if input::is_key_pressed(ctx, Key::Left) && self.snake.direction.x == 0 {
            self.snake.direction = Direction::new(-1, 0);
        } else if input::is_key_pressed(ctx, Key::Right) && self.snake.direction.x == 0 {
            self.snake.direction = Direction::new(1, 0);
        } else if input::is_key_pressed(ctx, Key::Up) && self.snake.direction.y == 0 {
            self.snake.direction = Direction::new(0, -1);
        } else if input::is_key_pressed(ctx, Key::Down) && self.snake.direction.y == 0 {
            self.snake.direction = Direction::new(0, 1);
        }
    }

    fn generate_apple(&mut self) {
        loop {
            let position = Position::new(
                rand::thread_rng().gen_range(0, SCREEN_SIZE),
                rand::thread_rng().gen_range(0, SCREEN_SIZE),
            );
            if !self.snake.check_collision(position) {
                self.apple.position = position;
                break;
            }
        }
    }
}

impl State for SnakeGame {
    fn update(&mut self, ctx: &mut Context) -> tetra::Result {
        self.handle_input(ctx);

        self.snake.update(ctx);

        if self.snake.check_collision(self.apple.position) {
            self.eat.play(ctx)?;
            self.snake.trail.push_front(*self.snake.trail.back().unwrap());
            self.generate_apple();
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> tetra::Result {
        graphics::clear(ctx, Color::rgb(0.0, 0.0, 0.0));

        self.apple.draw(ctx);
        self.snake.draw(ctx);

        Ok(())
    }
}

fn main() -> tetra::Result {
    let width = (SPRITE_SIZE * SCREEN_SIZE) as i32;
    let height = (SPRITE_SIZE * SCREEN_SIZE) as i32;

    ContextBuilder::new("snake", width, height)
        .quit_on_escape(true)
        .timestep(tetra::time::Timestep::Fixed(FRAMES_PER_SECOND))
        .build()?
        .run(SnakeGame::new)
}
