mod outermost {
    pub fn middle_function() {}

    fn middle_secret_function() {}

    mod inside {
        pub fn inner_function() {}

        fn secret_function() {}
    }
}

fn try_me() {
    // wiil work even when the module is not defined as pub
    // try_me function is allowed to access the outermost module since
    // outermost is in the current (root) module, as is try_me
    outermost::middle_function();
    //outermost::middle_secret_function(); function is private
    //outermost::inside::inner_function(); module is private
    //outermost::inside::secret_function(); module is private
}
