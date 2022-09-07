use gridbugs::spatial_table;

spatial_table::declare_layers_module! {
    layers {
        item: Item,
        floor: Floor,
        feature: Feature,
        character: Character,
    }
}
pub use layers::{Layer, Layers};
pub use spatial_table::UpdateError;
pub type SpatialTable = spatial_table::SpatialTable<Layers>;
pub type Location = spatial_table::Location<Layer>;
