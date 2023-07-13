use std::mem;
use js_sys::Math;
use na::{distance, point, Point2};
use py::{BBox, Holds, PointBounds, Walkable};
use py::wasm::{PointInt2D, VectorInt2D};
use wasm_bindgen::prelude::*;
use web_sys::CanvasRenderingContext2d;
use crate::binary_tree::BinaryTree;
use crate::universe_style::UniverseStyle;

#[cfg(feature = "quadtree")]
use crate::quadtree::Quadtree;

/// Life universe
#[cfg(feature = "binary-tree")]
#[derive(Clone)]
#[wasm_bindgen]
pub struct Universe {
    cells: BinaryTree,
    updates: BinaryTree,
    size: Vector2<usize>,
    style: UniverseStyle,
}

/// Life universe
#[cfg(feature = "quadtree")]
#[derive(Clone)]
#[wasm_bindgen]
pub struct Universe {
    cells: Quadtree,
    style: UniverseStyle,
    updates: BinaryTree,
    update_area: BBox<i32, 2>,
}

#[wasm_bindgen]
impl Universe {
    /// Builds a dead universe
    #[cfg(feature = "binary-tree")]
    pub fn dead(width: usize, height: usize) -> Universe {
        Universe {
            cells: BinaryTree::new(),
            updates: BinaryTree::new(),
            size: vector![width, height],
            style: UniverseStyle::default(),
        }
    }

    /// Builds a dead universe
    #[cfg(feature = "quadtree")]
    pub fn dead(size: VectorInt2D) -> Universe {
        let bbox = BBox::from_anchor_size(&Point2::origin(), size.as_ref());

        Universe {
            cells: Quadtree::new(),
            style: UniverseStyle::default(),
            updates: BinaryTree::new(),
            update_area: BBox::default(),
        }
    }

    /// Builds a fixed universe
    pub fn fixed(size: VectorInt2D) -> Universe {
        let mut universe = Universe::dead(size);

        for row in 0..size.dy() {
            for col in 0..size.dx() {
                let i = row * size.dx() + col;

                if i % 2 == 0 || i % 7 == 0 || i % 13 == 0 {
                    universe.set_alive(point![col, row])
                }
            }
        }

        universe
    }

    /// Builds a random universe
    pub fn random(size: VectorInt2D) -> Universe {
        let mut universe = Universe::dead(size);

        for row in 0..size.dy() {
            for col in 0..size.dx() {
                let rand = Math::random();

                if rand < 0.5 {
                    universe.set_alive(point![col, row])
                }
            }
        }

        universe
    }

    /// Inserts some cells around given position
    pub fn insert_around(&mut self, ctx: &CanvasRenderingContext2d, center: &PointInt2D, r: i32) {
        let center = center.as_ref();
        let area = point![center.x - r, center.y - r]..=point![center.x + r, center.y + r];

        area.walk().unwrap().iter()
            .filter(|cell| distance::<f64, 2>(&center.cast(), &cell.cast()) <= r as f64)
            .for_each(|cell| {
                let rand = Math::random();

                if rand < 0.25 {
                    self.set_alive(cell);

                    let pos = cell.cast::<f64>() * 5.0;

                    ctx.set_fill_style(self.style.alive_color());
                    ctx.fill_rect(pos.x, pos.y, 5.0, 5.0);
                }
            });
    }

    /// Compute next state
    pub fn tick(&mut self, ctx: &CanvasRenderingContext2d) {
        let old = Universe {
            cells: self.cells.clone(),
            style: self.style.clone(),
            updates: mem::replace(&mut self.updates, BinaryTree::new()),
            update_area: BBox::default(),
        };

        for &cell in old.updates.iter() {
            let (is_alive, neighbors) = old.cell_state(&cell);

            if is_alive {
                if !(2..=3).contains(&neighbors) {
                    self.set_dead(cell);

                    let pos = cell.cast::<f64>() * 5.0;

                    ctx.set_fill_style(self.style.dead_color());
                    ctx.fill_rect(pos.x, pos.y, 5.0, 5.0);
                }
            } else if neighbors == 3 {
                self.set_alive(cell);

                let pos = cell.cast::<f64>() * 5.0;

                ctx.set_fill_style(self.style.alive_color());
                ctx.fill_rect(pos.x, pos.y, 5.0, 5.0);
            }
        }
    }

    pub fn redraw(&self, ctx: &CanvasRenderingContext2d, size: VectorInt2D) {
        ctx.set_fill_style(self.style.dead_color());
        ctx.fill_rect(0.0, 0.0, size.dx() as f64, size.dy() as f64);

        let area = Point2::origin()..point![size.dx(), size.dy()];

        for cell in self.cells.query(area) {
            let pos = cell.cast::<f64>() * 5.0;

            ctx.set_fill_style(self.style.alive_color());
            ctx.fill_rect(pos.x, pos.y, 5.0, 5.0);
        }
    }

    pub fn set_update_area(&mut self, start: &PointInt2D, end: &PointInt2D) {
        let start = start.as_ref();
        let end = *end.as_ref();

        let old = mem::replace(&mut self.update_area, BBox::from_points(start, &end));

        for cell in self.cells.iter() {
            if !old.holds(cell) {
                let area = point![cell.x - 1, cell.y - 1]..=point![cell.x + 1, cell.y + 1];

                area.walk().unwrap().iter()
                    .for_each(|pt| self.updates.insert(pt));
            };
        }
    }

    #[wasm_bindgen(getter)]
    pub fn style(&self) -> UniverseStyle {
        self.style.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_style(&mut self, style: UniverseStyle) {
        self.style = style
    }
}

impl Universe {
    /// Register cells to update
    fn register_with_neighbors(&mut self, point: &Point2<i32>) {
        if !self.update_area.holds(point) {
            return;
        }
        let area = point![point.x - 1, point.y - 1]..=point![point.x + 1, point.y + 1];

        area.walk().unwrap().iter()
            .for_each(|pt| self.updates.insert(pt));
    }

    /// Set cell at given point alive
    fn set_alive(&mut self, point: Point2<i32>) {
        self.cells.insert(point);
        self.register_with_neighbors(&point);
    }

    /// Set cell at given point dead
    fn set_dead(&mut self, point: Point2<i32>) {
        self.register_with_neighbors(&point);
        self.cells.remove(&point);
    }

    /// Get cell state and neighbor count
    #[cfg(feature = "binary-tree")]
    fn cell_state(&self, point: &Point2<i32>) -> (bool, usize) {
        let area = point![point.x - 1, point.y - 1]..=point![point.x + 1, point.y + 1];
        let mut neighbors = 0;
        let mut is_alive = false;

        for pt in self.cells.query(area) {
            if pt == point {
                is_alive = true;
            } else {
                neighbors += 1;
            }
        }

        (is_alive, neighbors)
    }

    /// Get cell state and neighbor count
    #[cfg(feature = "quadtree")]
    fn cell_state(&self, point: &Point2<i32>) -> (bool, usize) {
        let area = point![point.x - 1, point.y - 1]..point![point.x + 2, point.y + 2];
        let mut neighbors = 0;
        let mut is_alive = false;

        for pt in self.cells.query(area) {
            if pt == point {
                is_alive = true;
            } else {
                neighbors += 1;
            }
        }

        (is_alive, neighbors)
    }
}
