use crate::prelude::*;

#[system]
#[read_component(Health)]
#[read_component(Player)]
#[read_component(Point)]
#[read_component(AmuletOfYala)]
pub fn end_turn(sub_world: &SubWorld, #[resource] turn_state: &mut TurnState) {

	let mut player = <(&Health, &Point)>::query().filter(component::<Player>());
	let mut amulet = <&Point>::query().filter(component::<AmuletOfYala>());

	let current_state = turn_state.clone();
	let mut new_state = match current_state {
		TurnState::AwaitingInput => return,
		TurnState::PlayerTurn => TurnState::MonsterTurn,
		TurnState::MonsterTurn => TurnState::AwaitingInput,
		_ => current_state
	};

	let amulet_pos = amulet.iter(sub_world).nth(0).unwrap();
	player.iter(sub_world).for_each(|(hp, pos)| {
		if hp.current < 1 {
			new_state = TurnState::GameOver;
		} else if pos == amulet_pos {
			new_state = TurnState::Victory;
		}
	});

	*turn_state = new_state;
}