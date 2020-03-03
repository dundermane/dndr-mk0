use quaternion::{Quaternion, id, rotate_vector};
use nalgebra::Vector3;
mod botfactory;

fn main() {
    println!("Hello, world!");
    println!("{:?}", sss(2.0,3.0,4.0));
    let bot = botfactory::load_from_file("src/robots/bot1.ron");
    //println!("{:?}", bot.links);

}

///
/// #sss Function
/// This calculates the three angles of a triangle based on its side lengths using
/// the "law of cosines".
///
/// input: ( a , b , c ) 
/// output: ( a_th, b_th, c_th )
///
/// where "th_a" is the side length opposite of the angle "a"
/// "th_b" is the side length opposite of the angle "b"
/// "th_c" is the side length opposite of the angle "c"
///

fn sss( a: f64, b: f64, c: f64 ) -> ( f64, f64, f64 ) {
    
    let th_a:f64 = ((b.powi(2) + c.powi(2) - a.powi(2))/(2.0*b*c)).acos();
    let th_b:f64 = ((c.powi(2) + a.powi(2) - b.powi(2))/(2.0*c*a)).acos();
    let th_c:f64 = ((a.powi(2) + b.powi(2) - c.powi(2))/(2.0*a*b)).acos();
    (th_a.to_degrees(), th_b.to_degrees(), th_c.to_degrees())

}



fn v_w(robot: &botfactory::Robot) {
    println!("{:?}", robot.links[0].len);
}

fn R7DOF_radius_of_elbow(r_u: f64, r_f: f64, r_w: f64) -> f64 {

    let mut inside: f64 = r_u.powi(2) - r_f.powi(2) + r_w.powi(2);
    inside = inside / ( 2.0 * r_w.abs().powi(2) );
    return inside.sqrt();

}

fn R7DOF_ru_mean_point(r_u: f64, r_f: f64, r_w: f64) -> f64 {
    
    let mut inside: f64 = r_u.powi(2) - r_f.powi(2) + r_w.powi(2);
    inside = inside / ( 2.0 * r_w.abs().powi(2) );
    inside = inside * r_w;
    return inside;

}

fn R7DOF_wrist_from_EEF(shoulder_from_base: Vector3<f64>, hand: botfactory::Link<f64>, eef: botfactory::Pose<f64>) -> Vector3<f64> {


    let shoulder_to_eef = eef.vect - shoulder_from_base;
    let eef_to_wrist = rotate_vector(eef.quat, hand.next);

    return Vector3::new(1.0, 1.0, 1.0);
}
