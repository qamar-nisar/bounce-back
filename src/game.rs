use macroquad::prelude::*;
use crate::{player::{PlayerController, INPUT_LEFT, INPUT_RIGHT}, world_map::WorldMap, level::LevelData};

#[derive(Clone, Copy, PartialEq, Eq)]
enum Screen { Menu, Levels, Playing, Paused, Complete, GameOver, Victory, Help, Records }

pub struct BounceBackGame {
    pub player: PlayerController,
    pub world: WorldMap,
    levels: Vec<LevelData>,
    screen: Screen,
    fixed_accumulator: f32,
    tick: u64,
    level: usize,
    score: u32,
    best_score: u32,
    lives: u8,
    menu_selection: usize,
    level_selection: usize,
    touch_jump_held: bool,
}

impl BounceBackGame {
    pub fn new(levels: Vec<LevelData>) -> Self {
        let world = levels.first().map(WorldMap::from_level).unwrap_or_else(WorldMap::fallback);
        let mut game = Self {
            player: PlayerController::new(32.0, 80.0), world, levels,
            screen: Screen::Menu, fixed_accumulator: 0.0, tick: 0,
            level: 0, score: 0, best_score: 0, lives: 3, menu_selection: 0, level_selection: 0,
            touch_jump_held: false,
        };
        game.reset_level();
        game
    }

    fn reset_level(&mut self) {
        self.world = self.levels.get(self.level).map(WorldMap::from_level).unwrap_or_else(WorldMap::fallback);
        self.respawn();
    }

    fn respawn(&mut self) {
        let (x, y) = self.world.spawn_position();
        self.player.reset(x, y);
        self.fixed_accumulator = 0.0;
    }

    fn start(&mut self, level: usize) {
        self.level = if self.levels.is_empty() { 0 } else { level.min(self.levels.len() - 1) };
        self.score = 0; self.lives = 3; self.tick = 0;
        self.reset_level(); self.screen = Screen::Playing;
    }

    fn advance_level(&mut self) {
        self.level += 1;
        self.reset_level();
        self.screen = Screen::Playing;
    }

