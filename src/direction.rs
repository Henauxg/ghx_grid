// TODO See if std::ops::index can be used here

#[cfg(feature = "bevy")]
use bevy::ecs::component::Component;
#[cfg(feature = "reflect")]
use bevy::{ecs::reflect::ReflectComponent, reflect::Reflect};

// TODO Might reduce this to a u8
/// Index of a direction
pub type DirectionIndex = usize;

// TODO, more generic
/// Represents a direction in a grid layout
pub trait DirectionTrait: Into<DirectionIndex> + Copy {
    /// Returns the opposite [`Direction`]
    fn opposite(&self) -> Self;
    /// Right-handed.
    fn rotation_basis(&self) -> &'static [Self];
}

/// Represents an oriented axis of a coordinate system
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "bevy", derive(Component))]
#[cfg_attr(feature = "reflect", derive(Reflect), reflect(Component))]
pub enum Direction {
    /// X+ axis
    #[default]
    XForward = 0,
    /// Y+ axis
    YForward = 1,
    /// X- axis
    XBackward = 2,
    /// Y- axis
    YBackward = 3,
    /// Z+ axis
    ZForward = 4,
    /// Z- axis
    ZBackward = 5,
}
impl DirectionTrait for Direction {
    /// Returns the opposite [`Direction`]
    fn opposite(&self) -> Direction {
        match self {
            Direction::XForward => Direction::XBackward,
            Direction::XBackward => Direction::XForward,
            Direction::YForward => Direction::YBackward,
            Direction::YBackward => Direction::YForward,
            Direction::ZForward => Direction::ZBackward,
            Direction::ZBackward => Direction::ZForward,
        }
    }

    /// Right-handed.
    fn rotation_basis(&self) -> &'static [Direction] {
        match self {
            Direction::XForward => X_POS_AXIS,
            Direction::XBackward => X_NEG_AXIS,
            Direction::YForward => Y_POS_AXIS,
            Direction::YBackward => Y_NEG_AXIS,
            Direction::ZForward => Z_POS_AXIS,
            Direction::ZBackward => Z_NEG_AXIS,
        }
    }
}
impl From<Direction> for usize {
    fn from(item: Direction) -> Self {
        item as Self
    }
}

pub(crate) const X_POS_AXIS: &'static [Direction] = &[
    Direction::YForward,
    Direction::ZForward,
    Direction::YBackward,
    Direction::ZBackward,
];
pub(crate) const X_NEG_AXIS: &'static [Direction] = &[
    Direction::ZForward,
    Direction::YForward,
    Direction::ZBackward,
    Direction::YBackward,
];
pub(crate) const Y_POS_AXIS: &'static [Direction] = &[
    Direction::ZForward,
    Direction::XForward,
    Direction::ZBackward,
    Direction::XBackward,
];
pub(crate) const Y_NEG_AXIS: &'static [Direction] = &[
    Direction::XForward,
    Direction::ZForward,
    Direction::XBackward,
    Direction::ZBackward,
];
pub(crate) const Z_POS_AXIS: &'static [Direction] = &[
    Direction::XForward,
    Direction::YForward,
    Direction::XBackward,
    Direction::YBackward,
];
pub(crate) const Z_NEG_AXIS: &'static [Direction] = &[
    Direction::YForward,
    Direction::XForward,
    Direction::YBackward,
    Direction::XBackward,
];

/// Represents a displacement on a grid
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "bevy", derive(Component))]
#[cfg_attr(feature = "reflect", derive(Reflect), reflect(Component))]
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
