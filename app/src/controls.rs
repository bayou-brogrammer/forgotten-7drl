use super::prelude::*;
use gridbugs::{
    chargrid::input::{GamepadButton, Input, KeyboardInput},
    direction::CardinalDirection,
};
use maplit::btreemap;
use std::collections::BTreeMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AppInput {
    Get,
    Wait,
    Examine,
    Descend,
    Slot(RangedWeaponSlot),
    Direction(CardinalDirection),
}

#[derive(Serialize, Deserialize)]
pub struct Controls {
    keys: BTreeMap<KeyboardInput, AppInput>,
    gamepad: BTreeMap<GamepadButton, AppInput>,
}

impl Default for Controls {
    fn default() -> Self {
        let keys = btreemap![
            // Action Keys
            KeyboardInput::Char('g') => AppInput::Get,
            KeyboardInput::Char(' ') => AppInput::Wait,
            KeyboardInput::Char('.') => AppInput::Descend,
            KeyboardInput::Char('x') => AppInput::Examine,

            // Movement Keys
            KeyboardInput::Up => AppInput::Direction(CardinalDirection::North),
            KeyboardInput::Left => AppInput::Direction(CardinalDirection::West),
            KeyboardInput::Right => AppInput::Direction(CardinalDirection::East),
            KeyboardInput::Down => AppInput::Direction(CardinalDirection::South),
            KeyboardInput::Char('a') => AppInput::Direction(CardinalDirection::West),
            KeyboardInput::Char('d') => AppInput::Direction(CardinalDirection::East),
            KeyboardInput::Char('w') => AppInput::Direction(CardinalDirection::North),
            KeyboardInput::Char('s') => AppInput::Direction(CardinalDirection::South),
            KeyboardInput::Char('h') => AppInput::Direction(CardinalDirection::West),
            KeyboardInput::Char('l') => AppInput::Direction(CardinalDirection::East),
            KeyboardInput::Char('k') => AppInput::Direction(CardinalDirection::North),
            KeyboardInput::Char('j') => AppInput::Direction(CardinalDirection::South),

            // Slot Keys
            KeyboardInput::Char('1') => AppInput::Slot(RangedWeaponSlot::Slot1),
            KeyboardInput::Char('2') => AppInput::Slot(RangedWeaponSlot::Slot2),
            KeyboardInput::Char('3') => AppInput::Slot(RangedWeaponSlot::Slot3),
        ];

        let gamepad = btreemap![
            // Action Keys
            GamepadButton::North => AppInput::Get,
            GamepadButton::Select => AppInput::Wait,
            GamepadButton::RightBumper => AppInput::Examine,
            GamepadButton::LeftBumper => AppInput::Descend,

            // Slot Keys
            GamepadButton::West => AppInput::Slot(RangedWeaponSlot::Slot1),
            GamepadButton::South => AppInput::Slot(RangedWeaponSlot::Slot2),
            GamepadButton::East => AppInput::Slot(RangedWeaponSlot::Slot3),

            // Movement Keys
            GamepadButton::DPadLeft => AppInput::Direction(CardinalDirection::West),
            GamepadButton::DPadRight => AppInput::Direction(CardinalDirection::East),
            GamepadButton::DPadUp => AppInput::Direction(CardinalDirection::North),
            GamepadButton::DPadDown => AppInput::Direction(CardinalDirection::South),
        ];

        Self { keys, gamepad }
    }
}
impl Controls {
    pub fn get(&self, input: Input) -> Option<AppInput> {
        match input {
            Input::Keyboard(keyboard_input) => self.keys.get(&keyboard_input).cloned(),
            Input::Gamepad(gamepad_input) => self.gamepad.get(&gamepad_input.button).cloned(),
            Input::Mouse(_) => None,
        }
    }

    pub fn get_direction(&self, input: Input) -> Option<CardinalDirection> {
        self.get(input).and_then(|app_input| match app_input {
            AppInput::Direction(direction) => Some(direction),
            _ => None,
        })
    }

    pub fn get_slot(&self, input: Input) -> Option<RangedWeaponSlot> {
        self.get(input).and_then(|app_input| match app_input {
            AppInput::Slot(slot) => Some(slot),
            _ => None,
        })
    }
}
