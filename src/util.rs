// --- PLAYGROUND SETUP --------------------------------------------------------
/// Define the following function as a runnable test.
/// 
/// Note: this is not a standard macro in Rust. It's a macro defined for this
///       playground (see util.rs).
#[macro_export] macro_rules! runnable {
    ($name: ident, $exp: expr) => (
        #[test] fn $name(){ 
            let test_name = stringify!($name);
            println!("{} [start]", test_name);
            let start_time = std::time::Instant::now();
            $exp 
            let end_time = std::time::Instant::now();
            println!(
                "{} [end]: took {} ms...", 
                test_name,
                end_time.duration_since(start_time).as_millis()
            );
        }
    );
}
// -----------------------------------------------------------------------------