use bevy::{input::common_conditions::input_just_pressed, prelude::*};
use bevy_shuffle_bag::ShuffleBag;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, pick.run_if(input_just_pressed(KeyCode::Space)))
        .run();
}

enum DamageModifier {
    CriticalHit,
    Normal,
    Minor,
}

fn setup(mut commands: Commands) {
    // Spawn the shuffle bag.
    let mut rng = rand::rng();

    commands.spawn(
        ShuffleBag::try_new(
            [
                DamageModifier::CriticalHit,
                DamageModifier::CriticalHit,
                DamageModifier::Normal,
                DamageModifier::Normal,
                DamageModifier::Normal,
                DamageModifier::Normal,
                DamageModifier::Normal,
                DamageModifier::Minor,
                DamageModifier::Minor,
            ],
            &mut rng,
        )
        .unwrap(),
    );

    // Spawn some UI text
    commands
        .spawn(Text::new("Press space to pick a damage amount."))
        .with_child((TextSpan::default(), PickText));
    commands.spawn(Camera2d);
}

#[derive(Component)]
struct PickText;

fn pick(
    mut shuffle_bag: Single<&mut ShuffleBag<DamageModifier>>,
    mut text: Single<&mut TextSpan, With<PickText>>,
) {
    // Pick a number from the shuffle bag. This is guaranteed to never pick the same number twice in a row.
    let pick = shuffle_bag.pick(&mut rand::rng());
    let damage = match pick {
        DamageModifier::CriticalHit => "20 (Critical hit!)",
        DamageModifier::Normal => "10",
        DamageModifier::Minor => "8",
    };
    text.0 = format!("\tLast pick: {}", damage);
}
