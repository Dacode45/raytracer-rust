mod math;
use math::vec::Vec3;

fn main() {
    let nx = 200;
    let ny = 100;
    println!("P3\n{} {}\n255", nx, ny);
    for y in (0..ny).rev() {
        for x in 0..nx {
            let i = x as f32;
            let j = y as f32;
            let col = Vec3::new(i / nx as f32, j / ny as f32, 0.2);
            let ir = (255.99 * col.r()) as isize;
            let ig = (255.99 * col.g()) as isize;
            let ib = (255.99 * col.b()) as isize;

            println!("{} {} {}", ir, ig, ib)
        }
    }
}
