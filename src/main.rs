use macroquad::prelude::*;
use macroquad_platformer::{Actor, Tile, World};
use macroquad_tiled::{self as tiled, Map};

struct Player {
    collider: Actor,
    speed: Vec2,
}

mod consts {
    pub const JUMP_SPEED: f32 = -700.0;
    pub const GRAVITY: f32 = 2000.0;
    pub const MOVE_SPEED: f32 = 300.0;
}

#[macroquad::main("FishGame")]
async fn main() {
    // Load assets
    let tiledset: Texture2D;
    let decorations: Texture2D;
    let tiled_map_json: String;

    if let Ok(r) = load_texture("assets/tileset.png").await {
        tiledset = r;
        tiledset.set_filter(FilterMode::Nearest);
    } else {
        println!("Not found tileset.png");
        return;
    }

    if let Ok(r) = load_texture("assets/decorations1.png").await {
        decorations = r;
        decorations.set_filter(FilterMode::Nearest);
    } else {
        println!("Not found decorations1.png");
        return;
    }

    if let Ok(r) = load_string("assets/map.json").await {
        tiled_map_json = r;
    } else {
        println!("Not found map.json");
        return;
    }

    let tiled_map: Map;

    match tiled::load_map(
        &tiled_map_json,
        &[("tileset.png", tiledset), ("decorations1.png", decorations)],
        &[],
    ) {
        Ok(r) => {
            tiled_map = r;
        }
        Err(e) => {
            println!("Error: {:?}", e);
            return;
        }
    }

    let whale = load_texture("assets/Whale/Whale(76x66)(Orange).png")
        .await
        .expect("Can not load whale");

    ////////////////////////////////////////////////////////////////////

    // Make tiles become colliable
    let mut static_colliders = vec![];
    for (_x, _y, tile) in tiled_map.tiles("main layer", None) {
        if tile.is_some() {
            static_colliders.push(Tile::Collider);
        } else {
            static_colliders.push(Tile::Empty);
        }
    }

    let mut world = World::new();
    world.add_static_tiled_layer(
        static_colliders,
        tiled_map.raw_tiled_map.tilewidth as f32,
        tiled_map.raw_tiled_map.tileheight as f32,
        tiled_map.raw_tiled_map.width as _,
        1,
    );

    let mut player = Player {
        collider: world.add_actor(vec2(200.0, 100.0), 36, 66),
        speed: vec2(0.0, 0.0),
    };

    let width = tiled_map.raw_tiled_map.tilewidth * tiled_map.raw_tiled_map.width;
    let height = tiled_map.raw_tiled_map.tileheight * tiled_map.raw_tiled_map.height;

    loop {
        clear_background(BLACK);

        tiled_map.draw_tiles(
            "main layer",
            Rect::new(0.0, 0.0, width as _, height as _),
            None,
        );

        let pos = world.actor_pos(player.collider);

        draw_texture_ex(
            &whale,
            pos.x - 20.0,
            pos.y,
            WHITE,
            DrawTextureParams {
                source: Some(Rect::new(0.0, 0.0, 76., 66.)),
                ..Default::default()
            },
        );

        {
            let pos = world.actor_pos(player.collider);
            let on_ground = world.collide_check(player.collider, pos + vec2(0.0, 1.0));

            if !on_ground {
                player.speed.y += consts::GRAVITY * get_frame_time();
            }

            if is_key_down(KeyCode::Right) {
                player.speed.x = consts::MOVE_SPEED;
            } else if is_key_down(KeyCode::Left) {
                player.speed.x = -consts::MOVE_SPEED;
            } else {
                player.speed.x = 0.;
            }

            if is_key_pressed(KeyCode::Space) && on_ground {
                player.speed.y = consts::JUMP_SPEED;
            }

            world.move_h(player.collider, player.speed.x * get_frame_time());
            world.move_v(player.collider, player.speed.y * get_frame_time());
        }

        next_frame().await;
    }
}
