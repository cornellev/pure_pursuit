use crate::unit;


// TODO: tune
pub const LOOKAHEAD : unit::Unit = unit::Unit::new(
    1.0, 
    unit::UnitType::Meters
);

pub const WHEELBASE : unit::Unit = unit::Unit::new(
    0.5, 
    unit::UnitType::Meters
);

pub const TRAJECTORY_KEY : &str = "cev/planning/ref_trajectory";
pub const ACKERMANN_KEY : &str = "cev/commands/control";
pub const POSE_KEY : &str = "cev/localization/pose";