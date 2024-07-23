fn main() {
    let args = std::env::args();

    let mut target_args = Vec::new();

    if let Ok(rust_target) = &std::env::var("CARGO_BUILD_TARGET") {
        target_args.push("-target");
        target_args.push(map_target(rust_target))
    }

    let args = args
        .skip(1)
        .filter(|a| !a.contains("-latomic"))
        .chain(target_args.into_iter().map(|s| s.to_owned()));

    let zig_command = cargo_zigbuild::Zig::Cc {
        args: args.collect(),
    };

    zig_command.execute().unwrap()
}

fn map_target(rust_target: &str) -> &'static str {
    match rust_target {
        "x86_64-unknown-linux-musl" => "x86_64-linux-musl",
        "aarch64-unknown-linux-musl" => "aarch64-linux-musl",
        "arm-unknown-linux-musleabihf" => "arm-linux-musleabihf",
        "armv7-unknown-linux-musleabihf" => "arm-linux-musleabihf",
        _ => panic!("target unkown: ZIG_WRAPPER_TARGET set to {rust_target}")
    }
}
