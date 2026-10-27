
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


    let session = zenoh::open(zenoh::Config::default()).await.unwrap();

    let traj_sub = session.declare_subscriber(config::TRAJECTORY_KEY).await?;
    let pose_sub = session.declare_subscriber(config::POSE_KEY).await?;

    loop {
        tokio::select! {
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

                let steering_angle = pure_pursuit::pure_pursuit(pose, traj.clone());

                let payload = serde_json::to_vec(&ackermann::AckermannCommand {
                    steering_angle,
                    speed: 0.0
                }).unwrap();
                
                session.put(config::ACKERMANN_KEY, payload).await.unwrap();
            }
        }


    }

    Ok(())
}
