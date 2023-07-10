use std::cmp::{max, min};
use std::mem;
use js_sys::Math;
use na::{distance, point, Point2, vector};
use py::{BBox, Walkable};
use wasm_bindgen::prelude::*;
use web_sys::{CanvasRenderingContext2d, console};
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
    updates: BinaryTree,
    style: UniverseStyle,
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
    pub fn dead(width: i32, height: i32) -> Universe {
        let bbox = BBox::from_anchor_size(&Point2::origin(), &vector![width, height]);

        Universe {
            cells: Quadtree::new(),
            updates: BinaryTree::new(),
            style: UniverseStyle::default(),
        }
    }

    /// Builds a fixed universe
    pub fn fixed(width: i32, height: i32) -> Universe {
        let mut universe = Universe::dead(width, height);

        for row in 0..height {
            for col in 0..width {
                let i = row * width + col;

                if i % 2 == 0 || i % 7 == 0 || i % 13 == 0 {
                    universe.set_alive(point![col, row])
                }
            }
        }

        universe
    }

    /// Builds a random universe
    pub fn random(width: i32, height: i32) -> Universe {
        let mut universe = Universe::dead(width, height);

        for row in 0..height {
            for col in 0..width {
                let rand = Math::random();

                if rand < 0.5 {
                    universe.set_alive(point![col, row])
                }
            }
        }

        universe
    }

    /// Inserts some cells around given position
    pub fn insert_around(&mut self, ctx: &CanvasRenderingContext2d, cx: i32, cy: i32, r: i32) {
        let center = point![cx, cy];
        let area = point![cx - r, cy - r]..=point![cx + r, cy + r];

        area.walk().unwrap().iter()
            .filter(|cell| distance::<f32, 2>(&center.cast(), &cell.cast()) <= r as f32)
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
            updates: mem::replace(&mut self.updates, BinaryTree::new()),
            style: self.style.clone(),
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
    fn register(&mut self, point: &Point2<i32>) {
        let area = point![point.x - 1, point.y - 1]..=point![point.x + 1, point.y + 1];

        area.walk().unwrap().iter()
            .filter(|pt| pt != point)
            .for_each(|pt| self.updates.insert(pt));
    }

    /// Set cell at given point alive
    fn set_alive(&mut self, point: Point2<i32>) {
        self.cells.insert(point);
        self.register(&point);
    }

    /// Set cell at given point dead
    fn set_dead(&mut self, point: Point2<i32>) {
        self.register(&point);
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
