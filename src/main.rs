mod camera;
mod components;
mod map;
mod map_builder;
mod spawner;
mod systems;

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
}

use prelude::*;

struct State {
    world: World,
    resources: Resources,
    systems: Schedule
}

impl State {
    fn new() -> Self {
        let mut world = World::default();
        let mut resources = Resources::default();
        let mut rng = RandomNumberGenerator::new();
        let map_builder = MapBuilder::new(&mut rng);
        
        resources.insert(map_builder.map);
        resources.insert(Camera::new(map_builder.player_start));
        spawn_player(&mut world, map_builder.player_start);

        map_builder.rooms
            .iter()
            .skip(1)
            .map(|r| r.center())
            .for_each(|pos| spawn_monster(&mut world, &mut rng, pos));

        Self {
            world,
            resources,
            systems: build_scheduler(),
        }
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        // I don't know why but trying to use the books code layer 0 for map and layer 1 for 
        // player with 0 being w/ bg and 2 being w/o bg then it doesn't render the map
        // if you change console 0 to be no_bg it renders it as ascii and offset. 
        // This hack of using 3 layer and just not using layer 0 seems to work. :shrug:
        // :thinking: wonder if this is the same on other platfroms
        ctx.set_active_console(1);
        ctx.cls();

        ctx.set_active_console(2);
        ctx.cls();

        self.resources.insert(ctx.key);
        self.systems.execute(&mut self.world, &mut self.resources);
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
        .with_simple_console(DISPLAY_WIDTH, DISPLAY_HEIGHT, "dungeonfont.png")
        .with_simple_console_no_bg(DISPLAY_WIDTH, DISPLAY_HEIGHT, "dungeonfont.png")
        .with_simple_console_no_bg(DISPLAY_WIDTH, DISPLAY_HEIGHT, "dungeonfont.png")
        .build()?;
    main_loop(context, State::new())
}
