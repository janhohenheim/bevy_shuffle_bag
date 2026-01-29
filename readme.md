# Bevy Shuffle Bag

[![crates.io](https://img.shields.io/crates/v/bevy_shuffle_bag)](https://crates.io/crates/bevy_shuffle_bag)
[![docs.rs](https://docs.rs/bevy_shuffle_bag/badge.svg)](https://docs.rs/bevy_shuffle_bag)

A tiny crate providing a shuffle bag, which is a collection of items that can endlessly be picked in a random, nonrepeating order.

The bag will be emptied in *drafts*, where each draft contains all the items in the bag, but in a random order.
This means that if you have e.g. a bag with 3 soundtracks, all of them will play once in a random order, and then the bag will be refilled with the same soundtracks in a random order.
If the bag contains no duplicates, items are always picked such that the same item is never picked twice in a row.

No more playing the same sound effect or dialogue twice in a row!

## Examples

Using a shuffle bag is really simple. You just initialize it with some contents and then pick from them:

```rust
use bevy::prelude::*;
use bevy_shuffle_bag::ShuffleBag;

let mut rng = rand::rng();
let mut treasure_chest = ShuffleBag::try_new(["gold", "armor", "sword"], &mut rng).unwrap();
let loot = treasure_chest.pick(&mut rng);
println!("I just picked up a {loot}!");
```

No need to add any plugin, this crate only brings in a `ShuffleBag`, nothing more.
`ShuffleBag` implements all your favorite Bevy traits like `Component`, `Resource`, `Asset`, etc., so it's really flexible.
For example, this is how you load sound assets and then play them in a random order, ensuring that no sound effect gets played twice:

```rust,no_run
use bevy::{input::common_conditions::input_just_pressed, prelude::*};
use bevy_shuffle_bag::ShuffleBag;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_resource::<SoundAssets>()
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

fn play_sound(mut commands: Commands, mut sound_assets: ResMut<SoundAssets>) {
    let mut rng = rand::rng();
    // Pick a sound from the shuffle bag. This is guaranteed to never pick the same sound twice in a row.
    let sound = sound_assets.steps.pick(&mut rng);

    // Spawn an audio player that plays the sound and despawns when it finishes.
    commands.spawn((AudioPlayer::new(sound.clone()), PlaybackSettings::DESPAWN));
}
```

See the `examples` directory for more :)


## Compatibility

| bevy        | bevy_shuffle_bag |
|-------------|------------------|
| 0.18        | 0.4              |
| 0.17        | 0.3              |
| 0.16        | 0.2              |
| 0.15        | 0.1              |

## Credits

Thanks for the one and only Tim Cain for teaching this concept in a YouTube video. Unfortunatly, I forgot which one! Heck!
