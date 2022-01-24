use crate::prelude::*;
use std::collections::HashSet;

#[system]
#[read_component(Point)]
#[read_component(ChasingPlayer)]
#[read_component(Health)]
#[read_component(Player)]
#[read_component(FieldOfView)]
pub fn chasing(#[resource] map: &Map, sub_world: &SubWorld, commands: &mut CommandBuffer) {
	let mut movers = <(Entity, &Point, &ChasingPlayer, &FieldOfView)>::query();
	let mut positions = <(Entity, &Point, &Health)>::query();
	let mut player = <(&Point, &Player)>::query();

	let player_pos = player.iter(sub_world).nth(0).unwrap().0; // Beautiful, ECS is so elegant
	let player_idx = get_map_idx(player_pos.x, player_pos.y);

	let search_targets = vec![player_idx];
	let dijkstra_map = DijkstraMap::new(
		SCREEN_WIDTH,
		SCREEN_HEIGHT,
		&search_targets,
		map, 
		1024.0);

	let mut taken_destinations = HashSet::new(); // prevent stacking

	movers.iter(sub_world).for_each(|(entity, pos, _, fov)| {
		if !fov.visible_tiles.contains(&player_pos) {
			return;
		}

		let idx = get_map_idx(pos.x, pos.y);
		if let Some(destination) = DijkstraMap::find_lowest_exit(&dijkstra_map, idx, map) {
			let distance_to_player = DistanceAlg::Pythagoras.distance2d(*pos, *player_pos);
			let destination = if distance_to_player > 1.2 {
				map.index_to_point2d(destination)
			} else {
				*player_pos
			};

			let mut should_move = true;
			positions.iter(sub_world)
				.filter(|(_, target_pos, _)| **target_pos == destination)
				.for_each(|(victim, _, _)| {
					if sub_world.entry_ref(*victim).unwrap().get_component::<Player>().is_ok() {
						commands.push(((), WantsToAttack { attacker: *entity, victim: *victim }));
					}
					should_move = false;
					// This also prevents moving into another units space, however, if they are successfully moving then 
					// actually this should be allowed - run from closest to furthest and tracking who is going to move
					// would allow us to resolve this (although that only works because the behaviour is simple (chase))
				});
			
			if should_move && !taken_destinations.contains(&destination) {
				taken_destinations.insert(destination);
				commands.push(((), WantsToMove { entity: *entity, destination }));
			}
		}
	});
}