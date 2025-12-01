extern crate allegro;
extern crate allegro_font;
extern crate allegro_ttf;
extern crate allegro_primitives;

use allegro::*;
use allegro_font::*;
use allegro_ttf::*;
use allegro_primitives::*;

use crate::geometry::*;
use crate::game_state::*;

pub const WORLD_SCREEN_SIZE: ISize = ISize {
    w: 640,
    h: 480
};

pub struct UIConfig {
    pub world_to_gfx_scale_factor: f32,
    pub screen: ISize
}

impl UIConfig {
    pub fn default() -> UIConfig {
        let scale = 2.5;
        UIConfig {
            world_to_gfx_scale_factor: scale,
            screen: ISize {
                w: (WORLD_SCREEN_SIZE.w as f32 * scale) as i32,
                h: (WORLD_SCREEN_SIZE.h as f32 * scale) as i32,
            }
        }
    }
}

trait WorldToGfx {
    fn world_to_gfx(&self, ui_config: &UIConfig) -> Self;
}

impl WorldToGfx for Rectangle {
    fn world_to_gfx(&self, ui_config: &UIConfig) -> Self {
        Rectangle {
            x1: self.x1 * ui_config.world_to_gfx_scale_factor,
            y1: ui_config.screen.h as f32 - self.y1 * ui_config.world_to_gfx_scale_factor,
            x2: self.x2 * ui_config.world_to_gfx_scale_factor,
            y2: ui_config.screen.h as f32 - self.y2 * ui_config.world_to_gfx_scale_factor
        }
    }
}

impl WorldToGfx for FPoint {
    fn world_to_gfx(&self, ui_config: &UIConfig) -> Self {
        FPoint {
            x: self.x * ui_config.world_to_gfx_scale_factor,
            y: ui_config.screen.h as f32 - self.y * ui_config.world_to_gfx_scale_factor
        }
    }
}

pub struct UI<'a> {
    ui_config: &'a UIConfig,
    core: &'a Core,
    font_addon: FontAddon,
    ttf_addon: TtfAddon,
    primitives_addon: PrimitivesAddon,
    pub font: Font,
    pub debug_font: Font,
    pub display: Display
}

impl<'a> UI<'a> {
    pub fn new(core: &'a Core, ui_config: &'a UIConfig) -> UI<'a> {
        let font_addon = FontAddon::init(&core).unwrap();
        let ttf_addon = TtfAddon::init(&font_addon).unwrap();
        let primitives_addon = PrimitivesAddon::init(&core).unwrap();
        let font = ttf_addon.load_ttf_font("data/Roboto-VariableFont_wdth,wght.ttf", (ui_config.world_to_gfx_scale_factor * -32.0) as i32, Flag::zero()).unwrap();
        let debug_font = ttf_addon.load_ttf_font("data/Roboto-VariableFont_wdth,wght.ttf", (ui_config.world_to_gfx_scale_factor * 8.0) as i32, Flag::zero()).unwrap();
        core.set_new_display_flags(WINDOWED | OPENGL);
        core.set_new_display_option(DisplayOption::Vsync, 0, DisplayOptionImportance::Require);
        let display = Display::new(&core, ui_config.screen.w, ui_config.screen.h).unwrap();
        UI {ui_config, core, font_addon, ttf_addon, primitives_addon, font, debug_font, display }
    }

    pub fn render(&self, game_state: &GameState) -> () {
        self.core.clear_to_color(Color::from_rgb_f(0.0, 0.0, 0.0));
        self.render_walls(&game_state);
        self.render_paddle(&game_state.paddle);
        self.render_bricks(&game_state.bricks);
        self.render_balls(&game_state.balls);
        self.render_game_over(&game_state);
        self.render_debug(&game_state);
        self.core.flip_display();
    }

    fn darken(&self, color: Color, percentage: u32) -> Color {
        let valid_percentage = percentage.clamp(0, 100);
        let (old_r, old_g, old_b) = color.to_rgb();
        let new_r = (((old_r as u32) * valid_percentage) / 100) as u8;
        let new_g = (((old_g as u32) * valid_percentage) / 100) as u8;
        let new_b = (((old_b as u32) * valid_percentage) / 100) as u8;
        Color::from_rgb(new_r, new_g, new_b)
    }

    fn render_filled_rect(&self, rect: &Rectangle, color: Color) {
        self.primitives_addon.draw_filled_rectangle(rect.x1, rect.y1, rect.x2, rect.y2, color);
    }

    fn render_paddle(&self, paddle: &Paddle) {
        let gfx_rect = paddle.position.world_to_gfx(&self.ui_config);
        self.render_filled_rect(&gfx_rect, Color::from_rgb(200, 200, 0));
    }

    fn render_bricks(&self, bricks: &Vec<Brick>) {
        for brick in bricks {
            let color = match brick {
                Brick { variety: BrickVariety::Standard { color }, .. } =>
                    match color {
                        1 => Color::from_rgb(255, 0, 0),
                        2 => Color::from_rgb(0, 255, 0),
                        3 => Color::from_rgb(50, 50, 255),
                        _ => Color::from_rgb(0, 0, 0),
                    },
                Brick { variety: BrickVariety::Steel, .. } => Color::from_rgb(150, 150, 150)
            };
            let gfx_outer_rect = brick.position.world_to_gfx(&self.ui_config);
            self.render_filled_rect(&gfx_outer_rect, self.darken(color, 50));
            let gfx_inner_rect = brick.position.grow(-1.0).world_to_gfx(&self.ui_config);
            self.render_filled_rect(&gfx_inner_rect, color);
        }
    }

    fn render_walls(&self, game_state: &GameState) {
        let gfx_outer_rect = Rectangle::make_by_coords(
            game_state.field.x1 - 8.0,
            game_state.field.y1,
            game_state.field.x2 + 8.0,
            game_state.field.y2 + 8.0
        ).world_to_gfx(&self.ui_config);
        self.render_filled_rect(&gfx_outer_rect, Color::from_rgb(200, 200, 200));
        let gfx_inner_rect = game_state.field.world_to_gfx(&self.ui_config);
        self.render_filled_rect(&gfx_inner_rect, Color::from_rgb(0, 0, 0));
    }

    fn render_balls(&self, balls: &Vec<Ball>) {
       for ball in balls.iter() {
           let center_point = ball.position.center.world_to_gfx(&self.ui_config);
           let radius = ball.position.radius * self.ui_config.world_to_gfx_scale_factor;
           let color = Color::from_rgb(255, 255, 255);
           self.primitives_addon.draw_filled_circle(center_point.x, center_point.y, radius, color);
       }
    }

    fn render_debug(&self, game_state: &GameState) {
        let text = format!("{:?}", game_state.balls[0].movement_vector.as_polar());
        self.core.draw_text(&self.debug_font, Color::from_rgb(255, 255, 255), 1060.0, 1000.0, FontAlign::Left, &text);
    }

    fn render_game_over(&self, game_state: &GameState) {
        if game_state.time_state == TimeState::GameOver {
            self.core.draw_text(&self.font, Color::from_rgb(230, 30, 30), 50.0, 100.0, FontAlign::Left, "GAME OVER");
        }
    }

}
