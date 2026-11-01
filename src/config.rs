use crate::unit;


// TODO: tune
pub const LOOKAHEAD : unit::Unit = unit::Unit::new(
    1.0, 
    unit::UnitType::Meters
);

pub const WHEELBASE : unit::Unit = unit::Unit::new(
    0.660, 
    unit::UnitType::Meters
);

pub const DEFAULT_SPEED : unit::Unit = unit::Unit::new(
    1.0,
    unit::UnitType::Meters
); // in m/s

pub const TRAJECTORY_KEY : &str = "cev/planning/ref_trajectory";
pub const ACKERMANN_KEY : &str = "cev/commands/control";
pub const POSE_KEY : &str = "cev/localization/pose";
pub const SPEED_KEY : &str = "cev/config/speed";

pub const MAX_STEERING_ANGLE: f64 = 45.0_f64.to_radians();

pub const MAX_CAR_SPEED : f64 = 2.0; // in m/s