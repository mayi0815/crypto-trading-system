use crate::{chat::Product, phase::TradingPhase, OrderType, Phase, Position, Tif};
use chrono::DateTime;
use chrono_tz::Tz;
use pyo3::prelude::*;

#[pyclass]
pub struct Subscription {
    product: Product,
    position: Option<Position>,
    phase: TradingPhase,
}

impl Subscription {
    pub fn new(product: Product) -> Self {
        Self {
            product,
            position: None,
            phase: TradingPhase::default(),
        }
    }

    pub fn on_position(&mut self, position: Position) {
        self.position = Some(position);
    }
}

#[pymethods]
impl Subscription {
    #[getter]
    pub fn symbol(&self) -> &String {
        self.product.symbol()
    }

    #[getter]
    pub fn delivery(&self) -> DateTime<Tz> {
        self.product.delivery()
    }

    #[getter]
    pub fn onboard(&self) -> DateTime<Tz> {
        self.product.onboard()
    }

    #[getter]
    pub fn max_prc(&self) -> f64 {
        self.product.max_prc()
    }

    #[getter]
    pub fn min_prc(&self) -> f64 {
        self.product.min_prc()
    }

    #[getter]
    pub fn tick_size(&self) -> f64 {
        self.product.tick_size()
    }

    #[getter]
    pub fn lot(&self) -> f64 {
        self.product.lot()
    }

    #[getter]
    pub fn min_notional(&self) -> f64 {
        self.product.min_notional()
    }

    #[getter]
    fn net(&self) -> f64 {
        match &self.position {
            Some(position) => position.net,
            None => 0.0,
        }
    }

    pub fn order_support(&self, order_type: &OrderType) -> bool {
        self.product.order_support(order_type)
    }

    pub fn tif_support(&self, tif: &Tif) -> bool {
        self.product.tif_support(tif)
    }

    fn floor_to_lot_size(&self, vol: f64) -> f64 {
        (vol / self.lot()).floor() * self.lot()
    }

    pub fn round_price(&self, price: f64) -> f64 {
        (price / self.tick_size()).floor() * self.tick_size()
    }

    fn tick_up(&self, price: f64, n: i32) -> f64 {
        self.round_price(price + (self.tick_size() * n as f64))
    }

    fn tick_dn(&self, price: f64, n: i32) -> f64 {
        self.round_price(price - (self.tick_size() * n as f64))
    }

    pub fn add_phase(&mut self, hour: u32, minute: u32, second: u32, phase: Phase) {
        self.phase.add_phase(hour, minute, second, phase)
    }

    pub fn determine(&self, mills: i64) -> Phase {
        self.phase.determine(mills)
    }
}
