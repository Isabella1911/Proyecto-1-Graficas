use raylib::prelude::*;
use std::collections::HashMap;

pub struct TextureManager {
    pub images: HashMap<String, Image>,
}

impl TextureManager {
    pub fn new(_rl: &mut RaylibHandle) -> Self {
        let mut images = HashMap::new();

        // ==========================
        // Texturas del templo
        // ==========================
        // Asegúrate de crear estos PNG en la carpeta assets/:
        //  - assets/wall_main.png
        //  - assets/wall_light.png
        //  - assets/wall_gate.png
        //  - assets/floor_temple.png
        //  - assets/ceil_temple.png
        let temple_textures = vec![
            ("WALL_MAIN",    "assets/wall_main.png"),
            ("WALL_LIGHT",   "assets/wall_light.png"),
            ("WALL_GATE",    "assets/wall_gate.png"),
            ("FLOOR_TEMPLE", "assets/floor_temple.png"),
            ("CEIL_TEMPLE",  "assets/ceil_temple.png"),
        ];

        for (key, path) in temple_textures {
            match Image::load_image(path) {
                Ok(image) => {
                    images.insert(key.to_string(), image);
                    println!("Textura cargada: {} -> {}", key, path);
                }
                Err(e) => {
                    eprintln!(
                        "Error cargando textura {} ({}): {:?}",
                        key, path, e
                    );
                }
            }
        }

        // ==========================
        // Sprites del juego
        // ==========================
        // F = enemigo, C = cofre, T = worker
        let sprite_list = vec![
            ("F", "assets/enemy.png"),
            ("C", "assets/chest.png"),
            ("T", "assets/worker.png"),
        ];

        for (key, path) in sprite_list {
            match Image::load_image(path) {
                Ok(img) => {
                    images.insert(key.to_string(), img);
                    println!("Sprite cargado: {} -> {}", key, path);
                }
                Err(e) => {
                    eprintln!("No se encontró sprite {} ({}): {:?}", key, path, e);
                }
            }
        }

        // Extras opcionales (si existen)
        let extra = vec![
            ("player_anim", "assets/player_anim.png"),
            ("enemy_alt",   "assets/enemy_alt.png"),
        ];

        for (key, path) in extra {
            if let Ok(img) = Image::load_image(path) {
                images.insert(key.to_string(), img);
                println!("Sprite extra cargado: {} -> {}", key, path);
            }
        }

        TextureManager { images }
    }

    pub fn get(&self, name: &str) -> Option<&Image> {
        self.images.get(name)
    }
}
