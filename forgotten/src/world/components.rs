use crate::prelude::*;
use gridbugs::entity_table;

entity_table::declare_entity_module! {
  components {
      tile: Tile,
      opacity: u8,
      solid: (),
      light: Light,

      map: (),
  }
}
pub use components::Components;
pub use components::EntityData;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
pub enum Tile {
    Wall,
    Floor,
    Player,
}
