use crate::direction::DirectionTrait;

#[cfg(feature = "bevy")]
use bevy::ecs::component::Component;
#[cfg(feature = "reflect")]
use bevy::{ecs::reflect::ReflectComponent, reflect::Reflect};

/// Represents a coordinate system
pub trait CoordinateSystem: Default + Clone + Sync + Send + 'static {
    /// [DirectionTrait] type used in this system
    type Direction: DirectionTrait;

    /// Returns the [`DirectionTrait`] used in this coordinate system
    fn directions(&self) -> &'static [Self::Direction];

    /// Returns the total count of directions
    fn directions_count(&self) -> usize;
}
