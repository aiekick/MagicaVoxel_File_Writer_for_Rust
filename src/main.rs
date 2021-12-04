///////////////////////////////////////////////////////////////////////////////////////////////////

pub fn get_mvid(a : char, b : char, c : char, d : char) -> u32
{
    return ((a as i32) | ((b as i32) << 8) | ((c as i32) << 16) | ((d as i32) << 24)) as u32;
}

#[test]
fn test_get_mvid()
{
    assert_eq!(get_mvid('V', 'O',  'X', ' '), 542658390);
}

///////////////////////////////////////////////////////////////////////////////////////////////////

fn main()
{
    let  id = get_mvid('V', 'O',  'X', ' ');

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
