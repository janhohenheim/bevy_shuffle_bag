use bevy::{input::common_conditions::input_just_pressed, prelude::*};
use bevy_shuffle_bag::ShuffleBag;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_resource::<SoundAssets>()
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            play_sound.run_if(input_just_pressed(KeyCode::Space)),
        )
        .run();
}

#[derive(Asset, Reflect, Resource)]
struct SoundAssets {
    // A shuffle bag of assets is itself a valid asset, so you can use it as a dependency,
    // which means that `SoundAssets` will only count as fully loaded once all the assets in the
    // shuffle bag have been loaded as well.
    #[dependency]
    steps: ShuffleBag<Handle<AudioSource>>,
}

impl FromWorld for SoundAssets {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();
        let mut rng = rand::rng();
        Self {
            steps: ShuffleBag::try_new(
                vec![
                    assets.load("step1.ogg"),
                    assets.load("step2.ogg"),
                    assets.load("step3.ogg"),
                    assets.load("step4.ogg"),
                ],
                &mut rng,
            )
            .unwrap(),
        }
    }
}

fn setup(mut commands: Commands) {
    // Spawn some UI text
    commands.spawn(Text::new("Press space to play a sound."));
    commands.spawn(Camera2d);
}

fn play_sound(mut commands: Commands, mut sound_assets: ResMut<SoundAssets>) {
    let mut rng = rand::rng();
    // Pick a sound from the shuffle bag. This is guaranteed to never pick the same sound twice in a row.
    let sound = sound_assets.steps.pick(&mut rng);

    // Spawn an audio player that plays the sound and despawns when it finishes.
    commands.spawn((AudioPlayer::new(sound.clone()), PlaybackSettings::DESPAWN));
}
