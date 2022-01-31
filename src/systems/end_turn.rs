use crate::prelude::*;

#[system]
#[read_component(Health)]
#[read_component(Point)]
#[read_component(Player)]
#[read_component(Carried)]
#[read_component(AmuletOfYala)]
pub fn end_turn(
	sub_world: &SubWorld,
	#[resource] turn_state: &mut TurnState,
	#[resource] map: &Map) {

	let mut player = <(&Health, &Point)>::query().filter(component::<Player>());
	let mut amulet = <&Carried>::query().filter(component::<AmuletOfYala>());

	let current_state = turn_state.clone();
	let mut new_state = match current_state {
		TurnState::AwaitingInput => return,
		TurnState::PlayerTurn => TurnState::MonsterTurn,
		TurnState::MonsterTurn => TurnState::AwaitingInput,
		_ => current_state
	};

	player.iter(sub_world).for_each(|(hp, pos)| {
		if hp.current < 1 {
			new_state = TurnState::GameOver;
		} else if amulet.iter(sub_world).nth(0).is_some() {
			new_state = TurnState::Victory;
		} else {
			let idx = map.point2d_to_index(*pos);
			if map.tiles[idx] == TileType::Exit {
				new_state = TurnState::NextLevel
			}
		}
	});

	*turn_state = new_state;
}