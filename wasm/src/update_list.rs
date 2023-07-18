use std::mem;
use std::ops::Bound::{self, Unbounded};
use std::slice::Iter;
use na::{point, Point2};
use py::{Holds, Walkable};
use crate::utils::cmp_xy_order;

/// Manages cells to update
#[derive(Clone, Debug)]
pub struct UpdateList {
    area: UpdateArea,
    cells: Vec<Point2<i32>>,
}

// Types
pub type UpdateArea = (Bound<Point2<i32>>, Bound<Point2<i32>>);

// Methods
impl UpdateList {
    /// Creates an empty update list
    #[inline]
    pub fn new() -> UpdateList {
        UpdateList {
            area: (Unbounded, Unbounded),
            cells: Vec::new(),
        }
    }

    /// Creates an empty update list
    #[inline]
    pub fn inside(area: UpdateArea) -> UpdateList {
        UpdateList {
            area,
            cells: Vec::new(),
        }
    }

    /// Return current area limit
    #[inline]
    pub fn area(&self) -> &UpdateArea {
        &self.area
    }

    /// Changes update area and return current one
    #[inline]
    pub fn change_area(&mut self, area: UpdateArea) -> UpdateArea {
        mem::replace(&mut self.area, area)
    }

    /// Iterates on registered cells
    #[inline]
    pub fn iter(&self) -> Iter<'_, Point2<i32>> {
        self.cells.iter()
    }

    /// Register given point
    pub fn register(&mut self, cell: Point2<i32>) {
        if let Err(idx) = self.cells.binary_search_by(|pt| cmp_xy_order(pt, &cell)) {
            self.cells.insert(idx, cell);
        }
    }

    /// Register given point and neighbors
    pub fn register_with_neighbors(&mut self, cell: Point2<i32>) {
        if !self.area.holds(&cell) {
            return;
        }

        let neighbors = point![cell.x - 1, cell.y - 1]..=point![cell.x + 1, cell.y + 1];
        self.cells.reserve(9);

        for cell in &neighbors.walk().unwrap() {
            self.register(cell);
        }
    }
}
