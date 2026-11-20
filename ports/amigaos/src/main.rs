// Copyright (c) 2026 AmiCradle Team. MIT OR Apache-2.0.
#![cfg_attr(target_arch = "m68k", no_main)]
#![cfg_attr(target_arch = "m68k", no_std)]
#![deny(clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::unimplemented, clippy::todo, clippy::unreachable)]

extern crate alloc;

#[cfg(any(target_arch = "m68k", test))]
pub mod model;
#[cfg(any(target_arch = "m68k", test))]
mod platform;

#[cfg(target_arch = "m68k")]
use model::{Document, Point};

#[cfg(target_arch = "m68k")]
#[allow(unsafe_code, reason = "the fixed AmigaDOS entry symbol is the port's isolated ABI boundary")]
#[unsafe(no_mangle)]
pub extern "C" fn gf_amiga_main(_args: *const u8, _length: i32) -> i32 {
    amiga_runtime::run(|runtime| {
        for line in platform::STARTUP_LINES {
            runtime.put_str(line);
        }
        let Ok(mut document) = Document::new("Untitled", "Artboard 1", 612.0, 792.0) else {
            return 20;
        };
        let Some(artboard_id) = document.artboards().first().map(|artboard| artboard.id) else {
            return 20;
        };
        let rectangle = [Point { x: 72.0, y: 72.0 }, Point { x: 540.0, y: 72.0 }, Point { x: 540.0, y: 720.0 }, Point { x: 72.0, y: 720.0 }];
        if document.add_path(artboard_id, &rectangle, true).is_err() {
            return 20;
        }
        runtime.put_str(c"portable model: one artboard and rectangle ready\n");
        if platform::BINDINGS.len() == 4 && document.paths().len() == 1 { 0 } else { 20 }
    })
}

#[cfg(target_arch = "m68k")]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {
        core::hint::spin_loop();
    }
}

#[cfg(not(target_arch = "m68k"))]
fn main() {}
