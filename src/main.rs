mod camera;
mod components;
mod map;
mod map_builder;
mod spawner;
mod systems;
mod turn_state;

// I feel this author is a bit trigger happy with the prelude, but I assume its
// to make the sample code less verbose / intimidating, but it obfuscates dependencies
// just every module is dependent on _everything_ the crate depends on if use statements 
// were to be believed, I don't fancy disentagling this right now though
mod prelude {
    pub use bracket_lib::prelude::*;
    pub use legion::*;
    pub use legion::world::SubWorld;
    pub use legion::systems::CommandBuffer;
    pub const SCREEN_WIDTH: i32 = 80;
    pub const SCREEN_HEIGHT: i32 = 50;
    pub const DISPLAY_WIDTH: i32 = SCREEN_WIDTH / 2;
    pub const DISPLAY_HEIGHT: i32 = SCREEN_HEIGHT / 2;
    pub use crate::camera::*;
    pub use crate::components::*;
    pub use crate::map::*;
    pub use crate::map_builder::*;
    pub use crate::spawner::*;
    pub use crate::systems::*;
    pub use crate::turn_state::*;
}

use prelude::*;

struct State {
    world: World,
    resources: Resources,
    input_systems: Schedule,
    player_systems: Schedule,
    monster_systems: Schedule,
}

impl State {
    fn new() -> Self {
        let mut world = World::default();
        let mut resources = Resources::default();
        let mut rng = RandomNumberGenerator::new();
        let mut map_builder = MapBuilder::new(&mut rng);

        // Assume at least 2 levels so just spawn exit
        let exit_idx = map_builder.map.point2d_to_index(map_builder.amulet_start);
        map_builder.map.tiles[exit_idx] = TileType::Exit;

        spawn_player(&mut world, map_builder.player_start);
        spawn_level(&mut world, &mut rng, 0, &map_builder.spawn_points);

        resources.insert(map_builder.map);
        resources.insert(Camera::new(map_builder.player_start));
        resources.insert(TurnState::AwaitingInput);
        resources.insert(map_builder.theme);

        Self {
            world,
            resources,
            input_systems: build_input_scheduler(),
            player_systems: build_player_scheduler(),
            monster_systems: build_monster_scheduler(),
        }
    }

    fn advance_level(&mut self) {
        let player_entity = *<Entity>::query().filter(component::<Player>()).iter(&mut self.world).nth(0).unwrap();
        use std::collections::HashSet;
        
        // Keep player and their inventory
        let mut entities_to_keep = HashSet::new();
        entities_to_keep.insert(player_entity); 
        <(Entity, &Carried)>::query().iter(&self.world).filter(|(_e, carry)| carry.0 == player_entity)
            .map(|(e, _carry)| *e).for_each(|e| { entities_to_keep.insert(e); });
        
        // Remove other entities
        let mut cb = CommandBuffer::new(&mut self.world);
        for e in Entity::query().iter(&self.world) {
            if !entities_to_keep.contains(e) {
                cb.remove(*e);
            }
        }
        cb.flush(&mut self.world);

        // Dirty player's Field of View
        <&mut FieldOfView>::query().iter_mut(&mut self.world).for_each(|fov| fov.is_dirty = true);

        // Build new level - lotta duplicated cone in new / reset and here
        let mut rng = RandomNumberGenerator::new();
        let mut map_builder = MapBuilder::new(&mut rng);
        let mut map_level = 0;
        <(&mut Player, &mut Point)>::query().iter_mut(&mut self.world)
            .for_each(|(player, pos)| {
                player.map_level += 1;
                map_level = player.map_level;
                pos.x = map_builder.player_start.x;
                pos.y = map_builder.player_start.y;
            });
        
        if map_level == 2 {
            spawn_amulet_of_yala(&mut self.world, map_builder.amulet_start);
        } else {
            let exit_idx = map_builder.map.point2d_to_index(map_builder.amulet_start);
            map_builder.map.tiles[exit_idx] = TileType::Exit;
        }

        spawn_level(&mut self.world, &mut rng, map_level as usize, &map_builder.spawn_points);

        self.resources.insert(map_builder.map);
        self.resources.insert(Camera::new(map_builder.player_start));
        self.resources.insert(TurnState::AwaitingInput);
        self.resources.insert(map_builder.theme);
    }

