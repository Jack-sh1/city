use macroquad::prelude::*;


// 游戏常量
const GRID_W: usize = 10;
const GRID_H: usize = 20;
const CELL: f32 = 30.0;
const BOARD_W: f32 = GRID_W as f32 * CELL;
// const BOARD_H: f32 = GRID_H as f32 * CELL;

// 颜色表
const COLORS: [Color; 8] = [
    BLACK, RED, GREEN, BLUE, YELLOW, MAGENTA, SKYBLUE, ORANGE,
];

// 7 种形状（旋转后的 4 个状态）
const SHAPES: [[[u8; 16]; 4]; 7] = [
    // I
    [
        [0,0,0,0, 1,1,1,1, 0,0,0,0, 0,0,0,0],
        [0,1,0,0, 0,1,0,0, 0,1,0,0, 0,1,0,0],
        [0,0,0,0, 0,0,0,0, 1,1,1,1, 0,0,0,0],
        [0,0,0,1, 0,0,0,1, 0,0,0,1, 0,0,0,1],
    ],
    // J
    [
        [1,0,0,0, 1,1,1,0, 0,0,0,0, 0,0,0,0],
        [0,1,1,0, 0,1,0,0, 0,1,0,0, 0,0,0,0],
        [0,0,0,0, 1,1,1,0, 0,0,1,0, 0,0,0,0],
        [0,1,0,0, 0,1,0,0, 1,1,0,0, 0,0,0,0],
    ],
    // L
    [
        [0,0,1,0, 1,1,1,0, 0,0,0,0, 0,0,0,0],
        [0,1,0,0, 0,1,0,0, 0,1,1,0, 0,0,0,0],
        [0,0,0,0, 1,1,1,0, 1,0,0,0, 0,0,0,0],
        [1,1,0,0, 0,1,0,0, 0,1,0,0, 0,0,0,0],
    ],
    // O
    [
        [0,1,1,0, 0,1,1,0, 0,0,0,0, 0,0,0,0],
        [0,1,1,0, 0,1,1,0, 0,0,0,0, 0,0,0,0],
        [0,1,1,0, 0,1,1,0, 0,0,0,0, 0,0,0,0],
        [0,1,1,0, 0,1,1,0, 0,0,0,0, 0,0,0,0],
    ],
    // S
    [
        [0,1,1,0, 1,1,0,0, 0,0,0,0, 0,0,0,0],
        [0,1,0,0, 0,1,1,0, 0,0,1,0, 0,0,0,0],
        [0,0,0,0, 0,1,1,0, 1,1,0,0, 0,0,0,0],
        [1,0,0,0, 1,1,0,0, 0,1,0,0, 0,0,0,0],
    ],
    // T
    [
        [0,1,0,0, 1,1,1,0, 0,0,0,0, 0,0,0,0],
        [0,1,0,0, 0,1,1,0, 0,1,0,0, 0,0,0,0],
        [0,0,0,0, 1,1,1,0, 0,1,0,0, 0,0,0,0],
        [0,1,0,0, 1,1,0,0, 0,1,0,0, 0,0,0,0],
    ],
    // Z
    [
        [1,1,0,0, 0,1,1,0, 0,0,0,0, 0,0,0,0],
        [0,0,1,0, 0,1,1,0, 0,1,0,0, 0,0,0,0],
        [0,0,0,0, 1,1,0,0, 0,1,1,0, 0,0,0,0],
        [0,1,0,0, 1,1,0,0, 1,0,0,0, 0,0,0,0],
    ],
];

#[derive(Clone, Copy)]
struct Piece {
    shape: usize,
    rot:   usize,
    x:     i32,
    y:     i32,
}

