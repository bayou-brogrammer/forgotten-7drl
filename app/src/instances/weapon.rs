use gridbugs::chargrid::text::StyledString;

use crate::prelude::*;

use super::{yes_no, GameExamineWithMouseComponent};

pub fn try_get_ranged_weapon(ranged_witness: GetRangedWeapon) -> AppCF<GameState> {
    on_state_then(move |state: &mut State| {
        let num_weapon_slots = if state.player_has_third_weapon_slot() { 3 } else { 2 };
        state.context_message = Some(StyledString {
            string: format!("Choose a weapon slot: (press 1-{} or escape to cancel)", num_weapon_slots),
            style: Style::plain_text().with_bold(true).with_foreground(Rgba32::hex_rgb(0xFF0000)),
        });

        on_input_state(move |input, state: &mut State| {
            use RangedWeaponSlot::*;
            let slot = state.controls.get_slot(input);
            if slot == Some(Slot3) && num_weapon_slots < 3 {
                None
            } else {
                slot
            }
        })
        .catch_escape_or_start()
        .overlay(render_state(|state: &State, ctx, fb| state.render(CURSOR, ctx, fb)), 10)
        .and_then(|slot_or_err| {
            on_state_then(move |state: &mut State| {
                state.context_message = None;
                match slot_or_err {
                    Err(_escape_or_start) => val_once(ranged_witness.cancel()),
                    Ok(slot) => {
                        if state.player_has_weapon_in_slot(slot) {
                            yes_no(format!("Replace ranged weapon in slot {}?", slot.number())).and_then(
                                move |yes| {
                                    on_state(move |state: &mut State| {
                                        if yes {
                                            ranged_witness.commit(state.scope_mut(), slot)
                                        } else {
                                            ranged_witness.cancel()
                                        }
                                    })
                                },
                            )
                        } else {
                            val_once(ranged_witness.commit(state.scope_mut(), slot))
                        }
                    }
                }
            })
        })
    })
}

pub fn try_get_melee_weapon(witness: GetMeleeWeapon) -> AppCF<GameState> {
    yes_no("Replace current melee weapon?".to_string()).and_then(move |yes| {
        on_state(move |state: &mut State| {
            if yes {
                let scope = state.scope_mut();
                witness.commit(scope)
            } else {
                witness.cancel()
            }
        })
    })
}

pub fn fire_weapon(witness: FireWeapon) -> AppCF<GameState> {
    on_state_then(move |state: &mut State| {
        state.context_message = Some(StyledString {
            string: format!(
                "Fire weapon {} in which direction? (escape/start to cancel)",
                witness.slot().number()
            ),
            style: Style::plain_text().with_bold(true).with_foreground(Rgba32::hex_rgb(0xFF0000)),
        });
        on_input_state(move |input, state: &mut State| state.controls.get_direction(input))
            .catch_escape_or_start()
            .overlay(GameExamineWithMouseComponent, 10)
            .and_then(|direction_or_err| {
                on_state(move |state: &mut State| {
                    state.context_message = None;
                    match direction_or_err {
                        Err(_escape_or_start) => witness.cancel(),
                        Ok(direction) => witness.commit(state.scope_mut(), direction),
                    }
                })
            })
    })
}
