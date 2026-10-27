use crate::config::{LOOKAHEAD, WHEELBASE};
use crate::loc;
use crate::unit::Unit;



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

    let alpha = target_point.atan2() - pose.get_heading();

    let a = 2.0 * WHEELBASE.to_meters() * alpha.sin();
    let steering_angle = a.atan2(LOOKAHEAD.to_meters());

    return steering_angle;
}