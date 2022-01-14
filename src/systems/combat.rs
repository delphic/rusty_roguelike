use crate::prelude::*;

#[system]
#[read_component(WantsToAttack)]
#[write_component(Health)]
pub fn combat(sub_world: &mut SubWorld, commands: &mut CommandBuffer) {
	let mut attackers = <(Entity, &WantsToAttack)>::query();
	let victims : Vec<(Entity, Entity)> = attackers
		.iter(sub_world)
		.map(|(messenger, intent)| (*messenger, intent.victim))
		.collect();
	
	victims.iter().for_each(|(messenger, victim)| {
		if let Ok(mut health) = sub_world.entry_mut(*victim).unwrap().get_component_mut::<Health>() {
			health.current -= 1;
			if health.current < 1 {
				commands.remove(*victim);
			}
		}
		commands.remove(*messenger);
	});
}