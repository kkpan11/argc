#[macro_export]
macro_rules! snapshot {
    (
        $source:expr,
        $args:expr,
    ) => {
        let (stdout, stderr) = match argc::eval($source, $args).unwrap() {
            either::Either::Left(stdout) => (stdout, String::new()),
            either::Either::Right(stderr) => (String::new(), stderr.to_string()),
        };

        let args = $args.join(" ");
        let output = format!(
            r###"RUN
{}

STDOUT
{}

STDERR
{}
"###,
            args, stdout, stderr
        );
        insta::assert_snapshot!(output);
    };
}

#[macro_export]
macro_rules! plain {
    (
        $source:expr,
        $args:expr,
        $(stdout: $stdout:expr,)?
        $(stderr: $stderr:expr,)?
    ) => {
        let result = match argc::eval($source, $args).unwrap()  {
            either::Either::Left(stdout) => (stdout, String::new()),
            either::Either::Right(stderr) => (String::new(), stderr.to_string()),
        };
        $({
            assert_eq!(result.0.as_str(), $stdout);
        })?
        $({
            assert_eq!(result.1.as_str(), $stderr);
        })?
    };
}

#[macro_export]
macro_rules! fatal {
    (
        $source:expr,
        $args:expr,
        $err:expr
    ) => {
        let err = argc::eval($source, $args).unwrap_err();
        assert_eq!(err.to_string().as_str(), $err);
    };
}

#[macro_export]
macro_rules! snapshot_compgen {
    (
        $line:expr
    ) => {
        let (script_file, script_content) = $crate::fixtures::get_spec();
        let (stdout, stderr) =
            match argc::compgen(argc::Shell::Fish, &script_file, &script_content, $line) {
                Ok(stdout) => (stdout, String::new()),
                Err(stderr) => (String::new(), stderr.to_string()),
            };

        let output = format!(
            r###"RUN
{}

STDOUT
{}

STDERR
{}
"###,
            $line, stdout, stderr
        );
        insta::assert_snapshot!(output);
    };
}

#[macro_export]
macro_rules! snapshot_export {
    ($source:expr, $name:literal) => {
        let json = argc::export($source, $name).unwrap();
        let output = serde_json::to_string_pretty(&json).unwrap();
        insta::assert_snapshot!(output);
    };
}
