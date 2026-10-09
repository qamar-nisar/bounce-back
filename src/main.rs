mod packed_resource;
mod player;
mod world_map;
mod replay;
mod level;
mod game;

use macroquad::prelude::*;
use game::BounceBackGame;
use level::LevelData;

async fn load_player_frames() -> Vec<Texture2D> {
    let mut frames=Vec::new();
    for i in 1..=25usize {
        let path=format!("assets/png/b/{i:03}.png");
        if let Ok(texture)=load_texture(&path).await { texture.set_filter(FilterMode::Nearest); frames.push(texture); }
    }
    frames
}

async fn load_tile_art() -> Vec<Texture2D> {
    let mut art=Vec::new();
    for i in 0..104usize {
        let path=format!("assets/png/kjmGlobal_images_theme0/{i:03}.png");
        if let Ok(texture)=load_texture(&path).await { texture.set_filter(FilterMode::Nearest); art.push(texture); }
        else { break; }
    }
    art
}

async fn load_levels() -> Vec<LevelData> {
    let mut levels=Vec::new();
    for index in 0..22usize {
        let meta_path=format!("assets/levels/{:03}.bin",index*2);
        let map_path=format!("assets/levels/{:03}.bin",index*2+1);
        let Some(meta_bytes)=load_file(&meta_path).await.ok() else { continue; };
        let Some(map_bytes)=load_file(&map_path).await.ok() else { continue; };
        let Some(meta)=LevelData::decode_meta(&meta_bytes) else { continue; };
        let Ok(mut level)=LevelData::decode_map(&map_bytes) else { continue; };
        level.set_meta(meta);
        levels.push(level);
    }
    levels
}

fn window_conf() -> Conf {
    Conf {
        window_title: "Bounce Back".to_owned(),
        window_width: 960,
        window_height: 540,
        high_dpi: true,
        fullscreen: false,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let levels=load_levels().await;
    let mut game=BounceBackGame::new(levels);
    let frames=load_player_frames().await;
    let tile_art=load_tile_art().await;
    loop {
        game.update(get_frame_time());
        game.draw(&frames,&tile_art);
        next_frame().await;
    }
}
