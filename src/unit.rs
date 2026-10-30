use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnitType {
    Meters,
    Centimeters,
    Millimeters,
    Feet,
    Inches
}

pub struct Unit {
    pub raw: f64,
    pub unit_type: UnitType
}

impl Unit {
    pub const fn new(raw: f64, unit_type: UnitType) -> Self {
        Unit { raw, unit_type }
    }

    pub fn to_meters(&self) -> f64 {
        match self.unit_type {
            UnitType::Meters => self.raw,
            UnitType::Centimeters => self.raw / 100.0,
            UnitType::Millimeters => self.raw / 1000.0,
            UnitType::Feet => self.raw * 0.3048,
            UnitType::Inches => self.raw * 0.0254,
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Speed {
    pub raw: f64,
}

impl Speed {
    pub const fn new(raw: f64) -> Self {
        Speed { raw }
    }

    pub fn to_meters_per_second(&self) -> f64 {
        self.raw
    }
}