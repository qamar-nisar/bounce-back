use macroquad::prelude::*;
use crate::world_map::{WorldMap, TILE, display_scale};

pub const INPUT_LEFT: u8 = 4;
pub const INPUT_RIGHT: u8 = 6;
pub const INPUT_JUMP: u8 = 2;

pub struct PlayerController {
    pub x: f32, pub y: f32,
    pub width: f32, pub height: f32,
    pub vx: f32, pub vy: f32,
    pub direction: i32,
    pub animation: usize,
    pub grounded: bool,
    pub state: u8,
    pub jump_queued: bool,
}

impl PlayerController {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y, width: 16.0, height: 16.0, vx: 0.0, vy: 0.0, direction: 1, animation: 0, grounded: false, state: 0, jump_queued: false }
    }

    pub fn reset(&mut self, x: f32, y: f32) {
        self.x=x; self.y=y; self.vx=0.0; self.vy=0.0; self.direction=1;
        self.animation=0; self.grounded=false; self.state=0; self.jump_queued=false;
    }

    pub fn set_position(&mut self, x: i32, y: i32) { self.x = x as f32; self.y = y as f32; }
    pub fn set_direction(&mut self, d: i32) { if d != 0 { self.direction = d.signum(); } }
    pub fn queue_jump(&mut self) { self.jump_queued = true; }
    pub fn update(&mut self, input: u8, world: &WorldMap, dt: f32) {
        let left = input == INPUT_LEFT;
        let right = input == INPUT_RIGHT;
        let accel = 110.0;
        let max_speed = 70.0;
        let gravity = 360.0;
        let jump_speed = -155.0;

        if left && !right { self.vx -= accel * dt; self.direction = -1; }
        else if right && !left { self.vx += accel * dt; self.direction = 1; }
        else { self.vx *= 0.82_f32.powf(dt * 60.0); }
        self.vx = self.vx.clamp(-max_speed, max_speed);

        if self.jump_queued && self.grounded { self.vy = jump_speed; self.grounded = false; }
        self.jump_queued = false;
        self.vy += gravity * dt;

        self.move_axis(self.vx * dt, 0.0, world);
        self.move_axis(0.0, self.vy * dt, world);
        self.animation = if !self.grounded { 24 } else if self.vx.abs() > 8.0 { ((get_time()*10.0) as usize % 8) } else { 0 };
    }

    fn move_axis(&mut self, dx: f32, dy: f32, world: &WorldMap) {
        let nx = self.x + dx; let ny = self.y + dy;
        if !world.rect_collides(nx, ny, self.width, self.height) {
            self.x = nx; self.y = ny; if dy != 0.0 { self.grounded = false; }
            return;
        }
        if dx != 0.0 {
            let step = dx.signum(); let mut x = self.x;
            let mut remaining = dx.abs();
            while remaining > 0.0 {
                let s = remaining.min(1.0); let test = x + step*s;
                if world.rect_collides(test, self.y, self.width, self.height) { self.vx=0.0; break; }
                x=test; remaining-=s;
            }
            self.x=x;
        }
        if dy != 0.0 {
            let step = dy.signum(); let mut y=self.y; let mut remaining=dy.abs();
            while remaining>0.0 {
                let s=remaining.min(1.0); let test=y+step*s;
                if world.rect_collides(self.x, test, self.width, self.height) {
                    if step>0.0 { self.grounded=true; }
                    self.vy=0.0; break;
                }
                y=test; remaining-=s;
            }
            self.y=y;
        }
    }

    pub fn draw(&self, world: &WorldMap, player_frames: &[Texture2D]) {
        let scale=display_scale();
        let sx = (self.x - world.camera_x)*scale; let sy = (self.y - world.camera_y)*scale;
        if let Some(tex) = player_frames.get(self.animation.min(player_frames.len().saturating_sub(1))) {
            let mut params = DrawTextureParams { dest_size: Some(vec2(self.width*scale,self.height*scale)), ..Default::default() };
            if self.direction < 0 { params.flip_x = true; }
            draw_texture_ex(tex, sx, sy, WHITE, params);
        } else {
            draw_rectangle(sx, sy, self.width*scale, self.height*scale, YELLOW);
        }
    }
}

#[allow(dead_code)]
fn _tile_snap(v: f32) -> f32 { (v / TILE).floor() * TILE }
