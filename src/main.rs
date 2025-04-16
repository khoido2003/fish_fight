
use collections::storage;
use macroquad::experimental::scene::{Node, RefMut};
use macroquad::prelude::*;
use macroquad_platformer::{Actor, Tile, World};
use macroquad_tiled::{self as tiled, Map};

#[derive(Debug)]
struct Player {
    collider: Actor,
    speed: Vec2,
}

struct Resources {
    whale: Texture2D,
    physics: World,
}

impl Player {
    pub const JUMP_SPEED: f32 = -700.0;
    pub const GRAVITY: f32 = 2000.0;
    pub const MOVE_SPEED: f32 = 300.0;

    fn new() -> Player {
        let mut resources = storage::get_mut::<Resources>();

        Player {
            collider: resources.physics.add_actor(vec2(200.0, 100.0), 36, 66),
            speed: vec2(0.0, 0.0),
        }
    }
}

impl Node for Player {
    fn draw(node: RefMut<Self>) {
        let resources = storage::get_mut::<Resources>();
        let pos = resources.physics.actor_pos(node.collider);

        draw_texture_ex(
            &resources.whale,
            pos.x - 20.0,
            pos.y,
            WHITE,
            DrawTextureParams {
                source: Some(Rect::new(0.0, 0.0, 76.0, 66.0)),
                ..Default::default()
            },
        );
    }

    fn update(mut node: RefMut<Self>) {
        let world = &mut storage::get_mut::<Resources>().physics;

        let pos = world.actor_pos(node.collider);
        let on_ground = world.collide_check(node.collider, pos + vec2(0.0, 1.0));

        if !on_ground {
            node.speed.y += Self::GRAVITY * get_frame_time();
        } else {
            node.speed.y = 0.0;
        }

        if is_key_down(KeyCode::Right) {
            node.speed.x = Self::MOVE_SPEED;
        } else if is_key_down(KeyCode::Left) {
            node.speed.x = -Self::MOVE_SPEED;
        } else {
            node.speed.x = 0.0;
        }

        if is_key_pressed(KeyCode::Space) && on_ground {
            node.speed.y = Self::JUMP_SPEED;
        }

        world.move_v(node.collider, node.speed.y * get_frame_time());
        world.move_h(node.collider, node.speed.x * get_frame_time());
    }
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

    let mut physics = World::new();
    physics.add_static_tiled_layer(
        static_colliders,
        tiled_map.raw_tiled_map.tilewidth as f32,
        tiled_map.raw_tiled_map.tileheight as f32,
        tiled_map.raw_tiled_map.width as _,
        1,
    );

    let resources = Resources { whale, physics };
    storage::store(resources);

    let player = Player::new();
    let player_handle = scene::add_node(player);

    let width = tiled_map.raw_tiled_map.tilewidth * tiled_map.raw_tiled_map.width;
    let height = tiled_map.raw_tiled_map.tileheight * tiled_map.raw_tiled_map.height;

    loop {
        clear_background(BLACK);

        let player = scene::get_node(player_handle);

        tiled_map.draw_tiles(
            "main layer",
            Rect::new(0.0, 0.0, width as _, height as _),
            None,
        );

        Player::draw(player);

        next_frame().await;
    }
}
