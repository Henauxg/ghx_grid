use std::collections::VecDeque;

use crate::{direction::Direction, grid::GridData};

use super::{
    coordinates::{Cartesian2D, Cartesian3D, CartesianCoordinates, CartesianPosition},
    grid::CartesianGrid,
};

impl<C: CartesianCoordinates, D> GridData<C, D, CartesianGrid<C>> {
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

    fn explore_vertical<C: FnMut(&D) -> bool, A: FnMut(&mut D)>(
        &mut self,
        queue: &mut VecDeque<CartesianPosition>,
        from: &CartesianPosition,
        condition: &mut C,
        action: &mut A,
    ) {
        for vertical_dir in vec![Direction::YForward, Direction::YBackward].iter() {
            if let Some(vertical_node_pos) =
                self.grid().get_next_pos_in_direction(&from, *vertical_dir)
            {
                let node_data = self.get_mut_from_pos(&vertical_node_pos);
                if condition(node_data) {
                    action(node_data);
                    queue.push_back(vertical_node_pos);
                }
            }
        }
    }

    // TODO Extend to Cartesian 3D
    // TODO See NodeRef for starting position

    /// Flood fill starting at `from`, applying `action` to all nodes for which `conditon` returns true.
    ///
    /// - `conditon`should be true for `from` else the function returns immediately.
    /// - If present `pre_allocated_queue` will be cleared before running the algorithm (but existing allocation will be kept)
    ///
    /// Based on <https://en.wikipedia.org/wiki/Flood_fill#Further_potential_optimizations> but working with looping grids. Some more optimizations may be taken from <https://en.wikipedia.org/wiki/Flood_fill#Span_filling> once adapted to looping grids.
    ///
    /// /!\ This uses 'conditon'+'action' as a way to not backtrack. If the effect of 'action' does not disables 'condition', this will loop !
    pub fn flood_fill<CO: FnMut(&D) -> bool, AC: FnMut(&mut D)>(
        &mut self,
        from: impl Into<CartesianPosition>,
        mut condition: CO,
        mut action: AC,
        pre_allocated_queue: Option<&mut VecDeque<CartesianPosition>>,
    ) {
        // We do not add to the queue if a node is already set. If not set, set and add to queue (to avoid queuing nodes multiple times)
        let mut queue = match pre_allocated_queue {
            Some(q) => {
                q.clear();
                q
            }
            None => &mut VecDeque::with_capacity(10),
        };

        let initial_pos = from.into();
        let initial_node = self.get_mut_from_pos(&initial_pos);
        if !condition(initial_node) {
            return;
        } else {
            action(initial_node);
            queue.push_back(initial_pos);
        }

        while let Some(pos) = queue.pop_front() {
            self.explore_vertical(&mut queue, &pos, &mut condition, &mut action);

            for &horizontal_dir in vec![Direction::XBackward, Direction::XForward].iter() {
                let mut x_pos = pos;

                // Use size_x as an upper limit of the iteration count
                for _ in 0..self.grid().size_x() {
                    // TODO Delta accessor helper: .delta(Direction::YForward)
                    if let Some(next_node_pos) = self
                        .grid()
                        .get_next_pos_in_direction(&x_pos, horizontal_dir)
                    {
                        let node_data = self.get_mut_from_pos(&next_node_pos);
                        if condition(node_data) {
                            action(node_data);
                            self.explore_vertical(
                                &mut queue,
                                &next_node_pos,
                                &mut condition,
                                &mut action,
                            );
                            x_pos = next_node_pos;
                        } else {
                            break;
                        }
                    } else {
                        break;
                    }
                }
            }
        }
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
