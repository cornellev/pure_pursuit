
mod config;
mod unit;
mod loc;
mod pure_pursuit;
mod ackermann;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    println!("jaiden's shitty pure pursuit impl");
    println!("lookahead: {:?}", config::LOOKAHEAD.to_meters());
    println!("wheelbase: {:?}", config::WHEELBASE.to_meters());

    println!("opening zenoh session...");

    let mut trajectory: Option<Vec<loc::Pose>> = None;
    let mut speed: f64 = config::DEFAULT_SPEED.to_meters();


    let session = zenoh::open(zenoh::Config::default()).await.unwrap();

    let traj_sub = session.declare_subscriber(config::TRAJECTORY_KEY).await?;
    let pose_sub = session.declare_subscriber(config::POSE_KEY).await?;
    let speed_sub = session.declare_subscriber(config::SPEED_KEY).await?;

    loop {
        tokio::select! {
            result = speed_sub.recv_async() => {
                let Ok(sample) = result else {
                    eprintln!("speed subscription error: {:?}", result);
                    break;
                };

                let payload = sample.payload().to_bytes();

                match serde_json::from_slice::<unit::Speed>(&payload) {
                    Ok(new_speed) => {
                        speed = unit::Unit::new(new_speed.to_meters_per_second(), unit::UnitType::Meters).to_meters();
                        println!("updated speed to: {:?}", speed);
                    }
                    Err(e) => {
                        eprintln!("failed to deserialize speed: {:?}", e);
                    }
                }
            }

            result = traj_sub.recv_async() => {
                let Ok(sample) = result else {
                    eprintln!("trajectory subscription error: {:?}", result);
                    break;
                };

                let payload = sample.payload().to_bytes();
                
                match serde_json::from_slice::<Vec<loc::Pose>>(&payload) {
                    Ok(traj) if !traj.is_empty() => {
                        trajectory = Some(traj);
                    }
                    Ok(_) => {
                        eprintln!("received empty trajectory");
                    }
                    Err(e) => {
                        eprintln!("failed to deserialize trajectory: {:?}", e);
                    }
                }
            }

            result = pose_sub.recv_async() => {
                let Ok(sample) = result else {
                    eprintln!("pose subscription error: {:?}", result);
                    break;
                };

                // only if trajectory exists lol
                let Some(ref traj) = trajectory else {
                    continue;
                };

                let payload = sample.payload().to_bytes();

                let pose: loc::Pose = match serde_json::from_slice(&payload) {
                    Ok(pose) => pose,
                    Err(e) => {
                        eprintln!("failed to deserialize pose: {:?}", e);
                        continue;
                    }
                };

                let mut steering_angle = pure_pursuit::pure_pursuit(pose, traj.clone());
                
                steering_angle = steering_angle.clamp(-config::MAX_STEERING_ANGLE, config::MAX_STEERING_ANGLE);
                steering_angle = steering_angle / config::MAX_STEERING_ANGLE; // normalize to [-1, 1]

                speed = speed.clamp(0.0, config::MAX_CAR_SPEED) / config::MAX_CAR_SPEED; // clamp speed to [0, 1]

                let payload = serde_json::to_vec(&ackermann::AckermannCommand {
                    steering_angle,
                    speed: speed
                }).unwrap();
                
                session.put(config::ACKERMANN_KEY, payload).await.unwrap();
            }
        }


    }

    Ok(())
}