    fn reset(&mut self) {
        self.world = World::default();
        self.resources = Resources::default();
        let mut rng = RandomNumberGenerator::new();
        let mut map_builder = MapBuilder::new(&mut rng);

        spawn_player(&mut self.world, map_builder.player_start);
        // Assume at least 2 levels so just spawn exit
        let exit_idx = map_builder.map.point2d_to_index(map_builder.amulet_start);
        map_builder.map.tiles[exit_idx] = TileType::Exit;
        spawn_level(&mut self.world, &mut rng, 0, &map_builder.spawn_points);
        self.resources.insert(map_builder.map);
        self.resources.insert(Camera::new(map_builder.player_start));
        self.resources.insert(TurnState::AwaitingInput);
        self.resources.insert(map_builder.theme);
    }

    fn game_over(&mut self, ctx: &mut BTerm) {
        ctx.set_active_console(3);
        ctx.print_color_centered(2, RED, BLACK, "Your quest has ended.");
        ctx.print_color_centered(4, WHITE, BLACK, "Slain by a monster, your hero's journey has come to a premature end.");
        ctx.print_color_centered(5, WHITE, BLACK, "The Amulet of Yala remains unclaimed, and your home town is not saved.");
        ctx.print_color_centered(8, YELLOW, BLACK, "Don't worry, you can always try again with a new hero.");
        ctx.print_color_centered(9, GREEN, BLACK, "Press 1 to play again.");

        if let Some(VirtualKeyCode::Key1) = ctx.key {
            self.reset();
        }
    }

    fn victory(&mut self, ctx: &mut BTerm) {
        ctx.set_active_console(3); // TODO: Some constants to prelude for render layers
        ctx.print_color_centered(2, GREEN, BLACK, "You have won!");
        ctx.print_color_centered(4, WHITE, BLACK, "You put on the Amulet of Yala and feel its power course through your veins.");
        ctx.print_color_centered(5, WHITE, BLACK, "Your town is saved and you can return to your normal life.");
        ctx.print_color_centered(7, GREEN, BLACK, "Press 1 to play again");
        if let Some(VirtualKeyCode::Key1) = ctx.key {
            self.reset();
        }
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        // Seems like set_active_console is 1 indexed despite the book saying it's 0 indexed
        ctx.set_active_console(1);
        ctx.cls();

        ctx.set_active_console(2);
        ctx.cls();

        ctx.set_active_console(3);
        ctx.cls();

        self.resources.insert(ctx.key);
        ctx.set_active_console(1);
        self.resources.insert(Point::from_tuple(ctx.mouse_pos()));

        let current_state = self.resources.get::<TurnState>().unwrap().clone(); // this clone is to not annoy the borrow checker
        match current_state {
            TurnState::AwaitingInput => self.input_systems.execute(&mut self.world, &mut self.resources),
            TurnState::PlayerTurn => self.player_systems.execute(&mut self.world, &mut self.resources),
            TurnState::MonsterTurn => self.monster_systems.execute(&mut self.world, &mut self.resources),
            TurnState::GameOver => self.game_over(ctx),
            TurnState::Victory => self.victory(ctx),
            TurnState::NextLevel => self.advance_level(),
        }
        render_draw_buffer(ctx).expect("Render error");
    }
}

fn main() -> BError {
    let context = BTermBuilder::simple80x50()
        .with_title("Rusty Roguelike")
        .with_fps_cap(30.0)
        .with_dimensions(DISPLAY_WIDTH, DISPLAY_HEIGHT)
        .with_tile_dimensions(32, 32)
        .with_resource_path("resources/")
        .with_font("dungeonfont.png", 32, 32)
        .with_font("terminal8x8.png", 8, 8)
        .with_simple_console(DISPLAY_WIDTH, DISPLAY_HEIGHT, "dungeonfont.png")
        .with_simple_console_no_bg(DISPLAY_WIDTH, DISPLAY_HEIGHT, "dungeonfont.png")
        .with_simple_console_no_bg(SCREEN_WIDTH * 2, SCREEN_HEIGHT * 2, "terminal8x8.png")
        .build()?;
    main_loop(context, State::new())
}
