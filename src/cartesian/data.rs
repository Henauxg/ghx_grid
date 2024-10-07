use crate::grid::GridData;

use super::{
    coordinates::{Cartesian2D, Cartesian3D, CartesianCoordinates, CartesianPosition},
    grid::CartesianGrid,
};

/// Uses Copy if possible.
impl<C: CartesianCoordinates, D: Clone> GridData<C, D, CartesianGrid<C>> {
    /// Sets all nodes of the grix with x=`x` to `value`
    pub fn set_all_x(&mut self, x: u32, value: D) {
        let mut index = x;
        for _z in 0..self.grid().size_z() {
            for _y in 0..self.grid().size_y() {
                self.set_raw(index as usize, value.clone());
                index += self.grid().size_x();
            }
        }
    }

    /// Sets all nodes of the grix with y=`y` to `value`
    pub fn set_all_y(&mut self, y: u32, value: D) {
        let mut index = y * self.grid().size_x();
        for _z in 0..self.grid().size_z() {
            for _x in 0..self.grid().size_x() {
                self.set_raw(index as usize, value.clone());
                index += 1;
            }
            index += self.grid().size_xy() - self.grid().size_x();
        }
    }
    /// Sets all nodes of the grix with z=`z` to `value`
    pub fn set_all_z(&mut self, z: u32, value: D) {
        let mut index = z * self.grid().size_xy();
        for _y in 0..self.grid().size_y() {
            for _x in 0..self.grid().size_x() {
                self.set_raw(index as usize, value.clone());
                index += 1;
            }
        }
    }

    /// Sets all nodes of the grix with x=`x`and y=`y` to `value`
    pub fn set_all_xy(&mut self, x: u32, y: u32, value: D) {
        let mut index = x + y * self.grid().size_x();
        for _z in 0..self.grid().size_z() {
            self.set_raw(index as usize, value.clone());
            index += self.grid().size_xy();
        }
    }

    /// Sets all nodes of the grix with x=`x`and z=`z` to `value`
    pub fn set_all_xz(&mut self, x: u32, z: u32, value: D) {
        let mut index = x + z * self.grid().size_xy();
        for _y in 0..self.grid().size_y() {
            self.set_raw(index as usize, value.clone());
            index += self.grid().size_x();
        }
    }

    /// Sets all nodes of the grix with y=`y` and z=`z` to `value`
    pub fn set_all_yz(&mut self, y: u32, z: u32, value: D) {
        let mut index = y * self.grid().size_x() + z * self.grid().size_xy();
        for _x in 0..self.grid().size_x() {
            self.set_raw(index as usize, value.clone());
            index += 1;
        }
    }

    /// Returns a reference to the element at this position.
    ///
    /// NO CHECK is done to verify that the given position is a valid position for this grid.
    #[inline]
    pub fn get_from_pos(&self, pos: &CartesianPosition) -> &D {
        &self.get(self.grid().index_from_pos(pos))
    }

    /// Returns a reference to the element at this position.
    ///
    /// NO CHECK is done to verify that the given position is a valid position for this grid.
    #[inline]
    pub fn get_mut_from_pos(&mut self, pos: &CartesianPosition) -> &mut D {
        self.get_mut(self.grid().index_from_pos(pos))
    }
}

impl<D> GridData<Cartesian2D, D, CartesianGrid<Cartesian2D>> {
    /// Returns a reference to the element at this position.
    ///
    /// NO CHECK is done to verify that the given position is a valid position for this grid.
    #[inline]
    pub fn get_2d(&self, x: u32, y: u32) -> &D {
        &self.get(self.grid().get_index_2d(x, y))
    }

    /// Returns a mutable reference to the data at this position.
    ///
    /// NO CHECK is done to verify that the given position is a valid position for this grid.
    #[inline]
    pub fn get_2d_mut(&mut self, x: u32, y: u32) -> &mut D {
        self.get_mut(self.grid().get_index_2d(x, y))
    }
}

impl<D> GridData<Cartesian3D, D, CartesianGrid<Cartesian3D>> {
    /// Returns a reference to the data at this position.
    ///
    /// NO CHECK is done to verify that the given position is a valid position for this grid.
    #[inline]
    pub fn get_3d(&self, x: u32, y: u32, z: u32) -> &D {
        &self.get(self.grid().index_from_coords(x, y, z))
    }

    /// Returns a mutable reference to the data at this position.
    ///
    /// NO CHECK is done to verify that the given position is a valid position for this grid.
    #[inline]
    pub fn get_3d_mut(&mut self, x: u32, y: u32, z: u32) -> &mut D {
        self.get_mut(self.grid().index_from_coords(x, y, z))
    }
}
