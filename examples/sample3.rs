use std::time::Instant;

fn main() {
    let now = Instant::now();
    let mut vox = vox_writer::VoxWriter::create_empty();
    for i in 0..300 {
        for j in 0..300 {
            for k in 0..300 {
                vox.add_voxel(i, j, k, 100);
            }
        }
    }
    vox.save_to_file("output_cube_voxwriter.vox".to_string())
        .expect("Fail to save vox file");
    vox.print_stats();
    println!("Elapsed time : {} secs", now.elapsed().as_secs_f32());
}
