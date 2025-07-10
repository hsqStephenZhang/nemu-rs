#![no_std]
#![no_main]
use user_lib::{
    driver::{
        KeyCode, get_key, get_time, new_frame_buffer, screen_width_height, sync_frame, write_frame,
        write_pixel,
    },
    println,
};

const MAX_LENGTH: usize = 100;
const TILE_W: usize = 8;

#[derive(Debug, Clone, Copy, PartialEq)]
enum Direction {
    None,
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug, Clone, Copy)]
struct Dimension {
    width: i32,
    height: i32,
}

#[derive(Debug, Clone, Copy)]
struct Rect {
    top: i32,
    bottom: i32,
    left: i32,
    right: i32,
}

#[derive(Debug)]
struct Snake {
    body: [Point; MAX_LENGTH],
    length: usize,
    index: usize,
    dead: bool,
}

impl Snake {
    fn new() -> Self {
        Snake {
            body: [Point { x: 0, y: 0 }; MAX_LENGTH],
            length: 0,
            index: 0,
            dead: false,
        }
    }
}

fn draw_tile(y: i32, x: i32, color: u32) {
    let start_x = x * TILE_W as i32;
    let start_y = y * TILE_W as i32;

    for dy in 0..TILE_W {
        for dx in 0..TILE_W {
            write_pixel(
                (start_x + dx as i32) as usize,
                (start_y + dy as i32) as usize,
                color,
            );
        }
    }
}

// 读取键盘输入
fn read_key() -> Direction {
    if let Some((key, is_down)) = get_key() {
        if is_down {
            match key {
                KeyCode::UP => Direction::Up,
                KeyCode::DOWN => Direction::Down,
                KeyCode::LEFT => Direction::Left,
                KeyCode::RIGHT => Direction::Right,
                _ => Direction::None,
            }
        } else {
            Direction::None
        }
    } else {
        Direction::None
    }
}

fn create_food(game_size: Dimension) -> Point {
    // 使用简单的伪随机数生成器
    static mut SEED: u32 = 12345;
    unsafe {
        SEED = SEED.wrapping_mul(1103515245).wrapping_add(12345);
        Point {
            x: (SEED % game_size.width as u32) as i32,
            y: ((SEED >> 16) % game_size.height as u32) as i32,
        }
    }
}

fn print_board(board: Rect) {
    let color = 0xFF00FF00; // Green border

    // Top and bottom borders
    for i in board.left..=board.right {
        draw_tile(board.top, i, color);
        draw_tile(board.bottom, i, color);
    }

    // Left and right borders
    for i in board.top..=board.bottom {
        draw_tile(i, board.left, color);
        draw_tile(i, board.right, color);
    }
}

fn print_food(food: Point, board: Rect) {
    draw_tile(food.y + board.top + 1, food.x + board.left + 1, 0xFF0000FF); // Blue food
}

fn print_head(snake: &Snake, board: Rect) {
    let head = &snake.body[snake.index];
    draw_tile(head.y + board.top + 1, head.x + board.left + 1, 0xFFFF0000); // Red head
}

fn clear_tail(snake: &Snake, board: Rect) {
    let t = if snake.index >= snake.length {
        snake.index - snake.length
    } else {
        snake.index + MAX_LENGTH - snake.length
    };

    draw_tile(
        snake.body[t].y + board.top + 1,
        snake.body[t].x + board.left + 1,
        0xFF000000, // Black (clear)
    );
}

fn move_snake(snake: &mut Snake, dir: Direction) {
    let mut p = snake.body[snake.index];

    match dir {
        Direction::Left => p.x -= 1,
        Direction::Down => p.y += 1,
        Direction::Right => p.x += 1,
        Direction::Up => p.y -= 1,
        Direction::None => {}
    }

    snake.index = (snake.index + 1) % MAX_LENGTH;
    snake.body[snake.index] = p;
}

fn is_dead(snake: &Snake, game_size: Dimension) -> bool {
    let head = snake.body[snake.index];

    // Check wall collision
    if head.x < 0 || head.x >= game_size.width || head.y < 0 || head.y >= game_size.height {
        return true;
    }

    // Check self collision
    for i in 1..snake.length {
        let j = if snake.index >= i {
            snake.index - i
        } else {
            snake.index + MAX_LENGTH - i
        };

        if head.x == snake.body[j].x && head.y == snake.body[j].y {
            return true;
        }
    }

    false
}

fn has_food(snake: &Snake, food: Point) -> bool {
    let head = snake.body[snake.index];
    head.x == food.x && head.y == food.y
}

#[unsafe(no_mangle)]
fn main() -> i32 {
    let mut frame = new_frame_buffer();

    // Clear screen to black
    for pixel in frame.chunks_exact_mut(4) {
        pixel[0] = 0x00; // B
        pixel[1] = 0x00; // G
        pixel[2] = 0x00; // R
        pixel[3] = 0x00; // A
    }
    write_frame(&frame);

    let mut snake = Snake::new();
    let (screen_width, screen_height) = screen_width_height();

    let screen = Dimension {
        width: (screen_width / TILE_W) as i32,
        height: (screen_height / TILE_W) as i32,
    };

    let game_size = Dimension {
        width: screen.width - 2,
        height: screen.height - 2,
    };

    // Initialize snake
    snake.body[0] = Point {
        x: game_size.width / 2,
        y: game_size.height / 2,
    };
    snake.body[1] = Point {
        x: game_size.width / 2,
        y: game_size.height / 2 + 1,
    };
    snake.length = 2;
    snake.index = 1;

    let board = Rect {
        left: screen.width / 2 - game_size.width / 2 - 1,
        right: screen.width / 2 - game_size.width / 2 - 1 + game_size.width + 1,
        top: screen.height / 2 - game_size.height / 2 - 1,
        bottom: screen.height / 2 - game_size.height / 2 - 1 + game_size.height + 1,
    };

    print_board(board);

    let mut food = create_food(game_size);
    print_food(food, board);

    let mut dir = Direction::Right;

    loop {
        print_head(&snake, board);
        clear_tail(&snake, board);

        // 处理键盘输入
        let move_dir = read_key();

        // Update direction based on input
        match move_dir {
            Direction::Up if dir != Direction::Down => dir = move_dir,
            Direction::Down if dir != Direction::Up => dir = move_dir,
            Direction::Left if dir != Direction::Right => dir = move_dir,
            Direction::Right if dir != Direction::Left => dir = move_dir,
            _ => {}
        }

        move_snake(&mut snake, dir);
        snake.dead = is_dead(&snake, game_size);

        if has_food(&snake, food) {
            snake.length += 1;
            food = create_food(game_size);
            print_food(food, board);
        }

        sync_frame();

        // Game speed control
        let sleep_time = if 100000 > snake.length * 5000 + 5000 {
            100000 - snake.length * 5000
        } else {
            5000
        };

        let current_time = get_time();
        while get_time() - current_time < sleep_time as u64 {}

        if snake.dead {
            break;
        }
    }

    println!("GAME OVER");
    println!("Press Q to Exit");

    // 等待Q键退出
    loop {
        if let Some((key, is_down)) = get_key() {
            if is_down && key == KeyCode::Q {
                break;
            }
        }
    }

    0
}
