use std::{
    fmt::Debug,
    marker::PhantomData,
    slice::{Iter, IterMut},
};

use crate::coordinate_system::CoordinateSystem;

#[cfg(feature = "bevy")]
use bevy::ecs::component::Component;
#[cfg(feature = "reflect")]
use bevy::{ecs::reflect::ReflectComponent, reflect::Reflect};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

// TODO Might reduce this to u32 by default, with a feature for u64
/// Index of a grid element
pub type GridIndex = usize;

/// Generic trait to represent a grid
pub trait Grid<C: CoordinateSystem>: Clone {
    /// Position type used in this grid layout. Can be [GridIndex] if the grid elements have no position.
    type Position: Debug;

    /// Returns the [CoordinateSystem] used by this grid
    fn coord_system(&self) -> &C;

    /// Returns the maximum number of neighbours of an element in this grid
    fn directions_count(&self) -> usize;

    /// Returns the total size of the grid
    fn total_size(&self) -> usize;

    /// Will retrieve the next element's indexes in each direction.
    ///
    /// - `neighbours_buffer` should be allocated by the caller and its size should be >= to `directions.len()`
    fn get_neighbours_in_all_directions(
        &self,
        grid_index: GridIndex,
        neighbours_buffer: &mut Vec<Option<GridIndex>>,
    );

    /// Converts a position into an index
    fn index_from_pos(&self, pos: &Self::Position) -> GridIndex;
    /// Converts an index into an position
    fn pos_from_index(&self, index: GridIndex) -> Self::Position;
}

/// Holds a [`Grid`] and generic data in a linear buffer that can be accessed through the grid definition to represent the grid content.
#[derive(Clone)]
#[cfg_attr(feature = "bevy", derive(Component, Default))]
#[cfg_attr(feature = "reflect", derive(Reflect), reflect(Component))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct GridData<C, D, G>
where
    C: CoordinateSystem,
    G: Grid<C>,
{
    grid: G,
    data: Vec<D>,
    #[cfg_attr(feature = "reflect", reflect(ignore))]
    #[cfg_attr(feature = "serde", serde(skip))]
    _phantom: PhantomData<C>,
}

impl<C, D, G> GridData<C, D, G>
where
    C: CoordinateSystem,
    G: Grid<C>,
{
    /// Prefer using `default_grid_data` or `new_grid_data` directly on an existing grid definition to create a `GridData` with a correct data Vec.
    #[inline]
    pub fn new(grid: G, data: Vec<D>) -> Self {
        Self {
            grid,
            data,
            _phantom: PhantomData,
        }
    }

    /// Returns a reference to the `GridDefinition` this is based on
    #[inline]
    pub fn grid(&self) -> &G {
        &self.grid
    }

    /// Sets the value of the element at `index` in the grid.
    ///
    /// NO CHECK is done to verify that the given index is a valid index for this grid.
    #[inline]
    pub fn set_raw(&mut self, index: GridIndex, value: D) {
        self.data[index] = value;
    }

    /// Sets the value of the element at `index_ref` in the grid.
    ///
    /// NO CHECK is done to verify that the given index is a valid index for this grid.
    #[inline]
    pub fn set<N: NodeRef<C, G>>(&mut self, index_ref: N, value: D) {
        self.data[index_ref.to_index(&self.grid)] = value;
    }

    /// Returns a reference to the element at this index.
    ///
    /// NO CHECK is done to verify that the given index is a valid index for this grid.
    #[inline]
    pub fn get(&self, index: GridIndex) -> &D {
        &self.data[index]
    }

    /// Returns a mutable reference to the element at this index.
    ///
    /// NO CHECK is done to verify that the given index is a valid index for this grid.
    #[inline]
    pub fn get_mut(&mut self, index: GridIndex) -> &mut D {
        &mut self.data[index]
    }

    /// Returns an iterator over all the elements.
    #[inline]
    pub fn iter(&self) -> Iter<'_, D> {
        self.data.iter()
    }

    /// Returns an iterator over all the elements that allows modifying each value.
    #[inline]
    pub fn iter_mut(&mut self) -> IterMut<'_, D> {
        self.data.iter_mut()
    }

    /// Returns a range of all the [GridIndex] in this grid.
    #[inline]
    pub fn indexes(&self) -> std::ops::Range<usize> {
        0..self.grid.total_size()
    }
}

impl<C: CoordinateSystem, D: Clone, G: Grid<C>> GridData<C, D, G> {
    /// Resets the whole grid buffer by setting the value of each element to `value`
    pub fn reset(&mut self, value: D) {
        for d in self.data.iter_mut() {
            *d = value.clone();
        }
    }
}

/// Represents a reference to an element of a [`Grid`] or [`GridData`]
pub trait NodeRef<C: CoordinateSystem, G: Grid<C>> {
    /// Returns the [`GridIndex`] that is referenced by this `NodeRef`.
    fn to_index(&self, grid: &G) -> GridIndex;
}

impl<C: CoordinateSystem, G: Grid<C>> NodeRef<C, G> for GridIndex {
    #[inline]
    fn to_index(&self, _grid: &G) -> GridIndex {
        *self
    }
}
