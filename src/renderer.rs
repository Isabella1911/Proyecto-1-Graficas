use raylib::color::Color;
use raylib::math::Vector2;

use crate::renderer::framebuffer::Framebuffer;
use crate::renderer::texture::TextureManager;
use crate::entities::player::Player;
use crate::raycasting::maze::{Maze, get_cell_color};
use crate::raycasting::caster::{cast_ray, cast_ray_debug};
use crate::raycasting::line;

// ======================= 3D ==========================
pub fn render_world_3d(
    framebuffer: &mut Framebuffer,
    maze: &Maze,
    player: &Player,
    block_size: usize,
    textures: &TextureManager,
) {
    let ray_step: usize = 3;
    let num_rays = (framebuffer.width as usize + ray_step - 1) / ray_step;
    let hw = framebuffer.width as f32 / 2.0;
    let hh = framebuffer.height as f32 / 2.0;

    let distance_to_projection_plane: f32 =
        (framebuffer.width as f32 / (2.0f32 * (player.fov / 2.0f32).tan())).abs();

    for i in 0..num_rays {
        let screen_x = (i * ray_step) as i32;
        let current_ray = i as f32 / num_rays as f32;
        let ray_angle = player.a - (player.fov / 2.0f32) + (player.fov * current_ray);
        let intersect = cast_ray(maze, player, ray_angle, block_size);

        let safe_distance: f32 = intersect.distance.max(0.1f32);

        let stake_height = block_size as f32;
        let adjusted_height = (stake_height / safe_distance) * distance_to_projection_plane;

        let stake_top = (hh - (adjusted_height / 2.0f32)) as i32;
        let stake_bottom = (hh + (adjusted_height / 2.0f32)) as i32;

        // ----------------------- PAREDES -----------------------
        let cell_char = intersect.impact;
        let texture_key = match cell_char {
            'E' | '#' => "WALL_MAIN",
            'L' => "WALL_LIGHT",
            '$' => "WALL_GATE",
            _ => "WALL_MAIN",
        };

        if let Some(image) = textures.get(texture_key) {
            let pixel_data = image.get_image_data();
            let tex_w = image.width as usize;
            let tex_h = image.height as usize;

            for y in stake_top..stake_bottom {
                if y >= 0 && y < framebuffer.height {
                    let texture_y = ((y - stake_top) as f32
                        / (stake_bottom - stake_top) as f32)
                        * tex_h as f32;
                    let texture_x =
                        (intersect.offset * tex_w as f32).min((tex_w - 1) as f32);

                    let tx = texture_x as usize;
                    let ty = (texture_y as usize).min(tex_h - 1);

                    let index = ty * tex_w + tx;
                    if index < pixel_data.len() {
                        let pix = pixel_data[index];

                        let fog: f32 =
                            (1.0f32 / (safe_distance / 80.0f32 + 1.0f32)).clamp(0.15f32, 1.0f32);
                        let light_dir = ray_angle.cos().abs();
                        let shadow = 0.6f32 + 0.4f32 * light_dir;

                        let color = Color::new(
                            (pix.r as f32 * fog * shadow) as u8,
                            (pix.g as f32 * fog * shadow) as u8,
                            (pix.b as f32 * fog * shadow) as u8,
                            255,
                        );

                        framebuffer.set_current_color(color);

                        for sx in 0..(ray_step as i32) {
                            let px = screen_x + sx;
                            if px >= 0 && px < framebuffer.width {
                                framebuffer.set_pixel(px, y);
                            }
                        }
                    }
                }
            }
        }

        // ----------------------- PISO TEXTURIZADO -----------------------
        if let Some(img_floor) = textures.get("FLOOR_TEMPLE") {
            let data = img_floor.get_image_data();
            let tw = img_floor.width as usize;
            let th = img_floor.height as usize;

            for y in stake_bottom.max(0)..framebuffer.height {
                let dy = (y as f32 - hh).max(1.0f32);
                let world_dist: f32 =
                    (block_size as f32 * distance_to_projection_plane / dy).abs();

                let world_x = player.pos.x + world_dist * ray_angle.cos();
                let world_y = player.pos.y + world_dist * ray_angle.sin();

                let cell_x = world_x / block_size as f32;
                let cell_y = world_y / block_size as f32;

                let tx_f = (cell_x.fract() * tw as f32).abs();
                let ty_f = (cell_y.fract() * th as f32).abs();

                let tx = tx_f as usize % tw;
                let ty = ty_f as usize % th;

                let pix = data[ty * tw + tx];

                let fog: f32 =
                    (1.0f32 / (world_dist / 90.0f32 + 1.0f32)).clamp(0.2f32, 1.0f32);
                let color = Color::new(
                    (pix.r as f32 * fog) as u8,
                    (pix.g as f32 * fog) as u8,
                    (pix.b as f32 * fog) as u8,
                    255,
                );

                framebuffer.set_current_color(color);

                for sx in 0..(ray_step as i32) {
                    let px = screen_x + sx;
                    if px >= 0 && px < framebuffer.width {
                        framebuffer.set_pixel(px, y);
                    }
                }
            }
        }

        // ----------------------- TECHO TEXTURIZADO -----------------------
        if let Some(img_ceil) = textures.get("CEIL_TEMPLE") {
            let data = img_ceil.get_image_data();
            let tw = img_ceil.width as usize;
            let th = img_ceil.height as usize;

            for y in 0..stake_top {
                if y < 0 {
                    continue;
                }

                let dy = (hh - y as f32).max(1.0f32);
                let world_dist: f32 =
                    (block_size as f32 * distance_to_projection_plane / dy).abs();

                let world_x = player.pos.x + world_dist * ray_angle.cos();
                let world_y = player.pos.y + world_dist * ray_angle.sin();

                let cell_x = world_x / block_size as f32;
                let cell_y = world_y / block_size as f32;

                let tx_f = (cell_x.fract() * tw as f32).abs();
                let ty_f = (cell_y.fract() * th as f32).abs();

                let tx = tx_f as usize % tw;
                let ty = ty_f as usize % th;

                let pix = data[ty * tw + tx];

                let fog: f32 =
                    (1.0f32 / (world_dist / 70.0f32 + 1.0f32)).clamp(0.15f32, 1.0f32);
                let color = Color::new(
                    (pix.r as f32 * fog) as u8,
                    (pix.g as f32 * fog) as u8,
                    (pix.b as f32 * fog) as u8,
                    255,
                );

                framebuffer.set_current_color(color);

                for sx in 0..(ray_step as i32) {
                    let px = screen_x + sx;
                    if px >= 0 && px < framebuffer.width {
                        framebuffer.set_pixel(px, y);
                    }
                }
            }
        }
    }

    let sprite_keys = vec!["F", "C", "T"];
    for key in sprite_keys {
        if textures.get(key).is_none() {
            continue;
        }
    }
}

