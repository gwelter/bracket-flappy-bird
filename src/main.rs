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
        ctx.set(self.x, self.y, YELLOW, BLACK, to_cp437('@'));
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
    frame_time: f32,
    mode: GameMode,
}

impl State {
    fn new() -> Self {
        Self {
            player: Player::new(5, 25),
            frame_time: 0.0,
            mode: GameMode::Menu,
        }
    }
    fn restart(&mut self) {
        self.player = Player::new(5, 25);
        self.frame_time = 0.0;
        self.mode = GameMode::Playing;
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
        ctx.print(0, 0, "Press SPACE to flap");
        if self.player.y > SCREEN_HEIGHT {
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
