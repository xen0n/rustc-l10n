extern crate rustc_l10n;
extern crate serde_json;

use std::env;
use std::ffi::OsStr;
use std::process;


fn main() {
    let argv: Vec<_> = env::args_os().collect();
    let output = invoke_rustc(&argv);
    println!("output = {:?}", output);

    process::exit(output.exit_code);
}


#[derive(Debug)]
struct RustcOutput {
    exit_code: i32,
    errors: Vec<rustc_l10n::spec::Diagnostic>,
}


fn invoke_rustc<S: AsRef<OsStr>>(argv: &[S]) -> RustcOutput {
    // TODO: overridable rustc executable
    let mut cmd = process::Command::new("rustc");
    cmd.arg("--error-format=json");
    // skip original argv[0] and shove the rest in
    for arg in argv.iter().skip(1) {
        cmd.arg(arg);
    }

    let output = cmd.output().expect("failed to execute rustc");
    let stderr = String::from_utf8(output.stderr)
        .expect("failed to parse rustc stderr as utf-8");
    let lines = stderr
        .split('\n')
        .filter(|s| !s.is_empty())
        .map(|s| serde_json::from_str(s))
        .filter(|r| r.is_ok())
        .map(|r| r.unwrap())
        .collect();

    RustcOutput {
        exit_code: derive_exit_code(&output.status),
        errors: lines,
    }
}


fn derive_exit_code(status: &process::ExitStatus) -> i32 {
    if let Some(code) = status.code() {
        code
    } else {
        // On Unix; process was terminated by a signal.
        // The exitcode should be 128 + <signal number> in this case, by
        // convention (of the shells).
        if cfg!(unix) {
            use std::os::unix::process::ExitStatusExt;
            128 + status.signal().unwrap()
        } else {
            unimplemented!();
        }
    }
}
