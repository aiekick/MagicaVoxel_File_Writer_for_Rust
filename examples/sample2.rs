use std::time::Instant;

fn main() {
    let now = Instant::now();
    let mut vox = vox_writer::VoxWriter::create_empty();
    const SIZE: i32 = 1000;
    const ZOOM_XZ: f64 = 5.0;
    const ZOOM_Y: f64 = 5.0;
    const ITERATIONS: i32 = 5;
    let mut kk;
    let mut hh;
    fn mix(x: f64, y: f64, a: f64) -> f64 {
        x * (1.0 - a) + y * a
    }
    for i in 0..SIZE {
        let px = (i as f64 * 2.0 / SIZE as f64 - 1.0) * ZOOM_XZ;
        for k in 0..SIZE {
            let pz = (k as f64 * 2.0 / SIZE as f64 - 1.0) * ZOOM_XZ;
            let an = f64::atan2(px, pz);
            let cx = mix(0.2, -0.5, f64::sin(an * 2.0));
            let cy = mix(0.5, 0.0, f64::sin(an * 3.0));
            let path = f64::sqrt(px * px + pz * pz) - 3.0;
            for j in 0..SIZE {
                let mut rev_y = (j as f64 * 2.0 / SIZE as f64 - 1.0) * ZOOM_Y;
                let mut rev_x = path;
                kk = 1.0;
                hh = 1.0;
                for _idx in 0..ITERATIONS {
                    let rev_x_squared = rev_x * rev_x;
                    let rev_y_squared = rev_y * rev_y;
                    hh *= 4.0 * kk;
                    kk = rev_x_squared + rev_y_squared;
                    if kk > 4.0 {
                        break;
                    }
                    rev_y = 2.0 * rev_x * rev_y + cy;
                    rev_x = rev_x_squared - rev_y_squared + cx;
                }
                let df = f64::sqrt(kk / hh) * f64::log10(kk);
                if f64::abs(df) - 0.01 < 0.0 {
                    let cube_color = ((f64::sin(rev_x + rev_y) * 0.5 + 0.5) * 6.0) as i32 + 249;
                    vox.add_voxel(i, k, j, cube_color); // magicavoxel use the z as up axis
                }
            }
        }
    }
    vox.save_to_file("julia_revolute_voxwriter.vox".to_string())
        .expect("Fail to save vox file");
    vox.print_stats();
    println!("Elapsed time : {} secs", now.elapsed().as_secs_f32());
}
