use gridbugs::spatial_table;

spatial_table::declare_layers_module! {
  layers {
      character: Character,
      feature: Feature,
      floor: Floor,
  }
}

pub use layers::{Layer, LayerTable, Layers};
pub type SpatialTable = spatial_table::SpatialTable<Layers>;
pub type Location = spatial_table::Location<Layer>;
