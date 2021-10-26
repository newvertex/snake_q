// use ::rand::Rng;
use macroquad::prelude::*;

const ROWS: usize = 10;
const COLS: usize = 15;
const SCREEN_WIDTH: usize = 640;
const SCREEN_HEIGHT: usize = 480;
const TITLE: &str = "Snake Q";
const TILE_SIZE: f32 = 32.0;
const TILE_GAP: f32 = 2.0;
const FOOD_SIZE: f32 = 10.0;

#[derive(Clone, Copy, PartialEq, Eq)]
struct Point {
    x: isize,
    y: isize,
}

impl Point {
    fn new(x: isize, y: isize) -> Self {
        Self {
            x,
            y,
        }
    }

    fn set(&mut self, x: isize, y: isize) {
        self.x = x;
        self.y = y;
    }

    fn add(&mut self, x: isize, y: isize) {
        self.x += x;
        self.y += y;
    }

    fn sub(&mut self, x: isize, y: isize) {
        self.x -= x;
        self.y -= y;
    }
}

#[derive(PartialEq, Eq)]
enum Direction {
    Right,
    Down,
    Left,
    Up,
}

struct Snake {
    dir: Direction,
    body: Vec<Point>,
}

impl Snake {
    fn new(body: Vec<Point>, dir: Direction) -> Self {
        Self {
            body,
            dir,
        }
    }

    fn move_to(&mut self, dir: Direction) {
        use Direction::*;
        
        match dir {
            Right if self.dir != Left => self.dir = dir,
            Down if self.dir != Up => self.dir = dir,
            Left if self.dir != Right => self.dir = dir,
            Up if self.dir != Down => self.dir = dir,
            _ => {}
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Shape {
    Wall,
    Ground,
    Body,
    Head,
    Food,
}

fn load_level() -> (Vec<Vec<Shape>>, Snake) {
    let width = COLS;
    let height = ROWS;

    let snake_body = vec![Point::new(2, 5), Point::new(2, 4), Point::new(2, 3)];
    let snake = Snake::new(snake_body, Direction::Right);

    let mut level: Vec<Vec<Shape>> = vec![vec![Shape::Ground; width]; height];

    for x in 0..width {
        level[0][x] = Shape::Wall;
        level[height - 1][x] = Shape::Wall;
    }

    for y in 1..(height - 1) {
        level[y][0] = Shape::Wall;
        level[y][width - 1] = Shape::Wall;
    }

    (level, snake)
}

fn generate_food(level: &Vec<Vec<Shape>>, snake: &Snake) -> Point {
    let width = level[0].len();
    let height = level.len();
    let mut point = Point::new(1, 1);

    // let mut rng = ::rand::thread_rng();

    loop {
        let x = rand::gen_range(0, width);//rng.gen_range(0..width); //rand::gen_range(0, width);
        let y = rand::gen_range(0, height);//rng.gen_range(0..height);//rand::gen_range(0, height);
        point.set(x as isize, y as isize);

        if level[y][x] != Shape::Ground || snake.body.iter().any(|p| *p == point) {
            continue;
        }
        break;
    }

    point
}

fn update(level: &Vec<Vec<Shape>>, snake: &mut Snake, food: &mut Point, score: &mut i32, speed: &mut f64) -> bool {
    let mut snake_head = snake.body[0];
    match snake.dir {
        Direction::Right => snake_head.add(1, 0),
        Direction::Down => snake_head.add(0, 1),
        Direction::Left => snake_head.sub(1, 0),
        Direction::Up => snake_head.sub(0, 1),
    }

    snake.body.insert(0, snake_head);

    if snake_head != *food {
        snake.body.pop();
    } else {
        *food = generate_food(level, snake);
        *score += 1;
        *speed *= 0.9;
    }

    if snake.body[1..].contains(&snake_head) {
        println!("You're not eatable!\nGameOver!");
        return false;
    }

    if level[snake_head.y as usize][snake_head.x as usize] == Shape::Wall {
        println!("Look your steps!\nGameOver!");
        return false;
    }

    true
}

fn window_conf() -> Conf {
    Conf {
        window_title: "Snake Q".to_string(),
        window_width: 640,
        window_height: 480,
        fullscreen: false,
        window_resizable: false,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {

    let (level, mut snake) = load_level();
    let mut food = generate_food(&level, &snake);
    let mut score = 0;
    let mut last_update = get_time();
    let mut speed = 0.3;

    loop {
        // Update
        if is_key_pressed(KeyCode::Escape) {
            break;
        }

        if is_key_down(KeyCode::A) || is_key_down(KeyCode::Left) {
            snake.move_to(Direction::Left);
        } else if is_key_down(KeyCode::W) || is_key_down(KeyCode::Up) {
            snake.move_to(Direction::Up);
        } else if is_key_down(KeyCode::D) || is_key_down(KeyCode::Right) {
            snake.move_to(Direction::Right);
        } else if is_key_down(KeyCode::S) || is_key_down(KeyCode::Down) {
            snake.move_to(Direction::Down);
        }

        if get_time() - last_update > speed {
            last_update = get_time();

            if !update(&level, &mut snake, &mut food, &mut score, &mut speed) {
                break;
            }
        }

        // Render
        clear_background(GRAY);

        for (y, row) in level.iter().enumerate() {
            for (x, tile) in row.iter().enumerate() {
                let color = match tile {
                    Shape::Wall => BROWN,
                    Shape::Ground => GREEN,
                    _ => PINK
                };

                draw_rectangle((x as f32 * TILE_SIZE) + (TILE_GAP * x as f32), (y as f32 * TILE_SIZE) + (TILE_GAP * y as f32), TILE_SIZE, TILE_SIZE, color);
            }
        }

        draw_circle((food.x as f32 * TILE_SIZE) + (TILE_GAP * food.x as f32) + (TILE_SIZE / 2.0), (food.y as f32 * TILE_SIZE) + (TILE_GAP * food.y as f32) + (TILE_SIZE / 2.0), FOOD_SIZE, RED);

        for (i, point) in snake.body.iter().enumerate() {
            if i == 0 {
                draw_circle((point.x as f32 * TILE_SIZE) + (TILE_GAP * point.x as f32) + (TILE_SIZE / 2.0), (point.y as f32 * TILE_SIZE) + (TILE_GAP * point.y as f32) + (TILE_SIZE / 2.0), FOOD_SIZE, BLUE);
            } else {
                draw_rectangle((point.x as f32 * TILE_SIZE) + (TILE_GAP * point.x as f32), (point.y as f32 * TILE_SIZE) + (TILE_GAP * point.y as f32), TILE_SIZE, TILE_SIZE, YELLOW);
            }
        }

        let score_text = format!("Score: {}", score);
        draw_text(score_text.as_str(), 20.0, 20.0, 23.0, YELLOW);

        next_frame().await
    }
}
