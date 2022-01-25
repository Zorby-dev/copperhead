use rand::Rng;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use std::ops::Add;
use std::time::Duration;

const SCALLAR_UP: Vector = Vector { x: 0, y: -1 };
const SCALLAR_DOWN: Vector = Vector { x: 0, y: 1 };
const SCALLAR_LEFT: Vector = Vector { x: -1, y: 0 };
const SCALLAR_RIGHT: Vector = Vector { x: 1, y: 0 };

const WINDOW_SIZE: Vector = Vector {
    x: 800,
    y: 600
};
const CELL_SIZE: u32 = 10;
const GRID_SIZE: Vector = Vector {
    x: WINDOW_SIZE.x / CELL_SIZE as i32,
    y: WINDOW_SIZE.y / CELL_SIZE as i32
};

#[derive(Clone, PartialEq)]
struct Vector {
    x: i32,
    y: i32
}

impl Vector {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn random() -> Self {
        let mut rng = rand::thread_rng();

        Self {
            x: rng.gen_range(0..GRID_SIZE.x),
            y: rng.gen_range(0..GRID_SIZE.y)
        }
    }
}

impl Into<Option<Rect>> for &Vector {
    fn into(self) -> Option<Rect> {
        Some(Rect::new(self.x * CELL_SIZE as i32, self.y * CELL_SIZE as i32, CELL_SIZE, CELL_SIZE))
    }
}

impl Add<&Vector> for &Vector {
    type Output = Vector;

    fn add(self, rhs: &Vector) -> Self::Output {
        Vector {
            x: self.x + rhs.x,
            y: self.y + rhs.y
        }
    }
}

struct Apple {
    pos: Vector
}

struct Segment {
    pos: Vector
}

impl Segment {
    pub fn new(pos: Vector) -> Self {
        Self { pos }
    }
}

struct Snake {
    pub direction: Direction,
    segments: Vec<Segment>
}

impl Snake {
    pub fn new(head_pos: &Vector, direction: Direction, length: u32) -> Self {
        let mut cur_pos = head_pos.clone();
        let dir = direction.invert();

        let mut segments = Vec::with_capacity(length as usize);
        for i in 0..length {
            segments.push(
                Segment::new(cur_pos.clone())
            );
            cur_pos = &dir + &cur_pos;
        }
        
        Self {
            direction,
            segments
        }
    }

    pub fn head(&self) -> &Segment {
        &self.segments[0]
    }

    pub fn body(&self) -> &[Segment] {
        &self.segments[1..]
    }
}

impl Apple {
    pub fn new(pos: Vector) -> Self {
        Self { pos }
    }
}

#[derive(PartialEq)]
enum Direction {
    Left,
    Right,
    Up,
    Down
}

impl Direction {
    pub const fn scallar(&self) -> &'static Vector {
        match self {
            Direction::Left => &SCALLAR_LEFT,
            Direction::Right => &SCALLAR_RIGHT,
            Direction::Up => &SCALLAR_UP,
            Direction::Down => &SCALLAR_DOWN,
        }
    }

    pub fn invert(&self) -> Self {
        match self {
            Direction::Left => Self::Right,
            Direction::Right => Self::Left,
            Direction::Up => Self::Down,
            Direction::Down => Self::Up,
        }
    }
}

impl Add<&Vector> for &Direction {
    type Output = Vector;

    fn add(self, rhs: &Vector) -> Self::Output {
        rhs + self.scallar()
    }
}

fn is_in_bounds(vec: &Vector) -> bool {
    let x = vec.x;
    let y = vec.y;

    if x < 0 || y < 0 || x > GRID_SIZE.x || y > GRID_SIZE.y {
        false
    }
    else {
        true
    }
}

pub fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window("Snake", 800, 600)
        .position_centered()
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;

    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump()?;

    let mut snake = Snake::new(
        &Vector::new(10, 5),
        Direction::Right,
        5
    );

    let mut apple = Apple::new(Vector::random());

    'running: loop {
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        canvas.set_draw_color(Color::RGB(255, 0, 0));
        canvas.fill_rect(&apple.pos)?;

        canvas.set_draw_color(Color::RGB(0, 255, 0));
        for segment in &snake.segments {
            canvas.fill_rect(&segment.pos)?;
        }

        canvas.present();

        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 30));
        
        'event_loop: for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => {
                    break 'running;
                },
                Event::KeyDown {keycode, ..} => {
                    match keycode.unwrap() {
                        | Keycode::W
                        | Keycode::Up => {
                            let direction = Direction::Up;

                            if snake.direction != direction.invert() {
                                snake.direction = direction;
                            }

                            break 'event_loop;
                        },
                        | Keycode::A
                        | Keycode::Left => {
                            let direction = Direction::Left;

                            if snake.direction != direction.invert() {
                                snake.direction = direction;
                            }

                            break 'event_loop;
                        },
                        | Keycode::S
                        | Keycode::Down => {
                            let direction = Direction::Down;

                            if snake.direction != direction.invert() {
                                snake.direction = direction;
                            }

                            break 'event_loop;
                        },
                        | Keycode::D
                        | Keycode::Right => {
                            let direction = Direction::Right;

                            if snake.direction != direction.invert() {
                                snake.direction = direction;
                            }

                            break 'event_loop;
                        },
                        Keycode::Escape => {
                            break 'running;
                        },
                        _ => {}
                    }
                },
                _ => {}
            }
        }

        snake.segments.insert(0, Segment::new(&snake.direction + &snake.head().pos));
        if snake.head().pos == apple.pos {
            apple = Apple::new(Vector::random());
        } else {
            snake.segments.pop();
        }
        if !is_in_bounds(&snake.head().pos) {
            break 'running;
        }
        for segment in snake.body() {
            if &snake.head().pos == &segment.pos {
                break 'running;
            }
        }
    }

    Ok(())
}