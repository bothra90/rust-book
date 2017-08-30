/* Privacy rules;
 *
 * Overall, these are the rules for item visibility:
 *
 *     1. If an item is public, it can be accessed through any of its parent modules.
 *     2. If an item is private, it may be accessed only by the current module and its child
 *         modules.
 *
 */

mod outermost {
    pub fn middle_function() {}

    fn middle_secret_function() {}

    pub mod inside {
        pub fn inner_function() {
            // ::outermost::middle_function();
            ::outermost::middle_secret_function();
        }

        pub fn secret_function() {}
    }
}

fn try_me() {
    outermost::middle_function();
    //outermost::middle_secret_function();
    outermost::inside::inner_function();
    outermost::inside::secret_function();
}
