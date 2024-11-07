use std::{fmt, ops::Range};

use crate::{
    coordinate_system::CoordinateSystem,
    direction::Direction,
    grid::{Grid, GridData, GridIndex},
};

use super::coordinates::{
    Cartesian2D, Cartesian3D, CartesianCoordinates, CartesianPosition, GridDelta,
};

#[cfg(feature = "bevy")]
use bevy::ecs::component::Component;
#[cfg(feature = "reflect")]
use bevy::{ecs::reflect::ReflectComponent, reflect::Reflect};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Definition of a grid
#[derive(Clone)]
#[cfg_attr(feature = "bevy", derive(Component, Default))]
#[cfg_attr(feature = "reflect", derive(Reflect), reflect(Component))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct CartesianGrid<C: CoordinateSystem> {
    size_x: u32,
    size_y: u32,
    size_z: u32,
    looping_x: bool,
    looping_y: bool,
    looping_z: bool,
    pub(crate) coord_system: C,
    /// Cache value of `size_x` * `size_y` for index computations
    size_xy: u32,
}

impl<C: CartesianCoordinates> Grid<C> for CartesianGrid<C> {
    type Position = CartesianPosition;

    /// Returns the total size of the grid
    #[inline]
    fn total_size(&self) -> usize {
        (self.size_xy * self.size_z).try_into().unwrap()
    }

    #[inline]
    fn directions_count(&self) -> usize {
        self.coord_system.directions().len()
    }

    #[inline]
    fn coord_system(&self) -> &C {
        &self.coord_system
    }

    /// Will retrieve the next index in each direction.
    ///
    /// - `neighbours_buffer` should be allocated by the caller and its size should be >= to `directions.len()`
    fn get_neighbours_in_all_directions(
        &self,
        grid_index: GridIndex,
        neighbours_buffer: &mut Vec<Option<GridIndex>>,
    ) {
        let pos = self.pos_from_index(grid_index);
        for dir in self.coord_system.directions() {
            neighbours_buffer[usize::from(*dir)] = self.get_next_index_in_direction(&pos, *dir);
        }
    }

    /// Returns a [`CartesianPosition`] from the index of an element in this [`CartesianGrid`].
    ///
    /// Panics if the index is not a valid index.
    #[inline]
    fn pos_from_index(&self, grid_index: GridIndex) -> CartesianPosition {
        let index = u32::try_from(grid_index).unwrap();
        CartesianPosition {
            x: index % self.size_x,
            y: (index / self.size_x) % self.size_y,
            z: index / self.size_xy,
        }
    }

    /// Returns the index from a grid position.
    ///
    /// NO CHECK is done to verify that the given `grid_position` is a valid position for this grid.
    #[inline]
    fn index_from_pos(&self, grid_position: &CartesianPosition) -> GridIndex {
        self.index_from_coords(grid_position.x, grid_position.y, grid_position.z)
    }
}

impl<C: CartesianCoordinates> fmt::Display for CartesianGrid<C> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "( size: {} {} {}, looping: {} {} {} )",
            self.size_x, self.size_y, self.size_z, self.looping_x, self.looping_y, self.looping_z
        )
    }
}

impl CartesianGrid<Cartesian2D> {
    /// Creates a new grid with a [`Cartesian2D`] coordinate system
    ///
    /// Use `looping` to specify if the coordinates on an axis should loop when reaching the end of the axis.
    pub fn new_cartesian_2d(
        size_x: u32,
        size_y: u32,
        looping_x: bool,
        looping_y: bool,
    ) -> CartesianGrid<Cartesian2D> {
        Self::new(size_x, size_y, 1, looping_x, looping_y, false, Cartesian2D)
    }

    /// Returns the index from a grid position, ignoring the Z axis.
    ///
    ///  NO CHECK is done to verify that the given position is a valid position for this grid.
    #[inline]
    pub fn get_index_2d(&self, x: u32, y: u32) -> GridIndex {
        (x + y * self.size_x).try_into().unwrap()
    }

    /// Returns the index from a grid position, ignoring the Z axis.
    ///
    ///  NO CHECK is done to verify that the given position is a valid position for this grid.
    #[inline]
    pub fn get_index_from_pos_2d(&self, grid_position: &CartesianPosition) -> GridIndex {
        self.get_index_2d(grid_position.x, grid_position.y)
    }
}

impl CartesianGrid<Cartesian3D> {
    /// Creates a new grid with a [`Cartesian3D`] coordinate system
    ///
    /// Use `looping` to specify if the coordinates on an axis should loop when reaching the end of the axis.
    pub fn new_cartesian_3d(
        size_x: u32,
        size_y: u32,
        size_z: u32,
        looping_x: bool,
        looping_y: bool,
        looping_z: bool,
    ) -> CartesianGrid<Cartesian3D> {
        Self::new(
            size_x,
            size_y,
            size_z,
            looping_x,
            looping_y,
            looping_z,
            Cartesian3D,
        )
    }
}

impl<C: CartesianCoordinates> CartesianGrid<C> {
    /// Creates a new [`CartesianGrid`]
    pub fn new(
        size_x: u32,
        size_y: u32,
        size_z: u32,
        looping_x: bool,
        looping_y: bool,
        looping_z: bool,
        coord_system: C,
    ) -> CartesianGrid<C> {
        Self {
            size_x,
            size_y,
            size_z,
            looping_x,
            looping_y,
            looping_z,
            coord_system,
            size_xy: size_x * size_y,
        }
    }

    /// Returns the size of the grid in the X axis.
    #[inline]
    pub fn size_x(&self) -> u32 {
        self.size_x
    }

    /// Returns the size of the grid in the Y axis.
    #[inline]
    pub fn size_y(&self) -> u32 {
        self.size_y
    }

