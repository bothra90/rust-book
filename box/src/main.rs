fn foo() -> Box<i32> {
    Box::new(5)
}

fn bar() -> &'static i32 {
    static X: i32 = 5;
    &X
}
/*
 * fn baz() -> &'static i32 {
 *     &5
 *     }
 *     */

fn main() {
    let a = foo();
    println!("{}", *a);
    println!("{}", *bar());
    // println!("{}", X);
}
