use std::fmt;

use crate::{coordinate_system::CoordinateSystem, direction::Direction};

#[cfg(feature = "bevy")]
use bevy::ecs::component::Component;
#[cfg(feature = "reflect")]
use bevy::{ecs::reflect::ReflectComponent, reflect::Reflect};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Right-handed 2d Cartesian coordinate system: 4 directions
#[derive(Default, Debug, Clone, Copy)]
#[cfg_attr(feature = "bevy", derive(Component))]
#[cfg_attr(feature = "reflect", derive(Reflect), reflect(Component))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
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
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
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

/// Represents a displacement on a grid
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "bevy", derive(Component))]
#[cfg_attr(feature = "reflect", derive(Reflect), reflect(Component))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct GridDelta {
    /// Amount of movement on the X axis
    pub dx: i32,
    /// Amount of movement on the Y axis
    pub dy: i32,
    /// Amount of movement on the Z axis
    pub dz: i32,
}

impl GridDelta {
    /// Creates a new [`GridDelta`]
    pub fn new(dx: i32, dy: i32, dz: i32) -> Self {
        Self { dx, dy, dz }
    }
}

impl std::ops::Mul<i32> for GridDelta {
    type Output = GridDelta;
    fn mul(self, rhs: i32) -> GridDelta {
        GridDelta {
            dx: self.dx * rhs,
            dy: self.dy * rhs,
            dz: self.dz * rhs,
        }
    }
}

/// Specific case for a cartesian coordinate system
pub trait CartesianCoordinates: CoordinateSystem<Direction = Direction> {
    /// Returns the [`GridDelta`] for each direction in this coordinate system
    fn deltas(&self) -> &'static [GridDelta];
}

/// Represents a position in a grid in a practical format
#[derive(Default, Hash, Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "bevy", derive(Component))]
#[cfg_attr(feature = "reflect", derive(Reflect), reflect(Component))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct CartesianPosition {
    /// Position on the x axis
    pub x: u32,
    /// Position on the y axis
    pub y: u32,
    /// Position on the z axis
    pub z: u32,
}
impl CartesianPosition {
    pub(crate) fn get_delta_position(&self, delta: &GridDelta) -> (i64, i64, i64) {
        (
            i64::from(self.x) + i64::from(delta.dx),
            i64::from(self.y) + i64::from(delta.dy),
            i64::from(self.z) + i64::from(delta.dz),
        )
    }

    /// Utility constructor
    pub fn new(x: u32, y: u32, z: u32) -> Self {
        Self { x, y, z }
    }

    /// Utility constructor for a 2D (x,y) position. z will be set to 0
    pub fn new_xy(x: u32, y: u32) -> Self {
        Self { x, y, z: 0 }
    }
}

impl From<(u32, u32)> for CartesianPosition {
    fn from(xy: (u32, u32)) -> Self {
        CartesianPosition::new(xy.0, xy.1, 0)
    }
}
impl From<(u32, u32, u32)> for CartesianPosition {
    fn from(xyz: (u32, u32, u32)) -> Self {
        CartesianPosition::new(xyz.0, xyz.1, xyz.2)
    }
}

impl fmt::Display for CartesianPosition {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "x: {}, y: {}, z: {}", self.x, self.y, self.z)
    }
}
