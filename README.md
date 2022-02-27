# MagicaVoxel File Writer for Rust

my conversion to Rust of my original c++ [MagicaVoxel File Writer](https://github.com/aiekick/MagicaVoxel_File_Writer)

vox.hm is the file format descriptor for HexaMonkey :
- original topic about it : https://github.com/ephtracy/voxel-model/issues/19
- HexaMonkey tool : http://hexamonkey.com/

## App

the main.cpp file show you how to generate a quick file :

With this simple code (thanks to [@unphased](https://github.com/unphased))

```rust
mod vox_writer;

fn main() {
    let mut vox = vox_writer::VoxWriter::create_empty();

    for i in 0..1000 {
        for j in 0..1000 {
            let cube_pos =
                f64::floor(f64::sin((i * i + j * j) as f64 / 50000.0 * 0.25) * 150.0) + 150.0;
            let cube_color = (i + j) % 255 + 1;
            vox.add_voxel(i, j, cube_pos as i32, cube_color);
        }
    }

    vox.save_to_file("output_voxwriter.vox".to_string());
}
```

you can generate that (previewed in [Magicavoxel](https://ephtracy.github.io/)

![main](main.jpg)
