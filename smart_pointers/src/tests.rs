#[test]
fn test() {
    let _x = TestStruct { elem: Rc::new(5u32) };
    // x = TestStruct { elem: Rc::new(6u32) };
    // x.elem = Rc::new(6);
}

use std::rc::Rc;

struct TestStruct {
    elem: Rc<u32>,
}
