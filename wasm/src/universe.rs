use std::cmp::{max, min};
use std::mem;
use js_sys::Math;
use na::{point, Point2, vector, Vector2};
use py::{BBox, Walkable};
use py::wasm::Vector2D;
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
    updates: BinaryTree,
    size: Vector2<usize>,
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
    pub fn dead(width: usize, height: usize) -> Universe {
        let size = vector![width, height];
        let bbox = BBox::from_anchor_size(&Point2::origin(), &size.cast());

        Universe {
            cells: Quadtree::inside(&bbox),
            updates: BinaryTree::new(),
            size,
            style: UniverseStyle::default(),
        }
    }

    /// Builds a fixed universe
    pub fn fixed(width: usize, height: usize) -> Universe {
        let mut universe = Universe::dead(width, height);

        for row in 0..universe.size.y as i32 {
            for col in 0..universe.size.x as i32 {
                let i = row * (universe.size.x as i32) + col;

                if i % 2 == 0 || i % 7 == 0 || i % 13 == 0 {
                    universe.set_alive(point![col, row])
                }
            }
        }

        universe
    }

    /// Builds a random universe
    pub fn random(width: usize, height: usize) -> Universe {
        let mut universe = Universe::dead(width, height);

        for row in 0..universe.size.y as i32 {
            for col in 0..universe.size.x as i32 {
                let rand = Math::random();

                if rand < 0.5 {
                    universe.set_alive(point![col, row])
                }
            }
        }

        universe
    }

    /// Compute next state
    pub fn tick(&mut self) {
        let old = Universe {
            cells: self.cells.clone(),
            updates: mem::replace(&mut self.updates, BinaryTree::new()),
            size: self.size,
            style: self.style.clone(),
        };

        for &cell in old.updates.iter() {
            let (is_alive, neighbors) = old.cell_state(&cell);

            if is_alive {
                if !(2..=3).contains(&neighbors) {
                    self.set_dead(cell);
                }
            } else if neighbors == 3 {
                self.set_alive(cell);
            }
        }
    }

    /// Render universe inside canvas
    pub fn render(&self, ctx: &CanvasRenderingContext2d) {
        let size = self.size.cast() * self.style.cell_size();

        ctx.begin_path();

        ctx.set_fill_style(self.style.dead_color());
        ctx.fill_rect(0.0, 0.0, size.x, size.y);

        ctx.set_fill_style(self.style.alive_color());

        for cell in self.cells.iter() {
            let pos = cell.cast() * self.style.cell_size();

            ctx.fill_rect(
                pos[0], pos[1],
                self.style.cell_size(), self.style.cell_size()
            );
        }

        ctx.stroke();
    }

    #[wasm_bindgen(getter)]
    pub fn size(&self) -> Vector2D {
        Vector2D::from(self.size.cast())
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
        let area = point![max(point.x - 1, 0), max(point.y - 1, 0)]..=point![min(point.x + 1, self.size.x as i32 - 1), min(point.y + 1, self.size.y as i32 - 1)];

        area.walk().unwrap().iter()
            .filter(|pt| pt != point)
            .for_each(|pt| self.updates.insert(pt));
    }

    /// Set cell at given point alive
    fn set_alive(&mut self, point: Point2<i32>) {
        self.register(&point);
        self.cells.insert(point);
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
