use core::dick::Dick;
use core::Dk;

#[test]
fn dick_find_by_word() {
    let result: Option<&Dk> = Dick::find_by_word("程");
    if !result.is_none() {
        assert_eq!(result.unwrap().al, "cheng");
    }
}
