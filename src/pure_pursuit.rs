use crate::config::{LOOKAHEAD, WHEELBASE};
use crate::loc;
use std::f64::consts::PI;

fn wrap_angle(angle: f64) -> f64 {
    let wrapped = angle.rem_euclid(2.0 * PI);
    if wrapped > PI {
        wrapped - 2.0 * PI
    } else {
        wrapped
    }
}

pub fn pure_pursuit(pose: loc::Pose, path: Vec<loc::Pose>) -> f64 {
    // find the closest point on the path to the current pose
    let mut closest_point = path[0];
    let mut closest_distance = pose.distance_to(&closest_point);
    let mut target_idx = 0;
    for point in path.iter() {
        let distance = pose.distance_to(point);
        if distance < closest_distance {
            closest_distance = distance;
            closest_point = *point;
        }
    }

    target_idx = path.iter().position(|&p| p.equals(&closest_point)).unwrap();

    let mut current_lookahead = 0.;

    // just fine the point that's closest to the lookahead distance / slightly above it
    // also just choose the last if we're just at the end of the path
    while current_lookahead < LOOKAHEAD.to_meters() && target_idx < path.len() - 1 {
        let d = path[target_idx].distance_to(&path[target_idx + 1]);
        current_lookahead += d;
        target_idx += 1;
    }

    let target_point = path[target_idx];
    let loc = pose.get_location();
    let target = target_point.get_location();
    let dx = target.x() - loc.x();
    let dy = target.y() - loc.y();
    let alpha = wrap_angle(dy.atan2(dx) - pose.get_heading());

    let a = 2.0 * WHEELBASE.to_meters() * alpha.sin();
    a.atan2(LOOKAHEAD.to_meters())
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::loc::Pose;

    #[test]
    fn straight_ahead_is_zero_steer() {
        let pose = Pose::new(0.0, 0.0, 0.0);
        let path = vec![Pose::new(0.0, 0.0, 0.0), Pose::new(LOOKAHEAD.to_meters(), 0.0, 0.0)];
        let steer = pure_pursuit(pose, path);
        assert!(steer.abs() < 1e-9, "steer={steer}");
    }

    #[test]
    fn left_target_is_positive_steer() {
        // REP-103 +Y is left
        let pose = Pose::new(0.0, 0.0, 0.0);
        let path = vec![
            Pose::new(0.0, 0.0, 0.0),
            Pose::new(LOOKAHEAD.to_meters(), LOOKAHEAD.to_meters(), 0.0),
        ];
        let steer = pure_pursuit(pose, path);
        assert!(steer > 0.0, "steer={steer}");
    }
}
