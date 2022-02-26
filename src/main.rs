mod vox_writer;

fn main() {
    let mut vox = vox_writer::VoxWriter::create_empty();

    for i in 0..1000
    {
        for j in 0..1000
        {
            let cube_pos = f64::floor(f64::sin((i * i + j * j) as f64 / 50000.0) * 150.0) + 150.0;
            let cube_color = (i + j) % 255 + 1;
            vox.add_voxel(i, j, cube_pos as i32, cube_color);
        }
    }

    vox.save_to_file("output_voxwriter.vox".to_string());
}
