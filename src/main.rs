mod vox_writer;

fn main()
{
    let  id = vox_writer::get_mvid('V', 'O',  'X', ' ');

    println!("id is {}", id);

    /*
    vox::VoxWriter vox;
    for (int i = 0; i < 1000; ++i) {
        for (int j = 0; j < 1000; ++j) {
            vox.AddVoxel(i, j, (int)std::floor(sinf((float)(i * i + j * j) / 50000) * 150) + 150, (i + j) % 255 + 1);
        }
    }
    vox.SaveToFile("output_voxwriter.vox");
    */
}
