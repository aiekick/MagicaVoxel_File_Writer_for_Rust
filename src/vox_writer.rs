pub fn get_mvid(a : char, b : char, c : char, d : char) -> u32
{
   return ((a as i32) | ((b as i32) << 8) | ((c as i32) << 16) | ((d as i32) << 24)) as u32;
}

#[test]
fn test_get_mvid()
{
   assert_eq!(get_mvid('V', 'O',  'X', ' '), 542658390);
}

