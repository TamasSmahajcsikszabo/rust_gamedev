#![warn(clippy::pedantic)]
use bracket_lib::prelude::*;


const SCREEN_WIDTH: i32 = 80;
const SCREEN_HEIGHT: i32 = 50;
const FRAME_DURATION: f32 = 75.0;
const G: f32 = 2.81;
const CORR: f32 = 0.0095;
const TERMINAL_VELOCVITY: f32 = 3.5;
const DRAGON_FRAME: [u16; 6] = [ 64, 1, 2, 3, 2, 1 ];

//storing the game state
struct State {
    player: Player,
    frame_time: f32,
    obstacle: Obstacle,
    mode: GameMode,
    score: i32,
}

impl State {
    fn new() -> Self {
        State {
            player: Player::new(5, 25),
            frame_time: 0.0,
            obstacle: Obstacle::new(SCREEN_WIDTH, 0),
            mode: GameMode::Menu,
            score: 0,
        }
    }
    fn play(&mut self, ctx: &mut BTerm) {
        ctx.cls_bg(NAVY);
        self.frame_time += ctx.frame_time_ms;
        if self.frame_time > FRAME_DURATION {
            self.frame_time = 0.0;
            self.player.gravity_and_move();
            self.player.t += 1;
        }
        if let Some(VirtualKeyCode::Space) = ctx.key {
            self.player.flap();
        }
        self.player.render(ctx);
        ctx.print(0, 0, "Press SPACE to flap.");
        ctx.print(0, 1, &format!("Score: {}", self.score));
        ctx.print(0, 2, &format!("Player Fall Time: {}", self.player.t));
        ctx.print(0, 3, &format!("Player Velocity: {}", self.player.velocity));

        self.obstacle.render(ctx, self.player.x);
        if self.player.x  > self.obstacle.x {
            self.score += 1;
            self.obstacle = Obstacle::new(
                self.player.x + SCREEN_WIDTH, self.score
            );
        }

        if self.player.y > SCREEN_HEIGHT as f32 || self.obstacle.hit_obstacle(&self.player) {
            self.mode = GameMode::End;
        }
    }

    fn restart(&mut self) {
        self.player = Player::new(5, 25);
        self.frame_time = 0.0;
        self.mode = GameMode::Playing;
        self.obstacle = Obstacle::new(SCREEN_WIDTH, 0);
        self.score = 0;
    }

    fn main_menu(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        ctx.print_centered(5, "Welcone to Flappy Dragon");
        ctx.print_centered(8, "(P) Play Game");
        ctx.print_centered(9, "(Q) Quit Game");

        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::P => self.restart(),
                VirtualKeyCode::Q => ctx.quitting = true,
                _ => {}
            }
        }
    }

    fn dead(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        ctx.print_centered(5, "You are dead!");
        ctx.print_centered(6, &format!("You earned {} points", self.score));
        ctx.print_centered(8, "(P) Play Game");
        ctx.print_centered(9, "(Q) Quit Game");

        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::P => self.restart(),
                VirtualKeyCode::Q => ctx.quitting = true,
                _ => {}
            }
        }
    }
}

enum GameMode {
    Menu,
    Playing,
    End,
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        match self.mode {
            GameMode::Menu => self.main_menu(ctx),
            GameMode::End => self.dead(ctx),
            GameMode::Playing => self.play(ctx),
        }
    }
}


struct Player {
    x: i32,
    y: f32,
    velocity: f32,
    t: i32, // fall time
    frame: usize
}

impl Player {
    fn new(x: i32, y: i32) -> Self {
        Player {
            x,
            y: y as f32,
            velocity: 0.0,
            t: 0,
            frame: 0,
        }
    }

    fn render(&mut self, ctx: &mut BTerm) {
        // ctx.set(4, self.y, YELLOW, BLACK, to_cp437('@'));
        ctx.set_active_console(1);
        ctx.cls();
        ctx.set_fancy(
            PointF::new(4.0, self.y),
            1,
            Degrees::new(0.0),
            PointF::new(2.0, 2.0),
            WHITE,
            NAVY,
            DRAGON_FRAME[self.frame]
        );
        ctx.set_active_console(0);
    }

    fn gravity_and_move(&mut self) {
        if self.velocity < TERMINAL_VELOCVITY {
            let g_velocity = G * self.t as f32 * CORR;
            self.velocity += g_velocity;
            if self.velocity > TERMINAL_VELOCVITY {
                self.velocity = TERMINAL_VELOCVITY;
            }
        }
        self.y += self.velocity;
        self.x += 1;
        if self.y < 0.0 {
            self.y = 0.0;
            self.velocity = 0.0;
        }
        self.frame += 1;
        self.frame = self.frame % 6;
    }

    fn flap(&mut self) {
        self.velocity = -1.75;
        self.t = 0;
    }
}

struct Obstacle {
    x: i32,
    gap_y: i32,
    size: i32,
}

impl Obstacle {
    fn new(x: i32, score: i32) -> Self {
        let mut random = RandomNumberGenerator::new();
        Obstacle {
            x,
            gap_y: random.range(10, 40),
            size: i32::max(2, 20 - score),
        }
    }

    fn render(&mut self, ctx: &mut BTerm, player_x : i32) {
        let screen_x = self.x - player_x;
        let half_size = self.size / 2;

        //the ground
        for x in 0..SCREEN_WIDTH {
            ctx.set(x, SCREEN_HEIGHT-1, WHITE, WHITE, to_cp437('#'));
        }

        //top half
        for y in 0..self.gap_y - half_size {
            ctx.set(
                screen_x,
                y,
                WHITE,
                NAVY,
                179,
            );
        }

        //bottom half
        for y in self.gap_y + half_size..SCREEN_HEIGHT {
            ctx.set(
                screen_x,
                y,
                WHITE,
                NAVY,
                179,
            );
        }
    }

    fn hit_obstacle(&self, player: &Player) -> bool {
        let half_size = self.size / 2;
        let does_x_match = player.x == self.x;
        let player_above_gap = player.y < (self.gap_y - half_size) as f32;
        let player_below_gap = player.y > (self.gap_y + half_size) as f32;
        does_x_match && (player_above_gap || player_below_gap)
    }
}


//using Results as error handling
fn main() -> BError {
    // using the builder pattern - function chaining
    let context = BTermBuilder::new()
        .with_font("../resources/flappy32.png", 32, 32)
        .with_simple_console(SCREEN_WIDTH, SCREEN_HEIGHT, "../resources/flappy32.png")
        .with_fancy_console(SCREEN_WIDTH, SCREEN_HEIGHT, "../resources/flappy32.png")
        .with_title("Flappy Dragon enhanced")
        .with_tile_dimensions(16, 16)
        .build()?;

    // state machine concept - game as set of modes
    // no colon needed, let Rust return the function result
    main_loop(context, State::new())
}
