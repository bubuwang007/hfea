use gui::run;
use geo::point::Point3d;

fn main() {
    let point = Point3d::new(1.0, 2.0, 3.0);
    println!("Point: x={}, y={}, z={}", point.x, point.y, point.z);
    run().unwrap();
}
