use bevy::prelude::*;
use std::{collections::HashMap, vec::IntoIter};

/// A simple component used to keep track of layer entities.
#[derive(Clone)]
pub struct Map {
    pub map_entity: Entity,
    pub id: u16,
    pub(crate) layers: HashMap<u16, Entity>,
}

impl Default for Map {
    fn default() -> Self {
        Self {
            map_entity: Entity::new(0),
            id: 0,
            layers: HashMap::new(),
        }
    }
}

impl Map {
    /// Creates a new map component
    pub fn new<M: Into<u16>>(id: M, map_entity: Entity) -> Self {
        Self {
            map_entity,
            id: id.into(),
            layers: HashMap::new(),
        }
    }

    /// Creates a new layer.
    pub fn add_layer<L: Into<u16>>(
        &mut self,
        commands: &mut Commands,
        layer_id: L,
        layer_entity: Entity,
    ) {
        commands
            .entity(self.map_entity)
            .push_children(&[layer_entity]);
        self.layers.insert(layer_id.into(), layer_entity);
    }

    /// Adds multiple layers to the map.
    pub fn add_layers<L: Into<u16>>(
        &mut self,
        commands: &mut Commands,
        layers: IntoIter<(L, Entity)>,
    ) {
        let layers: Vec<(u16, Entity)> = layers.map(|(id, entity)| (id.into(), entity)).collect();
        let entities: Vec<Entity> = layers.iter().map(|(_, entity)| *entity).collect();
        self.layers.extend(layers);
        commands.entity(self.map_entity).push_children(&entities);
    }

    /// Removes the layer from the map and despawns the layer entity.
    /// Note: Does not despawn the tile entities. Please use MapQuery instead.
    pub fn remove_layer<L: Into<u16>>(&mut self, commands: &mut Commands, layer_id: L) {
        if let Some(layer_entity) = self.layers.remove(&layer_id.into()) {
            commands.entity(layer_entity).despawn_recursive();
        }
    }

    /// Removes the layers from the map and despawns the layer entities.
    /// Note: Does not despawn the tile entities. Please use MapQuery instead.
    pub fn remove_layers<L: Into<u16>>(&mut self, commands: &mut Commands, layers: IntoIter<L>) {
        layers.for_each(|id| {
            let id: u16 = id.into();
            self.remove_layer(commands, id);
        });
    }

    /// Retrieves the entity for a given layer id.
    pub fn get_layer_entity<L: Into<u16>>(&self, layer_id: L) -> Option<&Entity> {
        self.layers.get(&layer_id.into())
    }

    /// Despawns a map. Better to call `map_query.despawn_map` as it will despawn layers/tiles as well.
    pub fn despawn(&self, commands: &mut Commands) {
        commands.entity(self.map_entity).despawn_recursive();
    }

    pub fn get_layers(&self) -> Vec<(u16, Entity)> {
        self.layers
            .iter()
            .map(|(key, value)| (*key, *value))
            .collect()
    }
}
