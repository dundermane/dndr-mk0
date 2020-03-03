use std::fs::File;
use ron::de::from_reader;
use serde::{Serialize, Deserialize};
use quaternion::{Quaternion};
use nalgebra::Vector3;

#[derive(Serialize, Deserialize, Debug)]
pub struct Pose<T> {
  pub vect: Vector3<T>,
  pub quat: Quaternion<T>

}

#[derive(Serialize, Deserialize, Debug)]
pub struct Link<T> {
  pub len: T,
  pub next: Pose<T>

}

#[derive(Serialize, Deserialize, Debug)]
pub struct Robot {

  pub links: Vec<Link<f32>>,

}

pub fn load_from_file( filename:&str ) -> Robot {

  let file = File::open(&filename)
    .expect("Something went wrong reading the file");
  let robot: Robot = match from_reader(file) {
    Ok(x) => x,
    Err(e) => {
      println!("Failed to load robot: {}", e);
      std::process::exit(1);
    }
  };

  return(robot);
}

