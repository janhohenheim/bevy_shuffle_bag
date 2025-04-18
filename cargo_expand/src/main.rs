use bevy::prelude::*;

#[derive(Component, Resource, Reflect)]
#[reflect(Component, Resource)]
struct ShuffleBag {
    pub full_collection: Vec<i64>,
    pub current_draft: Vec<i64>,
    pub last_pick: Option<i64>,
}

fn main() -> AppExit {
    App::new().add_plugins(DefaultPlugins).run()
}
