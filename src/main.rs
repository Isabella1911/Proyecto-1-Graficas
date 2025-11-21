mod input;       // sigue en src/input.rs
mod renderer;    // src/renderer.rs (el que te pasé)
mod raycasting;  // carpeta src/raycasting
mod entities;    // carpeta src/entities
mod audio;      // src/audio.rs
mod game;       // src/game.rs

use crate::renderer::framebuffer::Framebuffer;
use crate::renderer::{render_world_2d, render_world_3d, draw_sprite_billboard};
use crate::renderer::texture::TextureManager;
use crate::audio::Audio;

use crate::entities::player::Player;
use crate::entities::enemy::{Enemy, distance};

use crate::raycasting::maze::{find_player_start, load_maze, print_maze};
use crate::raycasting::caster::is_blocked_by_wall;

use crate::input::process_events;
use crate::game::run;

fn main() {
    if let Err(e) = game::run() {
        eprintln!("Error al ejecutar el juego: {e}");
    }
}