// ======================= SPRITES ==========================
pub fn draw_sprite_billboard(
    framebuffer: &mut Framebuffer,
    sprite_pos: Vector2,
    player: &Player,
    block_size: usize,
    textures: &TextureManager,
    key: &str,
) {
    if let Some(image) = textures.get(key) {
        let pixel_data = image.get_image_data();
        let tw = image.width as usize;
        let th = image.height as usize;

        let dx = sprite_pos.x - player.pos.x;
        let dy = sprite_pos.y - player.pos.y;
        let distance = (dx * dx + dy * dy).sqrt().max(0.001f32);

        let angle_to_sprite = dy.atan2(dx);
        let mut rel_angle = angle_to_sprite - player.a;
        while rel_angle > std::f32::consts::PI {
            rel_angle -= 2.0f32 * std::f32::consts::PI;
        }
        while rel_angle < -std::f32::consts::PI {
            rel_angle += 2.0f32 * std::f32::consts::PI;
        }

        if rel_angle.abs() > player.fov / 2.0f32 + 0.3f32 {
            return;
        }

        let framebuffer_w = framebuffer.width as f32;
        let framebuffer_h = framebuffer.height as f32;
        let distance_to_projection_plane: f32 =
            (framebuffer_w / (2.0f32 * (player.fov / 2.0f32).tan())).abs();

        let sprite_height = (block_size as f32 / distance) * distance_to_projection_plane;
        let sprite_width = sprite_height * (tw as f32 / th as f32);

        let center_x = (0.5f32 + (rel_angle / player.fov)) * framebuffer_w;
        let top = framebuffer_h / 2.0f32 - sprite_height / 2.0f32;
        let left = center_x - sprite_width / 2.0f32;

        for sy in 0..(sprite_height as i32) {
            let v = sy as f32 / sprite_height;
            let ty = ((v * th as f32).clamp(0.0f32, (th - 1) as f32)) as usize;
            let py = (top + sy as f32) as i32;
            if py < 0 || py >= framebuffer.height {
                continue;
            }

            for sx in 0..(sprite_width as i32) {
                let u = sx as f32 / sprite_width;
                let tx = ((u * tw as f32).clamp(0.0f32, (tw - 1) as f32)) as usize;
                let px = (left + sx as f32) as i32;
                if px < 0 || px >= framebuffer.width {
                    continue;
                }

                let index = ty * tw + tx;
                if index >= pixel_data.len() {
                    continue;
                }
                let pix = pixel_data[index];

                if pix.a > 10 {
                    let df = 1.0f32 / (distance / 50.0f32 + 1.0f32);
                    let color = Color::new(
                        (pix.r as f32 * df) as u8,
                        (pix.g as f32 * df) as u8,
                        (pix.b as f32 * df) as u8,
                        255,
                    );
                    framebuffer.set_current_color(color);
                    framebuffer.set_pixel(px, py);
                }
            }
        }
    }
}

// ======================= 2D (MINIMAP) ==========================
pub fn render_world_2d(
    framebuffer: &mut Framebuffer,
    maze: &Maze,
    player: &Player,
    block_size: usize,
) {
    let maze_w = maze[0].len() * block_size;
    let maze_h = maze.len() * block_size;
    let scale_x = framebuffer.width as f32 / maze_w as f32;
    let scale_y = framebuffer.height as f32 / maze_h as f32;
    let scale = scale_x.min(scale_y).max(0.0001f32);

    for (row_index, row) in maze.iter().enumerate() {
        for (col_index, &cell) in row.iter().enumerate() {
            let xo = (col_index * block_size) as f32 * scale;
            let yo = (row_index * block_size) as f32 * scale;
            let w = (block_size as f32 * scale).ceil() as i32;
            let h = (block_size as f32 * scale).ceil() as i32;

            let color = get_cell_color(cell);
            framebuffer.set_current_color(color);
            framebuffer.draw_rect(xo as i32, yo as i32, w, h);
        }
    }

    framebuffer.set_current_color(Color::RED);
    let px = (player.pos.x * scale) as i32;
    let py = (player.pos.y * scale) as i32;
    framebuffer.draw_rect(px - 2, py - 2, 4, 4);

    framebuffer.set_current_color(Color::YELLOW);
    let end_x = player.pos.x + 20.0f32 * player.a.cos();
    let end_y = player.pos.y + 20.0f32 * player.a.sin();
    line::line(
        framebuffer,
        Vector2::new(player.pos.x * scale, player.pos.y * scale),
        Vector2::new(end_x * scale, end_y * scale),
    );
}
