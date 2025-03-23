use std::io::Write;

const FEATURES: [&str; 7] = [
    "dev",
    "beta",
    "debug",
    "stable",
    "nightly",
    "release",
    "bootstrap",
];

/// The app function is used to configure the cfg setup for relaxed work with the correct compiler. It configures the following seven cfg attributes (dev, beta, debug, stable, nightly, releYe bootstrap), which can be used to adapt the code to the corresponding compiler (e.g., you provide a function under cfg(nightly) that uses optimizations that it otherwise could not use). The bootstrap cfg is used to obfuscate any nightly functions to ensure stability.
pub fn app() -> Result<(), Box<dyn std::error::Error>> {
    let mut stdout = std::io::stdout();
    for f in FEATURES {
        write!(stdout, "cargo::rustc-check-cfg=cfg({f})\n")?;
    }
    match rustc_version::version_meta()?.channel {
        rustc_version::Channel::Dev => {
            write!(stdout, "cargo::rustc-cfg=dev\n")?;
            write!(stdout, "cargo::rustc-cfg=beta\n")?;
            write!(stdout, "cargo::rustc-cfg=stable\n")?;
            write!(stdout, "cargo::rustc-cfg=nightly\n")?;
        },
        rustc_version::Channel::Beta => {
            write!(stdout, "cargo::rustc-cfg=beta\n")?;
            write!(stdout, "cargo::rustc-cfg=stable\n")?;
        },
        rustc_version::Channel::Stable => {
            write!(stdout, "cargo::rustc-cfg=stable\n")?;
        },
        rustc_version::Channel::Nightly => {
            write!(stdout, "cargo::rustc-cfg=beta\n")?;
            write!(stdout, "cargo::rustc-cfg=stable\n")?;
            write!(stdout, "cargo::rustc-cfg=nightly\n")?;
        },
    }
    if let Ok(env) = std::env::var("RUSTC_BOOTSTRAP") {
        if env.trim() == "1" {
            write!(stdout, "cargo::rustc-cfg=bootstrap\n")?;
        }
    }
    if cfg!(debug_assertions) {
        write!(stdout, "cargo::rustc-cfg=debug\n")?;
    } else {
        write!(stdout, "cargo::rustc-cfg=release\n")?;
    }
    Ok(stdout.flush()?)
}

/// the same as `self::app`, but returns a bool over the state of running.
pub fn checked_app() -> bool {
    match app() {
        Ok(_) => true,
        Err(_) => false,
    }
}

/// the same as `self::app`, but panics if noz successfully.
pub fn wrapping_app(expect_msg: Option<&str>) {
    if let Some(expect_msg) = expect_msg {
        app().expect(expect_msg)
    } else {
        app().unwrap()
    }
}
