// TODO: use generics to allow letters comparison
pub fn largest_number(a: i32, b: i32) -> i32 {
    match a {
        a if a > b => a,
        a if b > a => b,
        _ => a,
    }
}
