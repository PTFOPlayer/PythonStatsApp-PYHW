use git2::Repository;
use std::process::Command;
fn main() {
    let mut rm = Command::new("rm");
    rm.arg("-rf").arg("./msr_server");
    _ = rm.output();

    let url = "https://github.com/PTFOPlayer/msr_server";
    _ = match Repository::clone(url, "./msr_server") {
        Ok(repo) => repo,
        Err(e) => panic!("failed to clone: {}", e),
    };

    //cc::Build::new()
    //    .cpp(true)
    //    .file("./msr_server/src/main.cpp")
    //    .out_dir("./msr_server")
    //    .compile("msr_gen");
}