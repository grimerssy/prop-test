#[macro_export]
macro_rules! prop_test {
    ($strategy:expr, $test:expr, $config:expr) => {
        let mut config = $config;
        config.source_file = Some(file!());
        let mut runner = ::proptest::test_runner::TestRunner::new(config);
        match runner.run($strategy, $test) {
            Ok(()) => (),
            Err(e) => panic!("{e}\n{runner}"),
        }
    };
    ($strategy:expr, $test:expr) => {
        $crate::prop_test!($strategy, $test, ::proptest::test_runner::Config::default())
    };
}