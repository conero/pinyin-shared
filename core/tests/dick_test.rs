use core::dick::Dick;
use core::Dk;

#[test]
fn dick_find_by_word() {
    let result: Option<Dk> = Dick::find_by_word("程");
    if !result.is_none(){
        assert_ne!(result.unwrap().al, "cheng");
    }

}