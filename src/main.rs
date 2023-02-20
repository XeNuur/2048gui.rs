use ggez::{
    Context,
    ContextBuilder, 
    GameResult,
    graphics::{
        self,
        Color,
        Text,
    },
    event::{
        self,
        EventHandler
    },
    input::keyboard::{
        KeyCode,
        KeyInput
    },
};
use std::{env, path};
use rand::{Rng, SeedableRng, rngs::StdRng};

//configs
const MAP_SIZE:(usize, usize) = (4, 4);
const SCREEN_RES:(f32, f32) = (800., 600.);

//renderer options
const REN_BLOCK_GAP: f32 = 5.5;
const REN_BLOCK_SIZE: f32 = 100.;

fn main() {
    //get asstes forder
    let resource_dir = if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("res");
        path
    } else {
        path::PathBuf::from("./res")
    };
    // Make a Context.
    let (mut ctx, event_loop) = ContextBuilder::new("2048", "Cool Game Author")
        .window_mode(ggez::conf::WindowMode::default().dimensions(SCREEN_RES.0, SCREEN_RES.1))
        .window_setup(ggez::conf::WindowSetup::default().title("2048"))
        .add_resource_path(resource_dir)
        .build()
        .expect("aieee, could not create ggez context!");

    // Load/create resources such as images here.
    ctx.gfx.add_font(
        "0",
        graphics::FontData::from_path(&ctx, "/Font.ttf").expect("Font not found"),
    );

    // Create an instance of your event handler.
    // Usually, you should provide it with the Context object to
    // use when setting your game up.
    let my_game = MyGame::new(&mut ctx);

    // Run!
    event::run(ctx, event_loop, my_game);
}

struct MyGame {
    map: [[u32; MAP_SIZE.0]; MAP_SIZE.1],
    score: u32,

    is_gameover: bool
}

impl MyGame {
    pub fn new(_ctx: &mut Context) -> MyGame {
        let mut game = MyGame {
            map: [[0; MAP_SIZE.0]; MAP_SIZE.1],
            score: 0,
            is_gameover: false,
        };
        for _ in 0..2 {
            game.gen_new_tiles();
        }
        game
    }

    fn gen_new_tiles(&mut self) {
        loop {
            let mut rng = rand::thread_rng();
            let res = (
                rng.gen_range(0..MAP_SIZE.0),
                rng.gen_range(0..MAP_SIZE.1));

            if self.map[res.0][res.1] != 0 {
                continue;
            }
            self.map[res.0][res.1] = 2;
            break;
        }
    }

    fn compress(&mut self) -> bool {
        let mut is_changed = false;

        for i in 0..MAP_SIZE.0 {
            let mut pos = 0;
            for j in 0..MAP_SIZE.1 {
                if self.map[i][j] == 0 {
                    continue;
                }
                let tmp = self.map[i][pos];
                self.map[i][pos] = self.map[i][j];
                self.map[i][j] = tmp;

                if pos != j {
                    is_changed = true;
                }
                pos += 1;
            }
        }
        return is_changed;
    }

    fn merge(&mut self) -> bool{
        let mut changed = false;

        for i in 0..MAP_SIZE.0 {
            for j in 0..MAP_SIZE.1-1 { 
                if self.map[i][j] != 0 && self.map[i][j] == self.map[i][j+1] {
                    self.map[i][j] *= 2;
                    self.map[i][j+1] = 0;

                    self.score += self.map[i][j];
                    changed = true;
                }
            }
        }
        changed
    }

    fn reverse_map(&mut self) {
        let mut new_map = [[0; MAP_SIZE.0]; MAP_SIZE.1];
        for i in 0..MAP_SIZE.0 {
            for j in 0..MAP_SIZE.1 { 
                new_map[i][j] = self.map[i][MAP_SIZE.1-1-j];
            }
        }
        self.map = new_map;
    }
    
    fn transp_map(&mut self) {
        let mut new_map = [[0; MAP_SIZE.0]; MAP_SIZE.1];
        for i in 0..MAP_SIZE.0 {
            for j in 0..MAP_SIZE.1 { 
                new_map[i][j] = self.map[j][i];
            }
        }
        self.map = new_map;
    }

