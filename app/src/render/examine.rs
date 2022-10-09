use super::*;
use crate::controls::AppInput;
use gridbugs::chargrid::text::StyledString;

#[derive(Clone, Copy, Debug)]
enum MessageVerb {
    See,
    Remember,
} 

pub fn examine(game: &StateScope, coord: Coord) -> Option<StyledString> {
    let mut entity_under_cursor = None;
    let visibility = &game.visibility_grid().get_visibility(coord);
    let data = &game.visibility_grid().get_data(coord);
    if let Some(data) = data{
        let verb = match visibility {
            CellVisibility::Never => None,
            CellVisibility::Previous(..) => Some(MessageVerb::Remember),
            CellVisibility::Current { .. } => Some(MessageVerb::See),
        };
        
        if let Some(verb) = verb {
            data.tiles.option_for_each(|tile| {
                entity_under_cursor = Some((*tile, verb));
            });
        }
        
    }


    entity_under_cursor.and_then(|(tile, verb)| {
        tile_str(tile).map(|label| match label {
            TileLabel::Literal(literal) => StyledString::plain_text(literal),
            TileLabel::Name(name) => {
                let verb_str = match verb {
                    MessageVerb::See => "see",
                    MessageVerb::Remember => "remember seeing",
                };
                StyledString::plain_text(format!("You {} {} here.", verb_str, name))
            }
        })
    })
}

enum TileLabel {
    Literal(String),
    Name(String),
}

fn tile_str(tile: Tile) -> Option<TileLabel> {
    let desc = default_tile_str(tile).unwrap_or_default().to_string();
   matches!(
        tile,
        Tile::Player
            | Tile::DoorClosed
            | Tile::DoorOpen
            | Tile::Floor
            | Tile::Wall
    )
    .then(|| TileLabel::Name(desc.clone()))
    .or_else(|| matches!(
        tile,
        Tile::Weapon(..)
        )
        .then(|| TileLabel::Literal(desc.clone())))
}

fn default_tile_str(tile: Tile) -> Option<&'static str> {
    Some(match tile {
        Tile::Player => "yourself",
        Tile::Floor | Tile::CaveFloor => "the floor",
        Tile::DoorClosed | Tile::DoorOpen => "a door",
        Tile::Wall | Tile::CaveWall => "a wall",
        Tile::Weapon(wpn) => match wpn{
            WeaponType::CattleProd => "A cattle prod - can stun low-level enemies",
            WeaponType::Chainsaw => "A chainsaw - melee weapon with high DMG and limited uses.",
            WeaponType::Railgun => "A railgun - it can shoot through almost anything!",
            WeaponType::LifeStealer => "A life stealer - converts the recently deceased into health like some kind of creepy vampire. And you thought the zombies were gross!",
            WeaponType::BareHands => return None,
            
        }
        Tile::Npc(npc_type) => match npc_type{
            NpcType::MiniBot => "a mini-bot",
            NpcType::SecBot => "a sec-bot. an upgraded bot from mini-bot.",
            NpcType::RoboCop => "a robo-cop. a security bot for the <blank>",
            NpcType::DoomBot => "a doom-bot. its only purpose is to kill"
        }
        Tile::Laser => return None,
        Tile::Grass => "dense patch of grass",
        Tile::GrassCrushed => "crushed grass",
        Tile::Water => "some lovely cave water",
    })
}

//////////////////////////////////////////////////////////////////////////////////////////////
/// Examione Component
//////////////////////////////////////////////////////////////////////////////////////////////

struct GameExamineComponent;

impl Component for GameExamineComponent {
    type Output = Option<()>;
    type State = GameLoopData;

    fn render(&self, state: &Self::State, ctx: Ctx, fb: &mut FrameBuffer) {
        state.render(CURSOR.with_a(128), ctx, fb);
    }

    fn update(&mut self, state: &mut Self::State, _ctx: Ctx, event: Event) -> Self::Output {
        if let Some(input) = event.input() {
            if let Some(direction) = state.controls.get_direction(input) {
                let cursor = state.cursor.unwrap_or_else(|| state.scope().player_coord());
                state.cursor = Some(cursor + direction.coord());
            }

            if let Some(AppInput::Examine) = state.controls.get(input) {
                return Some(());
            }
        }
        state.examine_mouse(event);
        state.update_examine_text();
        None
    }

    fn size(&self, _state: &Self::State, ctx: Ctx) -> Size {
        ctx.bounding_box.size()
    }
}

pub fn game_examine_component() -> AppCF<()> {
    on_state_then(|state: &mut State| {
        state.context_message = Some(StyledString {
            string: "Examining (escape/start to return to game)".to_string(),
            style: Style::plain_text(),
        });

        let cursor = state.cursor.unwrap_or_else(|| state.scope().player_coord());
        state.cursor = Some(cursor);

        cf(GameExamineComponent).catch_escape_or_start().map_val(|| ()).side_effect(|state: &mut State| {
            state.context_message = None;
            state.cursor = None;
        })
    })
}

