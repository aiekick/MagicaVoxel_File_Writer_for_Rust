use std::time::Instant;
use crate::vox_writer::VoxWriter;
mod vox_writer;

#[allow(dead_code)]
fn mix(x:f64, y:f64, a:f64) -> f64 {
    x * (1.0 - a) + y * a
}

#[allow(dead_code)]
fn step(x:f64, y:f64)->f64 { if y < x { return 0.0; } return 1.0; }

#[allow(dead_code)]
fn sign(v:f64) -> f64 { if v < 0.0 { return -1.0; } return 1.0; }

#[allow(dead_code)]
fn atan2(x:f64, y:f64)->f64 { if x != 0.0 {return y / x; } return 1.0; }

#[allow(dead_code)]
fn generate_julia_revolute()
{
    let now = Instant::now();
    {
        let mut vox = VoxWriter::create_empty();

        const SX:i32 = 300;
        const SY:i32 = 300;
        const SZ:i32 = 300;
        const ZOOM:f64 = 5.0;

        for i in 0..SX {
            for j in 0..SY {
                for k in 0..SZ {
                    let px = (i as f64 * 2.0 / SX as f64 - 1.0) * ZOOM;
                    let py = (j as f64 * 2.0 / SY as f64 - 1.0) * ZOOM;
                    let pz = (k as f64 * 2.0 / SZ as f64 - 1.0) * ZOOM;

                    let an = atan2(px, py);

                    let cx = mix(0.2, -0.5, f64::sin(an * 2.0));
                    let cy = mix(0.5, 0.0, f64::sin(an * 3.0));

                    let path = f64::sqrt(px * px + py * py) - 3.0;

                    let mut rev_x = path;
                    let mut rev_z = pz;

                    //let _ca = an.cos();
                    //let _sa = an.sin();
                    //rev *= mat2(cos(a), -sin(a), sin(a), cos(a));

                    let mut kk = 1.0;
                    let mut hh = 1.0;

                    for _idx in 0..5
                    {
                        hh *= 4.0 * kk;
                        kk = rev_x * rev_x + rev_z * rev_z;
                        if kk > 4.
                        {
                            break;
                        }
                        let tmp_x = rev_x;
                        rev_x = rev_x * rev_x - rev_z * rev_z + cx;
                        rev_z = 2.0 * tmp_x * rev_z + cy;
                    }

                    let df = f64::sqrt(kk / hh) * f64::log10(kk);

                    if f64::abs(df) - 0.01 < 0.0
                    {
                        let cube_color = (i + j + k) % 255 + 1;
                        vox.add_voxel(i, j, k, cube_color);
                    }
                }
            }
        }
        vox.save_to_file("julia_revolute_voxwriter.vox".to_string())
            .expect("Fail to save vow file");
        vox.print_stats();
    }
    let elapsed = now.elapsed();
    println!("generate_julia_revolute Elapsed Time : {:.2?}", elapsed);
}

#[allow(dead_code)]
fn generate_default()
{
    let now = Instant::now();
    {
        let mut vox = VoxWriter::create_empty();
        for i in 0..1000 {
            for j in 0..1000 {
                let cube_pos =
                    f64::floor(f64::sin((i * i + j * j) as f64 / 50000.0 * 0.25) * 150.0) + 150.0;
                let cube_color = (i + j) % 255 + 1;
                vox.add_voxel(i, j, cube_pos as i32, cube_color);
            }
        }
        vox.save_to_file("default_voxwriter.vox".to_string())
            .expect("Fail to save vow file");
        vox.print_stats();
    }
    let elapsed = now.elapsed();
    println!("generate_default Elapsed Time : {:.2?}", elapsed);
}

fn main() {
    //generate_default();
    generate_julia_revolute();
}