    fn move_title(&mut self) -> bool {
        let mut if_moved = self.compress();
        if_moved |= self.merge();
        if_moved |= self.compress();
        if_moved
    }

    fn chk_gameover(&self) -> bool {
        for (_, row) in self.map.iter().enumerate() {
            for (_, col) in row.iter().enumerate() {
                if *col == 0 {
                    return false;
                }
            }
        }

        for i in 0..MAP_SIZE.0 {
            for j in 0..MAP_SIZE.1-1 { 
                if self.map[i][j] == self.map[i][j+1] {
                    return false;
                }
            }
        }

        for i in 0..MAP_SIZE.0-1 {
            for j in 0..MAP_SIZE.1 { 
                if self.map[i][j] == self.map[i+1][j] {
                    return false;
                }
            }
        }
        true
    }
}

impl EventHandler for MyGame {
    fn update(&mut self, _ctx: &mut Context) -> GameResult { Ok(()) }

    fn key_down_event(&mut self, ctx: &mut Context, input: KeyInput, _repeat: bool) -> GameResult {
        let mut is_chrg = false;
        match input.keycode {
            Some(KeyCode::Up) => { is_chrg = self.move_title();}
            Some(KeyCode::Down) => {
                self.reverse_map();
                is_chrg = self.move_title();
                self.reverse_map();
            }
            Some(KeyCode::Left) => {
                self.transp_map();
                is_chrg = self.move_title();
                self.transp_map();
            }
            Some(KeyCode::Right) => {
                self.transp_map();
                self.reverse_map();
                is_chrg = self.move_title();
                self.reverse_map();
                self.transp_map();
            }

            Some(KeyCode::Space) => { 
                *self = MyGame::new(ctx);
            }
            _ => ()
        }
        if !is_chrg {
            return Ok(());
        }
        self.gen_new_tiles();
        if self.chk_gameover() {
            self.is_gameover = true;
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, Color::from([0.1, 0.1, 0.1, 1.]));
        // Draw code here...
        let start_pos: (f32, f32) = (
            SCREEN_RES.0/2.-(2.0*(REN_BLOCK_SIZE + REN_BLOCK_GAP)),
            SCREEN_RES.1/2.-(2.0*(REN_BLOCK_SIZE + REN_BLOCK_GAP))
        );
        let title_text_size = 32.;
        let title_text_ctx = if self.is_gameover {
            "Game Over (Press space to restart the game)"
        } else {"2048"};

        let mut text_title = Text::new(title_text_ctx);
        text_title.set_font("0").set_scale(title_text_size);
        canvas.draw(
            &text_title,
            graphics::DrawParam::new()
                .dest([0.; 2])
        );

        let mut score_title = Text::new("Score: ".to_owned()+&self.score.to_string());
        score_title.set_font("0").set_scale(28.);
        canvas.draw(
            &score_title,
            graphics::DrawParam::new()
                .dest([0., title_text_size])
        );

        for (x, row) in self.map.iter().enumerate() {
            for (y, col) in row.iter().enumerate() {
                let rect = graphics::Rect::new(
                    start_pos.0 + (x as f32)*(REN_BLOCK_SIZE + REN_BLOCK_GAP),
                    start_pos.1 + (y as f32)*(REN_BLOCK_SIZE + REN_BLOCK_GAP),
                    REN_BLOCK_SIZE,
                    REN_BLOCK_SIZE
                );
                let is_null = *col == 0;

                let ren_color = if is_null 
                {[0.3, 0.3, 0.3, 0.3]}
                else {
                    let mut rng: StdRng = SeedableRng::seed_from_u64(*col as u64);
                    [
                        rng.gen_range(0.2..1.),
                        rng.gen_range(0.2..1.),
                        rng.gen_range(0.2..1.),
                        rng.gen_range(0.2..1.)
                    ]
                };

                canvas.draw(
                    &graphics::Quad,
                    graphics::DrawParam::new()
                        .dest_rect(rect)
                        .color(ren_color),
                );

                if is_null{
                    continue;
                }

                let mut text = Text::new(col.to_string());
                text.set_font("0").set_scale(REN_BLOCK_SIZE/4.);

                canvas.draw(
                    &text,
                    graphics::DrawParam::new()
                        .dest([rect.x, rect.y])
                );
            }
        }
        canvas.finish(ctx)
    }
}

