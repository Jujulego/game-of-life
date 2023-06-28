use js_sys::Math;
use na::{point, Point2, vector, Vector2};
use py::traits::BBoxBounded;
use py::wasm::Vector2D;
use wasm_bindgen::prelude::*;
use web_sys::CanvasRenderingContext2d;
use crate::quadtree::Quadtree;
use crate::universe_style::UniverseStyle;

/// Life universe
#[derive(Clone)]
#[wasm_bindgen]
pub struct Universe {
    cells: Quadtree,
    size: Vector2<usize>,
    style: UniverseStyle,
}

#[wasm_bindgen]
impl Universe {
    /// Builds a dead universe
    pub fn dead(width: usize, height: usize) -> Universe {
        Universe {
            cells: Quadtree::new(),
            size: vector![width, height],
            style: UniverseStyle::default(),
        }
    }

    /// Builds a fixed universe
    pub fn fixed(width: usize, height: usize) -> Universe {
        let mut universe = Universe::dead(width, height);

        for row in 0..universe.size[1] as i32 {
            for col in 0..universe.size[0] as i32 {
                let i = row * (universe.size[0] as i32) + col;

                if i % 2 == 0 || i % 7 == 0 || i % 13 == 0 {
                    universe.set_alive(&point![col, row])
                }
            }
        }

        universe
    }

    /// Builds a random universe
    pub fn random(width: usize, height: usize) -> Universe {
        let mut universe = Universe::dead(width, height);

        for row in 0..universe.size[1] as i32 {
            for col in 0..universe.size[0] as i32 {
                let rand = Math::random();

                if rand < 0.5 {
                    universe.set_alive(&point![col, row])
                }
            }
        }

        universe
    }

    /// Compute next state
    pub fn tick(&mut self) {
        let old = self.clone();

        for row in 0..self.size[1] as i32 {
            for col in 0..self.size[0] as i32 {
                let point = point![col, row];

                let cell = old.is_alive(&point);
                let live_neighbors = old.alive_neighbor_count(&point);

                match (cell, live_neighbors) {
                    (true, x) if x < 2 => self.set_dead(&point),
                    (true, x) if x > 3 => self.set_dead(&point),
                    (false, 3) => self.set_alive(&point),
                    _ => ()
                };
            }
        }
    }

    /// Render universe inside canvas
    pub fn render(&self, ctx: &CanvasRenderingContext2d) {
        let size = self.size.cast() * self.style.cell_size();

        ctx.begin_path();

        ctx.set_fill_style(self.style.dead_color());
        ctx.fill_rect(0.0, 0.0, size[0], size[1]);

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
    /// Check if cell at given point is alive
    fn is_alive(&self, point: &Point2<i32>) -> bool {
        self.cells.has(point)
    }

    /// Set cell at given point alive
    fn set_alive(&mut self, point: &Point2<i32>) {
        self.cells.insert(point);
    }

    /// Set cell at given point dead
    fn set_dead(&mut self, point: &Point2<i32>) {
        self.cells.remove(point);
    }

    /// Count alive neighbors of given point
    fn alive_neighbor_count(&self, point: &Point2<i32>) -> usize {
        let area = (point![point.x - 1, point.y - 1]..=point![point.x + 1, point.y + 1]).bbox();

        self.cells.search(area)
            .iter()
            .filter(|&pt| pt != point)
            .count()
    }
}
