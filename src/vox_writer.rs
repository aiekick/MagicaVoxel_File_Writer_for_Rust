use std::collections::HashMap;
use std::hash::Hash;
use std::fs::File;
use std::io::{Seek, SeekFrom, Write};
use std::{mem};
use std::borrow::BorrowMut;

///////////////////////////////////////////////////////////////////////////////////////////////////

struct Point3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T> Point3<T> {
    fn create3(x: T, y: T, z: T) -> Self {
        Self { x, y, z }
    }
}

impl<T: Clone> Point3<T> {
    fn create1(v: T) -> Self {
        Self {
            x: v.clone(),
            y: v.clone(),
            z: v,
        }
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////////

struct AABBCC
{
    // the lower left vertex
    lower_bound: Point3<f64>,
    // the upper right vertex
    upper_bound: Point3<f64>,
}

#[allow(dead_code)]
impl AABBCC
{
    fn create(lower_bound:f64, upper_bound:f64) -> Self
    {
        let mut lb = Point3::<f64>::create1(0.0);
        let mut ub = Point3::<f64>::create1(0.0);

        lb.x = f64::min(lower_bound, upper_bound);
        lb.y = f64::min(lower_bound, upper_bound);
        lb.z = f64::min(lower_bound, upper_bound);
        ub.x = f64::max(lower_bound, upper_bound);
        ub.y = f64::max(lower_bound, upper_bound);
        ub.z = f64::max(lower_bound, upper_bound);

        Self {
            lower_bound:lb,
            upper_bound:ub,
        }
    }

    fn size(&self) -> Point3<f64>
    {
        Point3::<f64>::create3(
            self.upper_bound.x - self.lower_bound.x,
            self.upper_bound.y - self.lower_bound.y,
            self.upper_bound.z - self.lower_bound.z,
        )
    }

    fn combine(&mut self, v_pt:Point3<f64>)
    {
        self.lower_bound.x = f64::min(self.lower_bound.x, v_pt.x);
        self.lower_bound.y = f64::min(self.lower_bound.y, v_pt.y);
        self.lower_bound.z = f64::min(self.lower_bound.z, v_pt.z);
        self.upper_bound.x = f64::max(self.upper_bound.x, v_pt.x);
        self.upper_bound.y = f64::max(self.upper_bound.y, v_pt.y);
        self.upper_bound.z = f64::max(self.upper_bound.z, v_pt.z);
    }
}
///////////////////////////////////////////////////////////////////////////////////////////////////

#[allow(dead_code)]
pub fn get_id_char(a: char, b: char, c: char, d: char) -> u32 {
    return ((a as i32) | ((b as i32) << 8) | ((c as i32) << 16) | ((d as i32) << 24)) as u32;
}
#[test]
fn test_get_id_char() {
    assert_eq!(get_id_char('V', 'O', 'X', ' '), 542658390);
}

#[allow(dead_code)]
pub fn get_id_u8(a: u8, b: u8, c: u8, d: u8) -> u32 {
    return ((a as i32) | ((b as i32) << 8) | ((c as i32) << 16) | ((d as i32) << 24)) as u32;
}

#[test]
fn test_get_id_u8() {
    assert_eq!(get_id_u8(86, 79, 88, 32), 542658390);
}

///////////////////////////////////////////////////////////////////////////////////////////////////

struct DICTstring
{
    buffer_size:i32,
    buffer:String
}

#[allow(dead_code)]
impl DICTstring
{
    fn create(v_buffer_size:i32, v_buffer:String) -> Self
    {
        Self {
            buffer_size:v_buffer_size,
            buffer:v_buffer
        }
    }

    fn create_from_string(v_buffer:String) -> Self
    {
        Self::create(v_buffer.len() as i32, v_buffer)
    }

    fn create_empty() -> Self
    {
        Self::create(0, "".to_string())
    }

    fn write(&mut self, mut v_fp: &File) -> std::io::Result<()>
    {
        self.buffer_size = self.buffer.len() as i32;
        v_fp.write_all(&self.buffer_size.to_le_bytes())?;
        v_fp.write_all(&self.buffer.as_bytes())?;
        Ok(())
    }

    fn get_size(&mut self) -> usize
    {
        self.buffer_size = self.buffer.len() as i32;
        return mem::size_of::<i32>() + mem::size_of::<char>() * (self.buffer_size as usize);
    }
}

#[test]
fn test_dictstring_get_size() {
    let mut stru = DICTstring::create_empty();
    assert_eq!(stru.get_size(), 5);
}

///////////////////////////////////////////////////////////////////////////////////////////////////

struct DICTitem
{
    key:DICTstring,
    value:DICTstring,
}

#[allow(dead_code)]
impl DICTitem
{
    fn create_empty() -> Self
    {
        Self {
            key: DICTstring::create_empty(),
            value: DICTstring::create_empty()
        }
    }

    fn create_from_key_value(key:String, value:String) -> Self
    {
        Self {
            key: DICTstring::create_from_string(key),
            value: DICTstring::create_from_string(value)
        }
    }

    fn write(&mut self, fp: &File) -> std::io::Result<()>
    {
        self.key.write(&fp)?;
        self.value.write(&fp)?;
        Ok(())
    }

    fn get_size(&mut self) -> usize
    {
        return self.key.get_size() + self.value.get_size();
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////////

struct DICT
{
    count: i32,
    keys: Vec<DICTitem>
}

#[allow(dead_code)]
impl DICT
{
    fn create_empty() -> Self
    {
        Self {
            count: 0,
            keys: vec![]
        }
    }

    fn write(&mut self, mut fp: &File)
    {
        self.count = self.keys.len() as i32;
        fp.write_all(&self.count.to_le_bytes());
        for i in 0..self.count
        {
            self.keys[i as usize].write(&fp);
        }
    }

    fn get_size(&mut self) -> usize
    {
        self.count = self.keys.len() as i32;
        let mut s = mem::size_of::<i32>();
        for i in 0..self.count
        {
            s += self.keys[i as usize].get_size();
        }
        return s;
    }

    fn add(&mut self, key:String, value:String)
    {
        self.keys.push(DICTitem::create_from_key_value(key, value));
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////////

struct Ntrn
{
    node_id:i32,
    node_attribs:DICT,
    child_node_id:i32,
    reserved_id:i32,
    layer_id:i32,
    num_frames:i32,
    frames:Vec<DICT>
}

#[allow(dead_code)]
impl Ntrn
{
    fn create(count_frames:i32) -> Self
    {
        let mut _frames:Vec<DICT> = vec![];
        while _frames.len() < count_frames as usize
        {
            _frames.push(DICT::create_empty());
        }

        Self {
            node_id: 0,
            node_attribs: DICT::create_empty(),
            child_node_id: 0,
            reserved_id: -1,
            layer_id: -1,
            num_frames: count_frames,
            frames: _frames
        }
    }

    fn write(&mut self, mut fp: &File)
    {
        // chunk header
        let id = get_id_char('n', 'T', 'R', 'N');
        fp.write_all(&id.to_le_bytes());
        let content_size = self.get_size();
        fp.write_all(&content_size.to_le_bytes());
        let child_size:i32 = 0;
        fp.write_all(&child_size.to_le_bytes());

        // datas's
        fp.write_all(&self.node_id.to_le_bytes());
        self.node_attribs.write(&fp);
        fp.write_all(&self.child_node_id.to_le_bytes());
        fp.write_all(&self.reserved_id.to_le_bytes());
        fp.write_all(&self.layer_id.to_le_bytes());
        fp.write_all(&self.num_frames.to_le_bytes());
        for i in 0..self.num_frames
        {
            self.frames[i as usize].write(&fp);
        }
    }

    fn get_size(&mut self) -> usize
    {
        let mut s = mem::size_of::<i32>() * 5 + self.node_attribs.get_size();
        for i in 0..self.num_frames
        {
            s += self.frames[i as usize].get_size();
        }
        return s;
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////////
struct Ngrp
{
    node_id:i32,
    node_attribs:DICT,
    node_children_nodes:i32,
    child_nodes:Vec<i32>
}

#[allow(dead_code)]
impl Ngrp
{
    fn create(count:i32) -> Self
    {
        let mut nodes:Vec<i32> = vec![];
        while nodes.len() < count as usize
        {
            nodes.push(0);
        }

        Self {
            node_id:0,
            node_attribs: DICT::create_empty(),
            node_children_nodes:count,
            child_nodes:nodes,
        }
    }

    fn write(&mut self, mut fp: &File)
    {
        // chunk header
        let id = get_id_char('n', 'G', 'R', 'P');
        fp.write_all(&id.to_le_bytes());
        let content_size = self.get_size();
        fp.write_all(&content_size.to_le_bytes());
        let child_size:i32 = 0;
        fp.write_all(&child_size.to_le_bytes());

        // datas's
        fp.write_all(&self.node_id.to_le_bytes());
        self.node_attribs.write(&fp);
        fp.write_all(&self.node_children_nodes.to_le_bytes());

        let mut _childs_nodes:Vec<u8> = vec![];
        for child in &self.child_nodes {
            let bytes = child.to_le_bytes();
            for byte in bytes {
                _childs_nodes.push(byte);
            }
        }
        fp.write_all(&_childs_nodes);
    }

    fn get_size(&mut self) -> usize
    {
        return mem::size_of::<i32>() * (2 + self.node_children_nodes as usize) + self.node_attribs.get_size();
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////////

struct Model
{
    model_id:i32,
    model_attribs:DICT
}

#[allow(dead_code)]
impl Model
{
    fn create_empty() -> Self
    {
        Self {
            model_id: 0,
            model_attribs: DICT::create_empty()
        }
    }

    fn write(&mut self, mut fp: &File)
    {
        fp.write_all(&self.model_id.to_le_bytes());
        self.model_attribs.write(&fp);
    }

    fn get_size(&mut self) -> usize
    {
        return mem::size_of::<i32>() + self.model_attribs.get_size();
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////////

struct Nshp
{
    node_id:i32,
    node_attribs:DICT,
    num_models:i32,
    models:Vec<Model>
}

#[allow(dead_code)]
impl Nshp
{
    fn create(count:i32) -> Self
    {
        let mut _models:Vec<Model> = vec![];
        while _models.len() < count as usize
        {
            _models.push(Model::create_empty());
        }

        Self {
            node_id: 0,
            node_attribs: DICT::create_empty(),
            num_models: count,
            models: _models
        }
    }

    fn write(&mut self, mut fp: &File)
    {
        // chunk header
        let id = get_id_char('n', 'S', 'H', 'P');
        fp.write_all(&id.to_le_bytes());
        let content_size = self.get_size();
        fp.write_all(&content_size.to_le_bytes());
        let child_size:usize = 0;
        fp.write_all(&child_size.to_le_bytes());

        // datas's
        fp.write_all(&self.node_id.to_le_bytes());
        self.node_attribs.write(&fp);
        fp.write_all(&self.num_models.to_le_bytes());
        for i in 0..self.num_models
        {
            self.models[i as usize].write(&fp);
        }
    }

    fn get_size(&mut self) -> usize
    {
        let mut s = mem::size_of::<i32>() * 2 + self.node_attribs.get_size();
        for i in 0..self.num_models
        {
            s += self.models[i as usize].get_size();
        }
        return s;
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////////

struct LAYR
{
    node_id:i32,
    node_attribs:DICT,
    reserved_id:i32
}

#[allow(dead_code)]
impl LAYR
{
    fn create_empty() -> Self
    {
        Self {
            node_id: 0,
            node_attribs: DICT::create_empty(),
            reserved_id: 0
        }
    }

    fn write(&mut self, mut fp:File)
    {
        // chunk header
        let id = get_id_char('L', 'A', 'Y', 'R');
        fp.write_all(&id.to_le_bytes());
        let content_size = self.get_size();
        fp.write_all(&content_size.to_le_bytes());
        let child_size:usize = 0;
        fp.write_all(&child_size.to_le_bytes());

        // datas's
        fp.write_all(&self.node_id.to_le_bytes());
        self.node_attribs.write(&fp);
        fp.write_all(&self.reserved_id.to_le_bytes());
    }

    fn get_size(&mut self) -> usize
    {
        return mem::size_of::<i32>() * 2 + self.node_attribs.get_size();
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////////

struct Size
{
    size_x:i32,
    size_y:i32,
    size_z:i32,
}

#[allow(dead_code)]
impl Size
{
    fn create_empty() -> Self
    {
        Self {
            size_x: 0,
            size_y: 0,
            size_z: 0
        }
    }

    fn write(&self, mut fp: &File)
    {
        // chunk header
        let id = get_id_char('S', 'I', 'Z', 'E');
        fp.write_all(&id.to_le_bytes());
        let content_size = self.get_size();
        fp.write_all(&content_size.to_le_bytes());
        let child_size:usize = 0;
        fp.write_all(&child_size.to_le_bytes());

        // datas's
        fp.write_all(&self.size_x.to_le_bytes());
        fp.write_all(&self.size_y.to_le_bytes());
        fp.write_all(&self.size_z.to_le_bytes());
    }

    fn get_size(&self) -> usize
    {
        return mem::size_of::<i32>() * 3;
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////////

struct XYZI
{
    num_voxels:i32,
    voxels:Vec<u8>
}

#[allow(dead_code)]
impl XYZI
{
    fn create_empty() -> Self
    {
        Self {
            num_voxels: 0,
            voxels: vec![]
        }
    }

    fn write(&mut self, mut fp: &File)
    {
        // chunk header
        let id = get_id_char('X', 'Y', 'Z', 'I');
        fp.write_all(&id.to_le_bytes());
        let content_size = self.get_size();
        fp.write_all(&content_size.to_le_bytes());
        let child_size:usize = 0;
        fp.write_all(&child_size.to_le_bytes());

        // datas's
        fp.write_all(&self.num_voxels.to_le_bytes());
        fp.write_all(&self.voxels);
    }

    fn get_size(&mut self) -> usize
    {
        self.num_voxels = (self.voxels.len() / 4) as i32;
        return mem::size_of::<i32>() * (1 + self.num_voxels as usize);
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////////

struct RGBA
{
    colors:Vec<i32>
}

#[allow(dead_code)]
impl RGBA
{
    fn create_empty() -> Self
    {
        Self {
            colors: vec![0; 256]
        }
    }

    fn write(&self, mut fp: &File)
    {
        // chunk header
        let id = get_id_char('R', 'G', 'B', 'A');
        fp.write_all(&id.to_le_bytes());
        let content_size = self.get_size();
        fp.write_all(&content_size.to_le_bytes());
        let child_size:usize = 0;
        fp.write_all(&child_size.to_le_bytes());

        // datas's
        let mut _colors:Vec<u8> = vec![];
        for color in &self.colors {
            let bytes = color.to_le_bytes();
            for byte in bytes {
                _colors.push(byte);
            }
        }
        fp.write_all(&_colors);
    }

    fn get_size(&self) -> usize
    {
        return mem::size_of::<u8>() * 4 * 256;
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////////

struct VoxCube
{
    cube_id:i32,

    // size
    size:Size,

    // translate
    tx:i32,
    ty:i32,
    tz:i32,

    xyzi:XYZI
}

#[allow(dead_code)]
impl VoxCube
{
    fn create_empty() -> Self
    {
        Self {
            cube_id: 0,
            size: Size::create_empty(),
            tx: 0,
            ty: 0,
            tz: 0,
            xyzi: XYZI::create_empty(),
        }
    }

    fn is_empty(&self) -> bool
    {
        self.cube_id == 0
    }

    fn write(&mut self, fp: &File)
    {
        self.size.write(&fp);
        self.xyzi.write(&fp);
    }

    fn add_coord(&mut self, v:u8)
    {
        self.xyzi.voxels.push(v);
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////////

trait Memory<A: Eq + Hash, B: Eq + Hash, C: Eq + Hash> {

    fn get(&self, a: &A, b: &B, c: &C) -> Option<&i32>;

    fn set(&mut self, a: A, b: B, c: C, v: i32);
}

pub struct Table<A: Eq + Hash, B: Eq + Hash, C: Eq + Hash> {
    table: HashMap<A, HashMap<B, HashMap<C, i32>>>,
}

impl<A: Eq + Hash, B: Eq + Hash, C: Eq + Hash> Table<A, B, C> {
    fn new() -> Table<A, B, C> {
        Table {
            table: HashMap::new(),
        }
    }
}

impl<A: Eq + Hash, B: Eq + Hash, C: Eq + Hash> Memory<A, B, C> for Table<A, B, C> {

    fn get(&self, a: &A, b: &B, c: &C) -> Option<&i32> {
        self.table.get(a)?.get(b)?.get(c)
    }

    fn set(&mut self, a: A, b: B, c: C, v: i32) {
        let inner_a = self.table.entry(a).or_insert(HashMap::new());
        let inner_b = inner_a.entry(b).or_insert(HashMap::new());
        inner_b.insert(c, v);
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////////

pub(crate) struct VoxWriter
{
    mv_version:i32,
    id_vox:i32,
    //id_pack:i32,
    id_main:i32,
    //id_size:i32,
    //id_xyzi:i32,
    //id_rgba:i32,
    //id_ntrn:i32,
    //id_ngrp:i32,
    //id_nshp:i32,
    max_voxel_per_cube_x:i32,
    max_voxel_per_cube_y:i32,
    max_voxel_per_cube_z:i32,
    max_volume:AABBCC,
    colors:Vec<i32>,
    cubes:Vec<VoxCube>,
    max_voxel_id:i32,
    min_voxel_x:i32,
    min_voxel_y:i32,
    min_voxel_z:i32,
    vox_cube_id:Table<i32,i32,i32>,
    voxel_id:Table<i32,i32,i32>,   // voxel of a cube
    //last_error:i32,
}

#[allow(dead_code)]
impl VoxWriter
{
    //////////////////////////////////////////////////////////////////
    // the limit of magicavoxel is 127 for one voxel, is 127 voxels (indexs : 0 -> 126)
    // vMaxVoxelPervoxelX,Y,Z define the limit of one voxel
    // default values for limit xyz will be 126
    fn create(limitx:i32, limity:i32, limitz:i32) -> Self
    {
        // the limit of magicavoxel is 127 because the first is 1 not 0
        // so this is 0 to 126
        // index limit, size is 127

        Self {
            mv_version:150,
            id_vox: get_id_char('V', 'O', 'X', ' ') as i32,
            //id_pack:get_id_char('P', 'A', 'C', 'K') as i32,
            id_main:get_id_char('M', 'A', 'I', 'N') as i32,
            //id_size:get_id_char('S', 'I', 'Z', 'E') as i32,
            //id_xyzi:get_id_char('X', 'Y', 'Z', 'I') as i32,
            //id_rgba:get_id_char('R', 'G', 'B', 'A') as i32,
            //id_ntrn:get_id_char('n', 'T', 'R', 'N') as i32,
            //id_ngrp:get_id_char('n', 'G', 'R', 'P') as i32,
            //id_nshp:get_id_char('n', 'S', 'H', 'P') as i32,
            max_voxel_id:0,
            min_voxel_x:1e7 as i32,
            min_voxel_y:1e7 as i32,
            min_voxel_z:1e7 as i32,
            max_voxel_per_cube_x : i32::clamp(limitx, 0, 126),
            max_voxel_per_cube_y : i32::clamp(limity, 0, 126),
            max_voxel_per_cube_z : i32::clamp(limitz, 0, 126),
            max_volume:AABBCC::create(1e7, 0.0),
            vox_cube_id: Table::new(),
            voxel_id: Table::new(),
            colors: Default::default(),
            cubes: Default::default(),
            //last_error: 0
        }
    }

    pub fn create_empty() -> Self
    {
        Self::create(126,126,126)
    }

    pub fn clear_voxels(&mut self)
    {
        self.cubes.clear();
    }

    pub fn clear_colors(&mut self)
    {
        self.colors.clear();
    }

    pub fn add_color(&mut self, v_r:u8, v_g:u8, v_b:u8, v_a:u8, index:u8)
    {
        while self.colors.len() <= index as usize
        {
            self.colors.push(0);
        }

        self.colors[index as usize] = get_id_u8(v_r, v_g, v_b, v_a) as i32;
    }

    pub fn add_voxel(&mut self, v_x:i32, v_y:i32, v_z:i32, v_color_index:i32)
    {
        // voxel pos
        let ox = f64::floor (v_x as f64 / self.max_voxel_per_cube_x as f64) as i32;
        let oy = f64::floor(v_y as f64 / self.max_voxel_per_cube_y as f64) as i32;
        let oz = f64::floor(v_z as f64 / self.max_voxel_per_cube_z as f64) as i32;

        self.min_voxel_x = i32::min(self.min_voxel_x, ox);
        self.min_voxel_y = i32::min(self.min_voxel_y, oy);
        self.min_voxel_z = i32::min(self.min_voxel_z, oz);

        self.max_volume.combine(Point3::<f64>::create3(v_x as f64, v_y as f64, v_z as f64));

        self.merge_voxel_in_cube(v_x, v_y, v_z, v_color_index as u8, ox, oy, oz);
    }

    pub fn save_to_file(&mut self, file_path_name:String) -> std::io::Result<()>
    {
        let mut file = File::create(file_path_name)?;

        let zero:i32 = 0;

        file.write_all(&self.id_vox.to_le_bytes());
        file.write_all(&self.mv_version.to_le_bytes());

        // MAIN CHUNCK
        file.write_all(&self.id_main.to_le_bytes());
        file.write_all(&zero.to_le_bytes());

        let num_bytes_main_chunk_pos = file.seek(SeekFrom::Start(0))?;
        file.write_all(&zero.to_le_bytes());

        let header_size = file.seek(SeekFrom::Start(0))?; // seek(SeekFrom::Start(0)) => get file pos

        //  file.seek(SeekFrom::Start(pos))?; +> set file pos

        let count_cubes = self.cubes.len();

        let mut node_ids = 0;
        let mut root_transform = Ntrn::create(1);
        root_transform.node_id = node_ids;
        node_ids += 1;
        root_transform.child_node_id = node_ids;

        let mut root_group = Ngrp::create(count_cubes as i32);
        root_group.node_id = node_ids; //
        root_group.node_children_nodes = count_cubes as i32;

        let mut shapes:Vec<Nshp> = vec![];
        let mut shape_transforms:Vec<Ntrn> = vec![];
        for i in 0..count_cubes
        {
            let c = self.cubes[i].borrow_mut();

            c.write(&file);

            let mut trans = Ntrn::create(1);
            node_ids += 1;
            trans.node_id = node_ids;
            root_group.child_nodes[i] = node_ids;
            node_ids += 1;
            trans.child_node_id = node_ids;
            trans.layer_id = 0;

            c.tx = f64::floor((c.tx as f64 - self.min_voxel_x as f64 + 0.5) * self.max_voxel_per_cube_x as f64 - self.max_volume.lower_bound.x - self.max_volume.size().x * 0.5) as i32;
            c.ty = f64::floor((c.ty as f64 - self.min_voxel_y as f64 + 0.5) * self.max_voxel_per_cube_y as f64 - self.max_volume.lower_bound.y - self.max_volume.size().y * 0.5) as i32;
            c.tz = f64::floor((c.tz as f64 - self.min_voxel_z as f64 + 0.5) * self.max_voxel_per_cube_z as f64) as i32;

            // not an animation in my case so only first frame frames[0]
            trans.frames[0].add("_t".to_string(), format!("({} {} {})", c.tx, c.ty, c.tz));

            shape_transforms.push(trans);

            let mut shape = Nshp::create(1);
            shape.node_id = node_ids; //
            shape.models[0].model_id = i as i32;
            shapes.push(shape);
        }

        root_transform.write(&file);
        root_group.write(&file);

        // trn & shp
        for i in 0..count_cubes
        {
            shape_transforms[i].write(&file);
            shapes[i].write(&file);
        }

        // no layr in my cases

        // layr
        /*for (int i = 0; i < 8; i++)
        {
            LAYR layr;
            layr.node_id = i;
            layr.node_attribs.Add("_name", ct::toStr(i));
            layr.write(m_File);
        }*/

        // RGBA Palette
        if self.colors.len() > 0
        {
            let mut palette = RGBA::create_empty();
            for i in 0..255
            {
                if i < self.colors.len()
                {
                    palette.colors[i] =  self.colors[i];
                }
                else
                {
                    palette.colors[i] = 0;
                }
            }

            palette.write(&file);
        }

        let main_child_chunk_size = file.seek(SeekFrom::Start(0))? - header_size;
        file.seek(SeekFrom::Start(num_bytes_main_chunk_pos))?;
        let size = main_child_chunk_size as i32;
        file.write_all(&size.to_le_bytes());

        file.sync_all()?; // ensure than all ops are done

        Ok(())
    }

    fn get_cube_id(&mut self, v_x:i32, v_y:i32, v_z: i32) -> i32
    {
        let id = self.vox_cube_id.get(&v_x, &v_y, &v_z);

        match id {
            Some(_) => {return id.unwrap().clone();}
            None => {
                self.max_voxel_id += 1;
                self.vox_cube_id.set(v_x,v_y,v_z, self.max_voxel_id);
                return self.max_voxel_id - 1;
            }
        }
    }

    fn get_cube(&mut self, v_x:i32, v_y:i32, v_z:i32) -> Option<&mut VoxCube>
    {
        let cube_id = self.get_cube_id(v_x, v_y, v_z) as usize;

        if cube_id == self.cubes.len()
        {
            let mut _cube = VoxCube::create_empty();

            _cube.cube_id = cube_id as i32;

            _cube.tx = v_x;
            _cube.ty = v_y;
            _cube.tz = v_z;

            _cube.size.size_x = self.max_voxel_per_cube_x + 1;
            _cube.size.size_y = self.max_voxel_per_cube_y + 1;
            _cube.size.size_z = self.max_voxel_per_cube_z + 1;

            self.cubes.push(_cube);
        }

        if cube_id < self.cubes.len()
        {
            return self.cubes.get_mut(cube_id);
        }

        return None;
    }

    fn mod_value(&self, vx:i32, vy:i32, vz:i32) -> Point3<u8>
    {
        Point3::<u8>::create3(
            (vx % self.max_voxel_per_cube_x) as u8,
            (vy % self.max_voxel_per_cube_y) as u8,
            (vz % self.max_voxel_per_cube_z) as u8)
    }

    fn merge_voxel_in_cube(&mut self, v_x:i32, v_y:i32, v_z:i32, v_color_index:u8, c_x:i32, c_y:i32, c_z:i32)
    {
        let id = self.voxel_id.get(&v_x, &v_y, &v_z);
        if id.is_none()
        {
            let p = self.mod_value(v_x, v_y, v_z);

            let _cube = self.get_cube(c_x, c_y, c_z);
            if _cube.is_some()
            {
                 let mut cid = 0;
                _cube.map(|c| {
                    c.xyzi.voxels.push(p.x);
                    c.xyzi.voxels.push(p.y);
                    c.xyzi.voxels.push(p.z);
                    c.xyzi.voxels.push(v_color_index as u8); // color index
                    cid = c.xyzi.voxels.len() as i32;
                });

                self.voxel_id.set(v_x, v_y, v_z, cid);
            }
        }
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////////