    pub fn update(&mut self, dt: f32) {
        let (mx, my) = mouse_position();
        let just_touched = is_mouse_button_pressed(MouseButton::Left);
        let active_touches: Vec<_> = touches().into_iter().filter(|touch| matches!(touch.phase, TouchPhase::Started | TouchPhase::Stationary | TouchPhase::Moved)).collect();
        let enter = is_key_pressed(KeyCode::Enter) || is_key_pressed(KeyCode::Space);
        let back = is_key_pressed(KeyCode::Escape) || is_key_pressed(KeyCode::Backspace);

        match self.screen {
            Screen::Menu => {
                if is_key_pressed(KeyCode::Down) { self.menu_selection = (self.menu_selection + 1) % 4; }
                if is_key_pressed(KeyCode::Up) { self.menu_selection = (self.menu_selection + 3) % 4; }
                if just_touched {
                    if my > screen_height() * 0.43 && my < screen_height() * 0.53 { self.menu_selection = 0; self.start(0); }
                    else if my > screen_height() * 0.53 && my < screen_height() * 0.61 { self.screen = Screen::Levels; }
                    else if my > screen_height() * 0.61 && my < screen_height() * 0.69 { self.screen = Screen::Help; }
                    else if my > screen_height() * 0.69 && my < screen_height() * 0.80 { self.screen = Screen::Records; }
                }
                if enter { match self.menu_selection { 0 => self.start(0), 1 => self.screen = Screen::Levels, 2 => self.screen = Screen::Help, _ => self.screen = Screen::Records } }
            }
            Screen::Levels => {
                let n = self.levels.len().max(1);
                if is_key_pressed(KeyCode::Right) { self.level_selection = (self.level_selection + 1) % n; }
                if is_key_pressed(KeyCode::Left) { self.level_selection = (self.level_selection + n - 1) % n; }
                if just_touched && my > screen_height()*0.57 && my < screen_height()*0.69 {
                    if mx < screen_width()*0.34 { self.level_selection=(self.level_selection+n-1)%n; }
                    else if mx > screen_width()*0.66 { self.level_selection=(self.level_selection+1)%n; }
                }
                if back { self.screen = Screen::Menu; }
                if enter || just_touched && my > screen_height() * 0.70 { self.start(self.level_selection); }
            }
            Screen::Playing => {
                let ui_scale=(screen_height()/540.0).clamp(1.0,3.0);
                if back || just_touched && mx > screen_width()-74.0*ui_scale && my < 54.0*ui_scale { self.screen = Screen::Paused; return; }
                let left = is_key_down(KeyCode::Left) || is_key_down(KeyCode::A) || is_key_down(KeyCode::Key4) || is_key_down(KeyCode::Kp4)
                    || is_mouse_button_down(MouseButton::Left) && my > screen_height()*0.55 && mx < screen_width() * 0.34;
                let right = is_key_down(KeyCode::Right) || is_key_down(KeyCode::D) || is_key_down(KeyCode::Key6) || is_key_down(KeyCode::Kp6)
                    || is_mouse_button_down(MouseButton::Left) && my > screen_height()*0.55 && mx >= screen_width() * 0.34 && mx < screen_width() * 0.67;
                let jump_key = is_key_pressed(KeyCode::Up) || is_key_pressed(KeyCode::W) || is_key_pressed(KeyCode::Space)
                    || is_key_pressed(KeyCode::Key2) || is_key_pressed(KeyCode::Key5)
                    || is_key_pressed(KeyCode::Kp2) || is_key_pressed(KeyCode::Kp5);
                let touch_left = active_touches.iter().any(|t| t.position.y > screen_height()*0.55 && t.position.x < screen_width()*0.34);
                let touch_right = active_touches.iter().any(|t| t.position.y > screen_height()*0.55 && t.position.x >= screen_width()*0.34 && t.position.x < screen_width()*0.67);
                let left=left||touch_left; let right=right||touch_right;
                let jump_touch = is_mouse_button_down(MouseButton::Left) && mx >= screen_width() * 0.67 && my > screen_height() * 0.55
                    || active_touches.iter().any(|t| t.position.x >= screen_width()*0.67 && t.position.y > screen_height()*0.55);
                if jump_key || jump_touch && !self.touch_jump_held { self.player.queue_jump(); }
                self.touch_jump_held = jump_touch;
                self.fixed_accumulator += dt.min(0.1);
                while self.fixed_accumulator >= 0.05 {
                    self.fixed_accumulator -= 0.05; self.tick += 1;
                    let input = match (left, right) { (true, false) => INPUT_LEFT, (false, true) => INPUT_RIGHT, _ => 0 };
                    self.player.update(input, &self.world, 0.05);
                    self.world.set_camera(self.player.x + 8.0, self.player.y + 8.0);
                    self.collect_under_player();
                    if self.world.player_in_hazard(self.player.x, self.player.y, self.player.width, self.player.height)
                        || self.player.y > self.world.height as f32 * 16.0 + 24.0 {
                        self.lose_life();
                    }
                    if self.screen == Screen::Playing && (!self.world.has_collectibles() || self.world.player_at_exit(self.player.x, self.player.y, self.player.width, self.player.height)) {
                        self.finish_level();
                    }
                    if self.screen != Screen::Playing { break; }
                }
            }
            Screen::Paused => {
                if back || enter || just_touched { self.screen = Screen::Playing; }
                if just_touched && my > screen_height() * 0.62 { self.best_score=self.best_score.max(self.score); self.screen = Screen::Menu; }
            }
            Screen::Complete | Screen::GameOver | Screen::Victory => {
                if enter || just_touched { if self.screen == Screen::Complete { self.advance_level(); } else { self.screen = Screen::Menu; } }
                if back { self.screen = Screen::Menu; }
            }
            Screen::Help | Screen::Records => if back || enter || just_touched { self.screen = Screen::Menu; },
        }
    }

    fn collect_under_player(&mut self) {
        let found = self.world.overlapping_tiles(self.player.x, self.player.y, self.player.width, self.player.height);
        for (x, y, tile) in found {
            let points = match tile & 0x7f { 34 => 200, 12 => 1000, 30 => 2500, 93 | 94 | 97 | 98 | 101 | 102 => 500, _ => 0 };
            if points > 0 { self.world.clear_tile(x, y); self.score += points; }
        }
    }

    fn lose_life(&mut self) {
        self.lives = self.lives.saturating_sub(1);
        if self.lives == 0 { self.best_score = self.best_score.max(self.score); self.screen = Screen::GameOver; }
        else { self.respawn(); }
    }

    fn finish_level(&mut self) {
        if self.level + 1 >= self.levels.len() { self.best_score = self.best_score.max(self.score); self.screen = Screen::Victory; }
        else { self.best_score = self.best_score.max(self.score); self.screen = Screen::Complete; }
    }

