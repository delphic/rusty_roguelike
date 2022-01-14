#[derive(Copy, Clone, Debug, PartialEq)]
pub enum TurnState {
	AwaitingInput,
	PlayerTurn,
	MonsterTurn,
	GameOver, // This isn't a Turn State it's a game state
}