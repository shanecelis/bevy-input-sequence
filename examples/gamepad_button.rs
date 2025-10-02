use bevy::prelude::*;
use bevy_input_sequence::prelude::*;

#[derive(Message, Clone, Debug)]
struct MyEvent(usize, Entity);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(InputSequencePlugin::default().match_button(true))
        .add_message::<MyEvent>()
        .add_systems(Startup, setup)
        .add_systems(Update, input_sequence_event_system)
        .run();
}

fn setup(mut commands: Commands) {
    commands.queue(
        ButtonSequence::new(
            action::send_event_with_input(|gamepad| MyEvent(0, gamepad)),
            [
                GamepadButton::North,
                GamepadButton::East,
                GamepadButton::South,
                GamepadButton::West,
            ],
        )
        .time_limit(Duration::from_secs(1)),
    );

    commands.queue(
        ButtonSequence::new(
            action::send_event_with_input(|gamepad| MyEvent(1, gamepad)),
            [
                GamepadButton::North,
                GamepadButton::West,
                GamepadButton::South,
                GamepadButton::East,
            ],
        )
        .time_limit(Duration::from_secs(1)),
    );

    println!("Press north, east, south, west to emit MyEvent 0.");
    println!("Press north, west, south, east to emit MyEvent 1.");
}

fn input_sequence_event_system(mut er: MessageReader<MyEvent>) {
    for e in er.read() {
        println!("{:?} emitted from gamepad {:?}", e.0, e.1);
    }
}
