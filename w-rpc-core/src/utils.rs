pub fn set_panic_hook() {
    // When the `console_error_panic_hook` feature is enabled, we can call the
    // `set_panic_hook` function at least once during initialization, and then
    // we will get better error messages if our code ever panics.
    //
    // For more details see
    // https://github.com/rustwasm/console_error_panic_hook#readme
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();

    // To get rid of the barrowing and lifetime problems that making this a
    // function causes.
    #[macro_export]
    macro_rules! get_module {
        ($name:expr) => {
            use std::ops::DerefMut;
            let list = &mut GLOBAL_MODULE_LIST.lock().unwrap();
            let list = DerefMut::deref_mut(list);
            let mut loop_num: u32 = 0;

            for item in list{
                if item.get_module_name() == $name {
                    break;
                }
                loop_num += 1
            }

            println!("{}", loop_num);
            let current_module = &GLOBAL_MODULE_LIST.lock().unwrap()[loop_num as usize];
        };
    }
}
