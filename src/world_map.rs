use macroquad::prelude::*;
use crate::level::LevelData;

pub const TILE: f32 = 16.0;

pub fn display_scale() -> f32 { (screen_height() / 320.0).clamp(1.0, 3.0) }

#[derive(Clone)]
pub struct WorldMap {
    pub tiles: Vec<Vec<u8>>,
    pub width: usize,
    pub height: usize,
    pub camera_x: f32,
    pub camera_y: f32,
    pub spawn_x: f32,
    pub spawn_y: f32,
}

impl WorldMap {
    pub fn from_level(level: &LevelData) -> Self {
        let mut tiles = vec![vec![0u8; level.width]; level.height];
        for y in 0..level.height { for x in 0..level.width { tiles[y][x] = level.tiles[y*level.width+x]; } }
        let sx = level.meta.spawn_x_cell.max(0) as usize;
        let mut sy = level.height.saturating_sub(3);
        for row in (0..level.height).rev() {
            if level.tiles[row * level.width + sx.min(level.width.saturating_sub(1))] == 0 {
                sy = row; break;
            }
        }
        Self { tiles, width: level.width, height: level.height, camera_x:0.0, camera_y:0.0,
            spawn_x: sx as f32 * TILE, spawn_y: sy as f32 * TILE }
    }

    pub fn fallback() -> Self {
        let width=80; let height=20; let mut tiles=vec![vec![0u8;width];height];
        for y in 0..height { for x in 0..width { if y>=height-2 { tiles[y][x]=110; } } }
        tiles[height-3][8]=34;
        Self { tiles,width,height,camera_x:0.0,camera_y:0.0,spawn_x:48.0,spawn_y:80.0 }
    }

    pub fn get(&self, tx:i32, ty:i32) -> u8 {
        if tx<0 || ty<0 || tx>=self.width as i32 || ty>=self.height as i32 { return 110; }
        self.tiles[ty as usize][tx as usize]
    }

    /// The recovered map is not a simple boolean grid. The original engine
    /// masks tile codes with 0x7f and dispatches properties/shape tables.
    /// Until all property-table records are lifted, the dominant terrain codes
    /// observed in shipped levels are treated as solid here.
    pub fn is_solid(&self, tx:i32, ty:i32) -> bool {
        let t=self.get(tx,ty) & 0x7f;
        matches!(t, 85..=87 | 103 | 110..=116)
    }

    pub fn rect_collides(&self,x:f32,y:f32,w:f32,h:f32)->bool {
        let x0=(x/TILE).floor() as i32; let x1=((x+w-0.001)/TILE).floor() as i32;
        let y0=(y/TILE).floor() as i32; let y1=((y+h-0.001)/TILE).floor() as i32;
        for ty in y0..=y1 { for tx in x0..=x1 { if self.is_solid(tx,ty) { return true; } } }
        false
    }

    pub fn set_camera(&mut self,target_x:f32,target_y:f32){
        let scale=display_scale();
        let vw=screen_width()/scale; let vh=screen_height()/scale;
        let max_x=(self.width as f32*TILE-vw).max(0.0);
        let max_y=(self.height as f32*TILE-vh).max(0.0);
        self.camera_x=(target_x-vw*0.5).clamp(0.0,max_x);
        self.camera_y=(target_y-vh*0.5).clamp(0.0,max_y);
    }

    pub fn spawn_position(&self) -> (f32, f32) { (self.spawn_x, self.spawn_y) }

    pub fn overlapping_tiles(&self, x:f32, y:f32, w:f32, h:f32) -> Vec<(usize,usize,u8)> {
        let x0=(x/TILE).floor() as i32; let x1=((x+w-0.001)/TILE).floor() as i32;
        let y0=(y/TILE).floor() as i32; let y1=((y+h-0.001)/TILE).floor() as i32;
        let mut out=Vec::new();
        for ty in y0..=y1 { for tx in x0..=x1 {
            if tx>=0 && ty>=0 && tx<self.width as i32 && ty<self.height as i32 {
                let t=self.get(tx,ty)&0x7f;
                if t>0 { out.push((tx as usize,ty as usize,t)); }
            }
        }} out
    }

    pub fn clear_tile(&mut self, x:usize, y:usize) { if y<self.height && x<self.width { self.tiles[y][x]=0; } }

    pub fn player_in_hazard(&self, x:f32, y:f32, w:f32, h:f32) -> bool {
        self.overlapping_tiles(x,y,w,h).iter().any(|(_,_,t)| *t==39)
    }

    pub fn has_collectibles(&self) -> bool {
        self.tiles.iter().flatten().any(|t| matches!(*t & 0x7f, 12 | 30 | 34 | 93 | 94 | 97 | 98 | 101 | 102))
    }

    pub fn player_at_exit(&self, x:f32, y:f32, w:f32, h:f32) -> bool {
        let tiles=self.overlapping_tiles(x,y,w,h);
        if tiles.iter().any(|(_,_,t)| matches!(*t, 11 | 18 | 22)) { return !self.has_collectibles(); }
        !self.has_collectibles() && x > (self.width as f32 - 3.0) * TILE
    }

    pub fn draw(&self, tile_art: &[Texture2D]){
        let scale=display_scale(); let vw=screen_width()/scale; let vh=screen_height()/scale;
        let sx=(self.camera_x/TILE).floor().max(0.0) as i32; let sy=(self.camera_y/TILE).floor().max(0.0) as i32;
        let ex=((self.camera_x+vw)/TILE).ceil() as i32+1; let ey=((self.camera_y+vh)/TILE).ceil() as i32+1;
        for ty in sy..ey.min(self.height as i32){
            for tx in sx..ex.min(self.width as i32){
                let t=self.get(tx,ty)&0x7f; if t==0 { continue; }
                let x=(tx as f32*TILE-self.camera_x)*scale; let y=(ty as f32*TILE-self.camera_y)*scale;
                let color=match t {
                    12|30|34|93|94|97|98|101|102 => Color::from_rgba(241,188,66,255),
                    11|18|22 => Color::from_rgba(67,210,158,255),
                    39 => Color::from_rgba(229,73,82,255),
                    110..=116|85..=87 => Color::from_rgba(62,94,132,255),
                    _ => Color::from_rgba(103,116,139,210),
                };
                if let Some(texture)=tile_art.get(t as usize).filter(|_| t<104) {
                    draw_texture_ex(texture,x,y,WHITE,DrawTextureParams{dest_size:Some(vec2(TILE*scale,TILE*scale)),..Default::default()});
                } else if self.is_solid(tx,ty) { draw_rectangle(x,y,TILE*scale,TILE*scale,color); }
                else { draw_rectangle(x+3.0*scale,y+3.0*scale,(TILE-6.0)*scale,(TILE-6.0)*scale,color); }
            }
        }
    }
}
