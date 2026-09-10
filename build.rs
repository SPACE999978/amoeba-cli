use std::process::Command;

fn main() {
    for git_path in ["HEAD", "index", "packed-refs"] {
        emit_git_rerun_path(git_path);
    }
    if let Some(symbolic_head) = git_stdout(&["symbolic-ref", "-q", "HEAD"]) {
        emit_git_rerun_path(&symbolic_head);
    }
    println!("cargo:rerun-if-changed=assets/Petri.ico");

    if std::env::var("CARGO_CFG_TARGET_OS").ok().as_deref() == Some("windows") {
        let mut resource = winresource::WindowsResource::new();
        resource.set_icon("assets/Petri.ico");
        resource
            .compile()
            .expect("failed to embed the Petri Windows icon");
    }

    let commit = git_stdout(&["rev-parse", "HEAD"])
        .filter(|value| !value.is_empty())
        .unwrap_or_else(|| "unknown".to_string());

    println!("cargo:rustc-env=PETRI_BUILD_COMMIT={commit}");
}

fn emit_git_rerun_path(path: &str) {
    if let Some(path) = git_stdout(&["rev-parse", "--path-format=absolute", "--git-path", path]) {
        println!("cargo:rerun-if-changed={path}");
    }
}

fn git_stdout(args: &[&str]) -> Option<String> {
    Command::new("git")
        .args(args)
        .output()
        .ok()
        .filter(|output| output.status.success())
        .and_then(|output| String::from_utf8(output.stdout).ok())
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty())
}
