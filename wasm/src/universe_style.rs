use wasm_bindgen::prelude::*;

/// Universe style
#[derive(Clone)]
#[wasm_bindgen]
pub struct UniverseStyle {
    cell_size: f64,
    alive_color: JsValue,
    dead_color: JsValue,
}

#[wasm_bindgen]
impl UniverseStyle {
    /// Styles of the dark universe
    pub fn light() -> UniverseStyle {
        UniverseStyle {
            cell_size: 5.0,
            alive_color: JsValue::from_str("#000000"),
            dead_color: JsValue::from_str("#ffffff"),
        }
    }

    /// Styles of the dark universe
    pub fn dark() -> UniverseStyle {
        UniverseStyle {
            cell_size: 5.0,
            alive_color: JsValue::from_str("#ffffff"),
            dead_color: JsValue::from_str("#000000"),
        }
    }

    #[wasm_bindgen(getter = cell_size)]
    pub fn js_cell_size(&self) -> f64 {
        self.cell_size
    }

    #[wasm_bindgen(getter = alive_color)]
    pub fn js_alive_color(&self) -> JsValue {
        self.alive_color.clone()
    }

    #[wasm_bindgen(getter = dead_color)]
    pub fn js_dead_color(&self) -> JsValue {
        self.dead_color.clone()
    }
}

impl UniverseStyle {
    pub fn cell_size(&self) -> f64 {
        self.cell_size
    }

    pub fn alive_color(&self) -> &JsValue {
        &self.alive_color
    }

    pub fn dead_color(&self) -> &JsValue {
        &self.dead_color
    }
}

impl Default for UniverseStyle {
    fn default() -> Self {
        UniverseStyle::light()
    }
}