    /// Returns the size of the grid in the Z axis.
    #[inline]
    pub fn size_z(&self) -> u32 {
        self.size_z
    }

    /// Returns th value of `size_x` * `size_y
    #[inline]
    pub fn size_xy(&self) -> u32 {
        self.size_xy
    }

    /// Returns the size of this grid as a tuple
    #[inline]
    pub fn size(&self) -> (u32, u32, u32) {
        (self.size_x, self.size_y, self.size_z)
    }

    /// Returns a [`Range`] over all indexes in this grid
    #[inline]
    pub fn indexes(&self) -> Range<GridIndex> {
        0..self.total_size()
    }

    /// Returns all the the [`CoordinateSystem`] used by this [`CartesianGrid`]
    #[inline]
    pub fn coord_system(&self) -> &C {
        &self.coord_system
    }

    /// Returns the index from a grid position.
    ///
    /// NO CHECK is done to verify that the given position is a valid position for this grid.
    #[inline]
    pub fn index_from_coords(&self, x: u32, y: u32, z: u32) -> GridIndex {
        (x + y * self.size_x + z * self.size_xy).try_into().unwrap()
    }

    /// Returns the index from a grid position.
    ///
    /// NO CHECK is done to verify that the given `grid_position` is a valid position for this grid.
    #[inline]
    pub fn index_from_pos(&self, grid_position: &CartesianPosition) -> GridIndex {
        self.index_from_coords(grid_position.x, grid_position.y, grid_position.z)
    }

    /// Returns a [`GridPosition`] from the index of an element in this [`GridDefinition`].
    ///
    /// Panics if the index is not a valid index.
    #[inline]
    pub fn pos_from_index(&self, grid_index: GridIndex) -> CartesianPosition {
        let index = u32::try_from(grid_index).unwrap();
        CartesianPosition {
            x: index % self.size_x,
            y: (index / self.size_x) % self.size_y,
            z: index / self.size_xy,
        }
    }

    /// Returns the index of the next position in the grid when moving 1 unit in `direction` from `grid_position`.
    ///
    /// Returns `None` if the destination is not in the grid.
    ///
    /// NO CHECK is done to verify that the given `grid_position` is a valid position for this grid.
    pub fn get_next_index_in_direction(
        &self,
        grid_position: &CartesianPosition,
        direction: Direction,
    ) -> Option<GridIndex> {
        let delta = &self.coord_system.deltas()[direction as usize];
        match self.get_next_pos(grid_position, &delta) {
            Some(next_pos) => Some(self.index_from_pos(&next_pos)),
            None => None,
        }
    }

    /// Returns the index of the next position in the grid when moving `units` in `direction` from `grid_position`.
    ///
    /// Returns `None` if the destination is not in the grid.
    ///
    /// NO CHECK is done to verify that the given `grid_position` is a valid position for this grid.
    pub fn get_index_in_direction(
        &self,
        grid_position: &CartesianPosition,
        direction: Direction,
        units: i32,
    ) -> Option<GridIndex> {
        let delta = self.coord_system.deltas()[direction as usize].clone() * units;
        match self.get_next_pos(grid_position, &delta) {
            Some(next_pos) => Some(self.index_from_pos(&next_pos)),
            None => None,
        }
    }

    /// Returns the the next position in the grid when moving 1 unit in `direction` from `grid_position`.
    ///
    /// Returns `None` if the destination is not in the grid.
    ///
    /// NO CHECK is done to verify that the given `grid_position` is a valid position for this grid.
    pub fn get_next_pos_in_direction(
        &self,
        grid_position: &CartesianPosition,
        direction: Direction,
    ) -> Option<CartesianPosition> {
        let delta = &self.coord_system.deltas()[direction as usize];
        match self.get_next_pos(grid_position, &delta) {
            Some(next_pos) => Some(next_pos),
            None => None,
        }
    }

    /// Returns the next position in the grid when moving `delta` unit(s) in `direction` from `grid_position`.
    ///
    /// Returns `None` if the destination is not in the grid.
    ///
    /// NO CHECK is done to verify that the given `grid_position` is a valid position for this grid.
    pub fn get_next_pos(
        &self,
        grid_position: &CartesianPosition,
        delta: &GridDelta,
    ) -> Option<CartesianPosition> {
        let mut next_pos = grid_position.get_delta_position(&delta);
        for (looping, pos, size) in vec![
            (self.looping_x, &mut next_pos.0, self.size_x),
            (self.looping_y, &mut next_pos.1, self.size_y),
            (self.looping_z, &mut next_pos.2, self.size_z),
        ] {
            match looping {
                true => {
                    if *pos < 0 {
                        *pos += size as i64
                    }
                    if *pos >= size as i64 {
                        *pos -= size as i64
                    }
                }
                false => {
                    if *pos < 0 || *pos >= size as i64 {
                        return None;
                    }
                }
            }
        }
        Some(CartesianPosition {
            x: u32::try_from(next_pos.0).unwrap(),
            y: u32::try_from(next_pos.1).unwrap(),
            z: u32::try_from(next_pos.2).unwrap(),
        })
    }

    /// Creates a default [`GridData`] with the size of the [`CartesianGrid`] with each element value set to its default one.
    pub fn default_grid_data<D: Default + Clone>(&self) -> GridData<C, D, CartesianGrid<C>> {
        GridData::new(self.clone(), vec![D::default(); self.total_size()])
    }

    /// Creates a [`GridData`] with the size of the [`CartesianGrid`] with each element value being a copy of the given one.
    pub fn new_grid_data<D: Clone>(&self, element: D) -> GridData<C, D, CartesianGrid<C>> {
        GridData::new(self.clone(), vec![element; self.total_size()])
    }
}
