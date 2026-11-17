//! SigmaOS-engrampa — thin CLI entry point.
//!
//! The library lives in `src/lib.rs`; this binary only wires argv/stdout.

use sigmaos_engrampa_lib::Component;

fn main() {
    let mut component = Component::new();
    if let Err(e) = component.init().and_then(|_| component.run()) {
        eprintln!("SigmaOS-engrampa: {e}");
        std::process::exit(1);
    }
    println!("SigmaOS-engrampa: ok");
}