    pub fn draw(&self, frames: &[Texture2D], tile_art: &[Texture2D]) {
        clear_background(Color::from_rgba(9, 15, 31, 255));
        if matches!(self.screen, Screen::Playing | Screen::Paused | Screen::Complete | Screen::GameOver | Screen::Victory) {
            self.world.draw(tile_art); self.player.draw(&self.world, frames);
            let ui_scale=(screen_height()/540.0).clamp(1.0,3.0);
            draw_rectangle(0.0, 0.0, screen_width(), 38.0*ui_scale, Color::from_rgba(8, 13, 24, 225));
            draw_text(&format!("LEVEL {:02}     SCORE {:06}     LIVES {}", self.level + 1, self.score, self.lives), 18.0*ui_scale, 26.0*ui_scale, 20.0*ui_scale, WHITE);
            draw_text("||", screen_width()-45.0*ui_scale, 26.0*ui_scale, 21.0*ui_scale, LIGHTGRAY);
            self.draw_touch_controls();
        }
        match self.screen {
            Screen::Menu => self.draw_panel("BOUNCE BACK", &["START GAME", "LEVEL SELECT", "HOW TO PLAY", "RECORDS"], self.menu_selection),
            Screen::Levels => {
                self.draw_panel("CHOOSE A LEVEL", &["PREVIOUS / NEXT: ARROWS OR SIDES", "TAP CENTER OR PRESS ENTER TO PLAY", "ESC TO RETURN"], 0);
                draw_text(&format!("LEVEL {:02} / {:02}", self.level_selection + 1, self.levels.len().max(1)), screen_width()*0.5-105.0, screen_height()*0.62, 30.0, GOLD);
            }
            Screen::Paused => self.draw_panel("PAUSED", &["TAP OR PRESS ENTER TO RESUME", "TAP LOWER AREA TO QUIT TO MENU"], 0),
            Screen::Complete => self.draw_panel("LEVEL COMPLETE!", &["TAP OR PRESS ENTER FOR NEXT LEVEL", "ESC TO MAIN MENU"], 0),
            Screen::GameOver => self.draw_panel("GAME OVER", &["TAP OR PRESS ENTER TO RETURN TO MENU", "TRY AGAIN FROM LEVEL SELECT"], 0),
            Screen::Victory => self.draw_panel("ALL LEVELS COMPLETE!", &["YOU FINISHED BOUNCE BACK", "TAP OR PRESS ENTER FOR MENU"], 0),
            Screen::Help => self.draw_panel("HOW TO PLAY", &["MOVE: ARROWS / A,D / 4,6", "JUMP: UP / W / SPACE / 2 / 5", "TOUCH: LEFT, CENTER AND RIGHT ZONES", "ESC: PAUSE"], 0),
            Screen::Records => self.draw_panel("RECORDS", &["BEST RUN SCORE"], 0),
            Screen::Playing => {}
        }
        if self.screen == Screen::Records { draw_text(&format!("{:06}", self.best_score), screen_width()*0.5-45.0, screen_height()*0.62, 34.0, GOLD); }
    }

    fn draw_panel(&self, title: &str, rows: &[&str], selected: usize) {
        let w = screen_width(); let h = screen_height();
        draw_rectangle(0.0, 0.0, w, h, Color::from_rgba(4, 9, 21, 220));
        let pw = (w * 0.78).min(720.0); let ph = (h * 0.80).min(470.0); let px = (w-pw)*0.5; let py = (h-ph)*0.5;
        draw_rectangle(px, py, pw, ph, Color::from_rgba(19, 34, 58, 250));
        draw_rectangle_lines(px, py, pw, ph, 2.0, Color::from_rgba(70, 188, 234, 255));
        let title_size = (h * 0.09).clamp(26.0, 48.0);
        let tw = measure_text(title, None, title_size as u16, 1.0).width;
        draw_text(title, (w-tw)*0.5, py+ph*0.28, title_size, Color::from_rgba(109, 224, 255, 255));
        let spacing = (h * 0.075).clamp(24.0, 38.0);
        let first = py + ph*0.49;
        for (i, row) in rows.iter().enumerate() {
            let color = if i == selected { GOLD } else { LIGHTGRAY };
            let size = if rows.len() > 3 { 22.0 } else { 19.0 };
            let tw = measure_text(row, None, size as u16, 1.0).width;
            draw_text(row, (w-tw)*0.5, first+i as f32*spacing, size, color);
        }
        draw_text("BOUNCE BACK  |  CLASSIC PLATFORMER", px+18.0, py+ph-14.0, 13.0, GRAY);
    }

    fn draw_touch_controls(&self) {
        if self.screen != Screen::Playing { return; }
        let w=screen_width(); let h=screen_height(); let s=(h/540.0).clamp(1.0,3.0); let y=h-70.0*s;
        let left_x=w*0.16; let right_x=w*0.50; let jump_x=w*0.90;
        draw_circle(left_x,y,31.0*s,Color::from_rgba(40,105,155,115));
        draw_text("<",left_x-9.0*s,y+9.0*s,27.0*s,WHITE);
        draw_circle(right_x,y,31.0*s,Color::from_rgba(40,105,155,115));
        draw_text(">",right_x-9.0*s,y+9.0*s,27.0*s,WHITE);
        draw_circle(jump_x,y,37.0*s,Color::from_rgba(43,157,124,165));
        draw_text("JUMP",jump_x-27.0*s,y+6.0*s,17.0*s,WHITE);
    }
}
