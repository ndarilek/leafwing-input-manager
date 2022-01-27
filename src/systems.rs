//! The systems that power each [`InputManagerPlugin`](crate::InputManagerPlugin).

use crate::{
    action_state::{ActionState, ActionStateDriver},
    input_map::InputMap,
    user_input::InputStreams,
    Actionlike,
};
use bevy::prelude::*;

/// Clears the just-pressed and just-released values of all [`ActionState`]s
///
/// Also resets the internal `pressed_this_tick` field, used to track whether or not to release an action.
pub fn tick_action_state<A: Actionlike>(mut query: Query<&mut ActionState<A>>, time: Res<Time>) {
    for mut action_state in query.iter_mut() {
        // If `Time` has not ever been advanced, something has gone horribly wrong
        // and the user probably forgot to add the `core_plugin`.
        action_state.tick(
            time.last_update()
                .expect("The `Time` resource has never been updated!"),
        );
    }
}

/// Fetches all of the releveant [`Input`] resources to update [`ActionState`] according to the [`InputMap`]
///
/// Missing resources will be ignored, and treated as if none of the corresponding inputs were pressed
pub fn update_action_state<A: Actionlike>(
    maybe_gamepad_input_stream: Option<Res<Input<GamepadButton>>>,
    maybe_keyboard_input_stream: Option<Res<Input<KeyCode>>>,
    maybe_mouse_input_stream: Option<Res<Input<MouseButton>>>,
    mut query: Query<(&mut ActionState<A>, &InputMap<A>)>,
) {
    let gamepad = maybe_gamepad_input_stream.as_deref();

    let keyboard = maybe_keyboard_input_stream.as_deref();

    let mouse = maybe_mouse_input_stream.as_deref();

    for (mut action_state, input_map) in query.iter_mut() {
        let input_streams = InputStreams {
            gamepad,
            keyboard,
            mouse,
            associated_gamepad: input_map.gamepad(),
        };

        let pressed_set = input_map.which_pressed(&input_streams);

        action_state.update(pressed_set);
    }
}

/// When a button with a component `A` is clicked, press the corresponding virtual button in the [`ActionState`]
///
/// The action triggered is determined by the variant stored in your UI-defined button.
pub fn update_action_state_from_interaction<A: Actionlike>(
    ui_query: Query<(&Interaction, &ActionStateDriver<A>)>,
    mut action_state_query: Query<&mut ActionState<A>>,
) {
    for (&interaction, action_state_driver) in ui_query.iter() {
        if interaction == Interaction::Clicked {
            let mut action_state = action_state_query
                .get_mut(action_state_driver.entity)
                .expect("Entity does not exist, or does not have an `ActionState` component.");
            action_state.press(action_state_driver.action);
        }
    }
}
