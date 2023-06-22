use wasm_bindgen::prelude::wasm_bindgen;
use js_sys::Math;
use na::{point, Point2, vector, Vector2};
use py::wasm::Vector2D;
use wasm_bindgen::JsValue;
use web_sys::CanvasRenderingContext2d;
use crate::utils::{cmp_zorder, DIRECTIONS};

/// Life universe
#[derive(Clone)]
#[wasm_bindgen]
pub struct Universe {
    size: Vector2<usize>,
    cells: Vec<Point2<i32>>,
}

impl Universe {
    /// Returns index of point in cells vector (where it is, or where it should be)
    fn index_of(&self, point: &Point2<i32>) -> Result<usize, usize> {
        self.cells.binary_search_by(|pt| cmp_zorder(pt, point))
    }

    /// Check if cell at given point is alive
    fn is_alive(&self, point: &Point2<i32>) -> bool {
        self.index_of(point).is_ok()
    }

    /// Set cell at given point alive
    fn set_alive(&mut self, point: &Point2<i32>) {
        if let Err(idx) = self.index_of(point) {
            self.cells.insert(idx, *point);
        }
    }

    /// Set cell at given point dead
    fn set_dead(&mut self, point: &Point2<i32>) {
        if let Ok(idx) = self.index_of(point) {
            self.cells.remove(idx);
        }
    }

    /// Count alive neighbors of given point
    fn alive_neighbor_count(&self, point: &Point2<i32>) -> usize {
        DIRECTIONS
            .iter()
            .filter(move |&d| {
                let mut neighbor = point + d;
                neighbor[0] %= self.size[0] as i32;
                neighbor[1] %= self.size[1] as i32;

                self.is_alive(&neighbor)
            })
            .count()
    }
}

#[wasm_bindgen]
impl Universe {
    /// Builds a dead universe
    pub fn dead(width: usize, height: usize) -> Universe {
        Universe {
            size: vector![width, height],
            cells: Vec::new(),
        }
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
    pub fn render(&self, ctx: &CanvasRenderingContext2d, alive: &JsValue, dead: &JsValue) {
        ctx.begin_path();

        ctx.set_fill_style(dead);
        ctx.fill_rect(0.0, 0.0, (self.size[0] * 5) as f64, (self.size[1] * 5) as f64);

        ctx.set_fill_style(alive);

        for cell in self.cells.iter() {
            ctx.fill_rect((cell[0] * 5) as f64, (cell[1] * 5) as f64, 5.0, 5.0);
        }

        ctx.stroke();
    }

    pub fn size(&self) -> Vector2D {
        Vector2D::from(self.size.cast())
    }
}
