use crate::prelude::*;

#[system]
#[read_component(WantsToAttack)]
#[read_component(Player)]
#[write_component(Health)]
#[read_component(Damage)]
#[read_component(Weapon)]
#[read_component(Carried)]
pub fn combat(sub_world: &mut SubWorld, commands: &mut CommandBuffer) {
	let mut attackers = <(Entity, &WantsToAttack)>::query();
	let victims : Vec<(Entity, Entity, Entity)> = attackers
		.iter(sub_world)
		.map(|(messenger, intent)| (*messenger, intent.attacker, intent.victim))
		.collect();

	victims.iter().for_each(|(messenger, attacker, victim)| {
		let is_player = sub_world.entry_ref(*victim).unwrap().get_component::<Player>().is_ok();

		let base_damage = if let Ok(v) = sub_world.entry_ref(*attacker) {
			if let Ok(dmg) = v.get_component::<Damage>() {
				dmg.0
			} else {
				0
			}
		} else {
			0
		};

		
		let weapon_damage : i32 = <(&Carried, &Damage)>::query().iter(sub_world)
			.filter(|(carried, _)| carried.0 == *attacker)
			.map(|(_, dmg)| dmg.0)
			.sum();
		
		let total_damage = base_damage + weapon_damage;

		if let Ok(mut health) = sub_world.entry_mut(*victim).unwrap().get_component_mut::<Health>() {
			health.current -= total_damage;
			if health.current < 1 && !is_player {
				commands.remove(*victim);
			}
		}
		commands.remove(*messenger);
	});
}