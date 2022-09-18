pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[no_mangle]
pub extern fn demo_name() {
    println!("shard demo from pinyin to dll!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
