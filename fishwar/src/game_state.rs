// Enum that will be used as a global state for the game
#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub enum FishWarState {
    Menu,
    Game,
    GameOver,
}
