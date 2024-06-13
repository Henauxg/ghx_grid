use crate::direction::{Direction, DirectionTrait, GridDelta};

#[cfg(feature = "bevy")]
use bevy::ecs::component::Component;
#[cfg(feature = "reflect")]
use bevy::{ecs::reflect::ReflectComponent, reflect::Reflect};

/// Represents a coordinate system
pub trait CoordinateSystem: Default + Clone + Sync + Send + 'static {
    /// [DirectionTrait] type used in this system
    type Direction: DirectionTrait;

    /// Returns the [`Direction`] in this coordinate system
    fn directions(&self) -> &'static [Self::Direction];

    /// Returns the total count of directions
    fn directions_count(&self) -> usize;
}

/// Specific case for a cartesian coordinate system
pub trait CartesianCoordinates: CoordinateSystem<Direction = Direction> {
    /// Returns the [`GridDelta`] for each direction in this coordinate system
    fn deltas(&self) -> &'static [GridDelta];
}

/// Right-handed 2d Cartesian coordinate system: 4 directions
#[derive(Default, Debug, Clone, Copy)]
#[cfg_attr(feature = "bevy", derive(Component))]
#[cfg_attr(feature = "reflect", derive(Reflect), reflect(Component))]
pub struct Cartesian2D;
impl CoordinateSystem for Cartesian2D {
    type Direction = Direction;

    #[inline]
    fn directions(&self) -> &'static [Direction] {
        CARTESIAN_2D_DIRECTIONS
    }

    #[inline]
    fn directions_count(&self) -> usize {
        CARTESIAN_2D_DIRECTIONS.len()
    }
}
impl CartesianCoordinates for Cartesian2D {
    #[inline]
    fn deltas(&self) -> &'static [GridDelta] {
        CARTESIAN_2D_DELTAS
    }
}

/// Right-handed 3d Cartesian coordinate system: 6 directions
#[derive(Default, Debug, Clone, Copy)]
#[cfg_attr(feature = "bevy", derive(Component))]
#[cfg_attr(feature = "reflect", derive(Reflect), reflect(Component))]
pub struct Cartesian3D;
impl CoordinateSystem for Cartesian3D {
    type Direction = Direction;

    #[inline]
    fn directions(&self) -> &'static [Direction] {
        CARTESIAN_3D_DIRECTIONS
    }

    #[inline]
    fn directions_count(&self) -> usize {
        CARTESIAN_3D_DIRECTIONS.len()
    }
}
impl CartesianCoordinates for Cartesian3D {
    #[inline]
    fn deltas(&self) -> &'static [GridDelta] {
        CARTESIAN_3D_DELTAS
    }
}

/// All the directions that forms a 2d cartesian coordinate system
pub const CARTESIAN_2D_DIRECTIONS: &'static [Direction] = &[
    Direction::XForward,
    Direction::YForward,
    Direction::XBackward,
    Direction::YBackward,
];

/// All the [`GridDelta`], one for each direction, in a cartesian 2d coordinate system
pub const CARTESIAN_2D_DELTAS: &'static [GridDelta] = &[
    GridDelta {
        // XForward
        dx: 1,
        dy: 0,
        dz: 0,
    },
    GridDelta {
        // YForward
        dx: 0,
        dy: 1,
        dz: 0,
    },
    GridDelta {
        // XBackward
        dx: -1,
        dy: 0,
        dz: 0,
    },
    GridDelta {
        // YBackward
        dx: 0,
        dy: -1,
        dz: 0,
    },
];

/// All the directions that forms a 3d cartesian coordinate system
pub const CARTESIAN_3D_DIRECTIONS: &'static [Direction] = &[
    Direction::XForward,
    Direction::YForward,
    Direction::XBackward,
    Direction::YBackward,
    Direction::ZForward,
    Direction::ZBackward,
];

/// All the [`GridDelta`], one for each direction, in a cartesian 3d coordinate system
pub const CARTESIAN_3D_DELTAS: &'static [GridDelta] = &[
    GridDelta {
        // XForward
        dx: 1,
        dy: 0,
        dz: 0,
    },
    GridDelta {
        // YForward
        dx: 0,
        dy: 1,
        dz: 0,
    },
    GridDelta {
        // XBackward
        dx: -1,
        dy: 0,
        dz: 0,
    },
    GridDelta {
        // YBackward
        dx: 0,
        dy: -1,
        dz: 0,
    },
    GridDelta {
        // ZForward
        dx: 0,
        dy: 0,
        dz: 1,
    },
    GridDelta {
        // ZBackward
        dx: 0,
        dy: 0,
        dz: -1,
    },
];
