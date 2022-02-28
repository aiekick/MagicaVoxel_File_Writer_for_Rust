use std::time::Instant;
use crate::vox_writer::VoxWriter;
mod vox_writer;

#[allow(dead_code)]
fn mix(x:f64, y:f64, a:f64) -> f64 {
    x * (1.0 - a) + y * a
}

#[allow(dead_code)]
fn step(edge:f64, v:f64)->f64 { if v < edge { return 0.0; } return 1.0; }

#[allow(dead_code)]
fn sign(v:f64) -> f64 { if v < 0.0 { return -1.0; } return 1.0; }

/*
struct vec2
{
    x:f64,
    y:f64
}

// m 00 10
// m 10 11
struct mat2
{
    m:[f64, 4]
}

impl mat2
{
    fn create(v_a:f64, v_b:f64, v_c:f64, v_d:f64) -> Self {
        Self { m[0]=v_a, m[1]=v_b, m[2]=v_c, m[3]=v_d }
    }
    
    fn mult(&self, p:vec2) -> vec2> {
        vec2 { p.x * self.m[0] + p.x * self.m[1],
               p.y * self.m[2] + p.y * self.m[3] }
    }
}
*/

#[allow(dead_code)]
fn generate_julia_revolute()
{
    let now = Instant::now();
    {
        let mut vox = VoxWriter::create_empty();

        const SIZE:i32 = 300;
        const ZOOM_XZ:f64 = 5.0;
        const ZOOM_Y:f64 = 5.0;

        for i in 0..SIZE {
            for j in 0..SIZE {
                for k in 0..SIZE {
                    let px = (i as f64 * 2.0 / SIZE as f64 - 1.0) * ZOOM_XZ;
                    let py = (j as f64 * 2.0 / SIZE as f64 - 1.0) * ZOOM_Y;
                    let pz = (k as f64 * 2.0 / SIZE as f64 - 1.0) * ZOOM_XZ;

                    let an = f64::atan2(px, pz);

                    let cx = mix(0.2, -0.5, f64::sin(an * 2.0));
                    let cy = mix(0.5, 0.0, f64::sin(an * 3.0));

                    let path = f64::sqrt(px * px + pz * pz) - 3.0;

                    let mut rev_x = path;
                    let mut rev_y = py;

                    let _ca = f64::cos(an);
                    let _sa = f64::sin(an);

                    //let tmp_x = rev_x;
                    //rev_x = rev_x * _ca - rev_y - _sa;
                    //rev_y = tmp_x * _sa - rev_y + _sa;

                    let mut kk = 1.0;
                    let mut hh = 1.0;

                    for _idx in 0..5
                    {
                        hh *= 4.0 * kk;
                        kk = rev_x * rev_x + rev_y * rev_y;
                        if kk > 4.
                        {
                            break;
                        }
                       let tmp_x = rev_x;
                        rev_x = rev_x * rev_x - rev_y * rev_y + cx;
                        rev_y = 2.0 * tmp_x * rev_y + cy;
                    }

                    let df = f64::sqrt(kk / hh) * f64::log10(kk);

                    if f64::abs(df) - 0.01 < 0.0
                    {
                        let cube_color = (i + j + k) % 255 + 1;
                        //vox.add_voxel(i, j, k, cube_color);
                        vox.add_voxel(i, k, j, cube_color); // magicavoxel use the z as up axis
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
