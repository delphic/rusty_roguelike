use crate::prelude::*;
use serde::Deserialize;
use ron::de::from_reader;
use std::fs::File;
use std::collections::HashSet;
use legion::systems::CommandBuffer;

#[derive(Clone, Deserialize, Debug)]
pub struct EntityDefinition {
	pub entity_type: EntityType,
	pub levels: HashSet<usize>,
	pub frequency: i32,
	pub name: String,
	pub glyph: char,
	pub vision_range: Option<i32>,
	pub provides: Option<Vec<(String, i32)>>,
	pub hp: Option<i32>,
	pub base_damage: Option<i32>,
}

#[derive(Clone, Deserialize, Debug, PartialEq)]
pub enum EntityType {
	Enemy,
	Item,
}

#[derive(Clone, Deserialize, Debug)]
pub struct Config {
	pub entity_definitions: Vec<EntityDefinition>,
}

impl Config {
	pub fn load() -> Self {
		let file = File::open("resources/config.ron").expect("Failed opening file");
		from_reader(file).expect("Unable to load game config")
	}

	pub fn spawn_entities(
		&self,
		world: &mut World,
		rng: &mut RandomNumberGenerator,
		level: usize,
		spawn_points: &[Point]
	) {
		let mut definitions = Vec::new();
		self.entity_definitions.iter().filter(|e| e.levels.contains(&level))
			.for_each(|t| {
				for _ in 0 .. t.frequency {
					definitions.push(t);
				}
			});

		let mut commands = CommandBuffer::new(world);
		spawn_points.iter().for_each(|pt| {
			if let Some(definition) = rng.random_slice_entry(&definitions) {
				self.spawn_entity(pt, definition, &mut commands);
			}
		});
		commands.flush(world);
	}

	fn spawn_entity(
		&self,
		pt: &Point,
		definition: &EntityDefinition,
		commands: &mut legion::systems::CommandBuffer
	) {
		let entity = commands.push((
			pt.clone(),
			Render {
				color: ColorPair::new(WHITE, BLACK),
				glyph: to_cp437(definition.glyph),
			},
			Name(definition.name.clone())
		));

		match definition.entity_type {
			EntityType::Item => commands.add_component(entity, Item{}),
			EntityType::Enemy => {
				commands.add_component(entity, Enemy{});
				commands.add_component(entity, FieldOfView::new(definition.vision_range.unwrap()));
				commands.add_component(entity, ChasingPlayer {});
				commands.add_component(entity, Health {
					current: definition.hp.unwrap(),
					max: definition.hp.unwrap(),
				})
			}
		}

		if let Some(effects) = &definition.provides {
			effects.iter().for_each(|(provides, n)| {
				match provides.as_str() {
					"Healing" => commands.add_component(entity, ProvidesHealing { amount: *n }),
					"MagicMap" => commands.add_component(entity, ProvidesDungeonMap{}),
					_ => println!("Warning we don't know how to provide {}", provides),
				}
			});
		}

		if let Some(damage) = &definition.base_damage {
			commands.add_component(entity, Damage(*damage));
			if definition.entity_type == EntityType::Item {
				commands.add_component(entity, Weapon{});
			}
		}
	}
}