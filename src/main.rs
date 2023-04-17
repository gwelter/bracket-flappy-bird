use bracket_lib::prelude::*;

const SCREEN_WIDTH: i32 = 80;
const SCREEN_HEIGHT: i32 = 50;
const FRAME_DURATION: f32 = 75.0;
const MAX_SPEED: f32 = 2.0;
const GRAVITY_SPEED: f32 = 0.4;

enum GameMode {
    Menu,
    Playing,
    GameOver,
}

struct Obstacle {
    x: i32,
    gap_y: i32,
    size: i32,
}

impl Obstacle {
    fn new(x: i32) -> Self {
        let mut random = RandomNumberGenerator::new();
        Self {
            x,
            gap_y: random.range(5, 40),
            size: i32::max(3, 15),
        }
    }
    fn check_collision(&self, player: &Player) -> bool {
        let half_size = self.size / 2;
        let does_x_match = player.x == self.x;
        let is_player_above_gap = player.y < self.gap_y - half_size;
        let is_player_below_gap = player.y > self.gap_y + half_size;
        does_x_match && (is_player_above_gap || is_player_below_gap)
    }
    fn render(&mut self, ctx: &mut BTerm, player_x: i32) {
        let screen_x = self.x - player_x;
        let half_size = self.size / 2;
        for i in 0..self.gap_y - half_size {
            ctx.set(screen_x, i, WHITE, BLACK, to_cp437('|'));
        }
        for y in self.gap_y + half_size..SCREEN_HEIGHT {
            ctx.set(screen_x, y, WHITE, BLACK, to_cp437('|'));
        }
        if screen_x < -20 {
            self.x = SCREEN_WIDTH + player_x;
            self.size = i32::max(3, self.size - 2);
        }
    }
}

struct Player {
    x: i32,
    y: i32,
    velocity: f32
}

impl Player {
    fn new(x: i32, y: i32) -> Self {
        Self {
            x,
            y,
            velocity: 0.0,
        }
    }
    fn render(&self, ctx: &mut BTerm) {
        ctx.set(5, self.y, YELLOW, BLACK, to_cp437('@'));
    }
    fn apply_gravity(&mut self) {
        if self.velocity < MAX_SPEED {
            self.velocity += GRAVITY_SPEED;
        }
        self.y += self.velocity as i32;
        self.x += 1;
        if self.y <0 {
            self.y = 0;
        }
    }
    fn flap(&mut self) {
        self.velocity = -3.0;
    }
}

struct State {
    player: Player,
    obstacles: Vec<Obstacle>,
    frame_time: f32,
    mode: GameMode,
}

impl State {
    fn new() -> Self {
        Self {
            player: Player::new(5, 25),
            obstacles: Vec::new(),
            frame_time: 0.0,
            mode: GameMode::Menu,
        }
    }
    fn restart(&mut self) {
        self.player = Player::new(5, 25);
        self.frame_time = 0.0;
        self.mode = GameMode::Playing;
        self.obstacles.clear();
        for n in 0..5 {
            self.obstacles.push(Obstacle::new(SCREEN_WIDTH + n * 20));
        }
    }
    fn main_menu(&mut self, ctx: &mut BTerm) {
        // Initial State
        ctx.cls();
        ctx.print_centered(5, "Welcome to Flappy Dragon!");
        ctx.print_centered(8, "(Space) to play the game");
        ctx.print_centered(9, "(Q)uit the game");

        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::Space => self.restart(),
                VirtualKeyCode::Q => ctx.quitting = true,
                _ => {}
            }
        }
    }
    fn play(&mut self, ctx: &mut BTerm) {
        ctx.cls_bg(NAVY);
        self.frame_time += ctx.frame_time_ms;
        if self.frame_time > FRAME_DURATION {
            self.frame_time = 0.0;
            self.player.apply_gravity();
        }
        if let Some(VirtualKeyCode::Space) = ctx.key {
            self.player.flap()
        }
        self.player.render(ctx);
        for obstacle in self.obstacles.iter_mut() {
            obstacle.render(ctx, self.player.x);
        }
        ctx.print(0, 0, "Press SPACE to flap");
        if self.player.y > SCREEN_HEIGHT || self.obstacles.iter().any(|o| o.check_collision(&self.player)) {
            self.mode = GameMode::GameOver;
        }
    }
    fn dead(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        ctx.print_centered(5, "You died!");
        ctx.print_centered(8, "(R) Restart");
        ctx.print_centered(9, "(Q) Quit the game");

        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::R => self.restart(),
                VirtualKeyCode::Q => ctx.quitting = true,
                _ => {}
            }
        }
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        match self.mode {
            GameMode::Menu => self.main_menu(ctx),
            GameMode::Playing => self.play(ctx),
            GameMode::GameOver => self.dead(ctx),
        }
    }
}

fn main() -> BError {
    let context = BTermBuilder::simple(SCREEN_WIDTH, SCREEN_HEIGHT).unwrap()
        .with_title("Flappy Dragon")
        .build()?;
    main_loop(context, State::new())
}