impl Piece {
    fn new(shape: usize) -> Self {
        Self { shape, rot: 0, x: 3, y: 0 }
    }
    fn cells(&self) -> impl Iterator<Item = (i32, i32)> + '_ {
        let s = &SHAPES[self.shape][self.rot];
        (0..4).flat_map(move |r| (0..4).map(move |c| (r as i32, c as i32)))
            .filter(move |&(r, c)| s[(r * 4 + c) as usize] != 0)
    }
    fn left(&self, grid: &Grid) -> Option<Piece> { self.shift(-1, 0, grid) }
    fn right(&self, grid: &Grid) -> Option<Piece> { self.shift(1, 0, grid) }
    fn down(&self, grid: &Grid) -> Option<Piece> { self.shift(0, 1, grid) }
    fn rotate(&self, grid: &Grid) -> Option<Piece> {
        let mut p = *self;
        p.rot = (p.rot + 1) % 4;
        if p.valid(grid) { Some(p) } else { None }
    }
    fn shift(&self, dx: i32, dy: i32, grid: &Grid) -> Option<Piece> {
        let mut p = *self;
        p.x += dx;
        p.y += dy;
        if p.valid(grid) { Some(p) } else { None }
    }
    fn valid(&self, grid: &Grid) -> bool {
        self.cells().all(|(r, c)| {
            let x = self.x + c;
            let y = self.y + r;
            x >= 0 && x < GRID_W as i32 && y < GRID_H as i32 && (y < 0 || grid[y as usize][x as usize] == 0)
        })
    }
    fn lock(&self, grid: &mut Grid) {
        for (r, c) in self.cells() {
            let x = self.x + c;
            let y = self.y + r;
            if y >= 0 {
                grid[y as usize][x as usize] = (self.shape + 1) as u8;
            }
        }
    }
}

type Grid = [[u8; GRID_W]; GRID_H];

fn clear_lines(grid: &mut Grid) -> usize {
    let mut lines = 0;
    for r in (0..GRID_H).rev() {
        if grid[r].iter().all(|&c| c != 0) {
            lines += 1;
            for y in (1..=r).rev() {
                grid[y] = grid[y - 1];
            }
            grid[0] = [0; GRID_W];
        }
    }
    lines
}

fn draw_grid(grid: &Grid) {
    for (y, row) in grid.iter().enumerate() {
        for (x, &cell) in row.iter().enumerate() {
            draw_rectangle(
                x as f32 * CELL,
                y as f32 * CELL,
                CELL - 1.0,
                CELL - 1.0,
                COLORS[cell as usize],
            );
        }
    }
}

fn draw_piece(piece: &Piece) {
    for (r, c) in piece.cells() {
        draw_rectangle(
            (piece.x + c) as f32 * CELL,
            (piece.y + r) as f32 * CELL,
            CELL - 1.0,
            CELL - 1.0,
            COLORS[piece.shape + 1],
        );
    }
}

fn next_piece() -> Piece {
    Piece::new(rand::gen_range(0, 7))
}

#[macroquad::main("Tetris in Rust")]
async fn main() {
    let mut grid = [[0u8; GRID_W]; GRID_H];
    let mut piece = next_piece();
    let mut next = next_piece();
    let mut score = 0u32;
    let mut fall_timer = 0.0;
    let fall_speed = 0.5; // 秒

    loop {
        // 按键
        if is_key_pressed(KeyCode::Left)  { if let Some(p) = piece.left(&grid)  { piece = p; } }
        if is_key_pressed(KeyCode::Right) { if let Some(p) = piece.right(&grid) { piece = p; } }
        if is_key_pressed(KeyCode::Down)  { if let Some(p) = piece.down(&grid)  { piece = p; } }
        if is_key_pressed(KeyCode::Up)    { if let Some(p) = piece.rotate(&grid) { piece = p; } }

        // 自动下落
        fall_timer += get_frame_time();
        if fall_timer > fall_speed {
            fall_timer = 0.0;
            if let Some(p) = piece.down(&grid) {
                piece = p;
            } else {
                piece.lock(&mut grid);
                let cleared = clear_lines(&mut grid);
                score += cleared as u32 * 100;
                piece = next;
                next = next_piece();
                if !piece.valid(&grid) { break; } // Game Over
            }
        }

        // 绘制
        clear_background(BLACK);
        draw_grid(&grid);
        draw_piece(&piece);

        // 侧边栏
        let text = format!("Score: {}", score);
        draw_text(&text, BOARD_W + 20.0, 30.0, 30.0, WHITE);
        draw_text("Next:", BOARD_W + 20.0, 80.0, 30.0, WHITE);
        draw_piece(&Piece { x: (GRID_W as i32 + 2), y: 3, ..next });

        next_frame().await;
    }

    // Game Over 画面
    loop {
        clear_background(BLACK);
        draw_text("GAME OVER", 100.0, 200.0, 60.0, RED);
        draw_text(&format!("Score: {}", score), 100.0, 280.0, 40.0, WHITE);
        draw_text("Press ESC to exit", 100.0, 340.0, 30.0, WHITE);
        next_frame().await;
    }
}