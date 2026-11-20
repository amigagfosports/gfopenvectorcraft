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
const EDITOR_MARKUP: &core::ffi::CStr = c"<column id=app><row id=toolbar><button id=new>New</button><button id=open>Open</button><button id=save>Save</button><div class=rule></div><button id=undo>Undo</button><button id=redo>Redo</button><div class=spacer></div><select id=zoom><option>50%</option><option selected>100%</option><option>200%</option></select></row><row id=workspace><column id=tools><button id=select class=active>Select</button><button id=pen>Pen</button><button id=rectangle>Rectangle</button><button id=ellipse>Ellipse</button><button id=text>Text</button></column><column id=center><row id=documentbar><div class=tab>Untitled *</div><div class=spacer></div><button id=preview>Preview</button></row><div id=viewport><div id=artboard><div class=shape></div><div class=shape2></div><div class=label>VectorCraft</div></div></div></column><column id=inspector><div class=heading>Layers</div><div class=layer><span class=eye>●</span> Title</div><div class='layer selected'><span class=eye>●</span> Rectangle</div><div class=layer><span class=eye>●</span> Ellipse</div><div class=heading>Properties</div><row><label>X</label><input value='72'></row><row><label>Y</label><input value='72'></row><row><label>W</label><input value='468'></row><row><label>H</label><input value='648'></row><div class=heading>Appearance</div><row><label>Fill</label><input value='#5577cc'></row><row><label>Stroke</label><input value='#203050'></row></column></row><row id=status><div>Artboard 1 · 612 × 792 pt</div><div class=spacer></div><div>2 shapes · 100%</div></row></column>";

#[cfg(target_arch = "m68k")]
const EDITOR_CSS: &core::ffi::CStr = c"*{box-sizing:border-box}#app{width:100%;height:100%;background:#c8cbd0;color:#20242a}#toolbar{height:42px;padding:5px;gap:5px;background:#e4e6e9;border-bottom:1px solid #727780}#toolbar button{min-width:58px;height:30px}.spacer{flex:1}.rule{width:1px;height:28px;background:#9298a2;margin:0 3px}#workspace{flex:1;min-height:0}#tools{width:92px;padding:7px;gap:6px;background:#d7dade;border-right:1px solid #777d86}#tools button{height:34px;text-align:left}#tools .active{background:#496da8;color:#ffffff}#center{flex:1;min-width:0;background:#777d86}#documentbar{height:34px;padding:4px 7px;background:#d8dbe0;border-bottom:1px solid #666c74}.tab{padding:5px 16px;background:#f4f5f6;border:1px solid #858b94;border-bottom:0}#viewport{flex:1;padding:26px;background:#6f757d;align-items:center;justify-content:center;overflow:auto}#artboard{width:420px;height:520px;background:#ffffff;border:1px solid #34383e;box-shadow:3px 4px 8px #3e4248;position:relative}.shape{position:absolute;left:64px;top:78px;width:286px;height:224px;background:#5577cc;border:3px solid #203050}.shape2{position:absolute;left:155px;top:245px;width:170px;height:130px;border-radius:65px;background:#f2b84b;border:3px solid #7a5415}.label{position:absolute;left:102px;top:410px;font-size:28px;font-weight:bold;color:#263650}#inspector{width:218px;padding:8px;gap:5px;background:#eceef0;border-left:1px solid #686e77}.heading{font-weight:bold;padding:7px 4px 4px;border-bottom:1px solid #8d939b}.layer{padding:6px;background:#ffffff;border:1px solid #b1b6bd}.layer.selected{background:#6f8fbe;color:#ffffff}.eye{margin-right:7px}#inspector row{height:30px;gap:5px;align-items:center}#inspector label{width:48px}#inspector input{flex:1;min-width:0}#status{height:28px;padding:5px 9px;background:#e2e4e7;border-top:1px solid #777d86}";

#[cfg(target_arch = "m68k")]
#[allow(unsafe_code, reason = "the fixed AmigaDOS entry symbol is the port's isolated ABI boundary")]
#[unsafe(no_mangle)]
pub extern "C" fn gf_amiga_main(_args: *const u8, _length: i32) -> i32 {
    amiga_runtime::run(|runtime| {
        runtime.put_str(c"VectorCraft: native entry\n");
        runtime.put_str(c"VectorCraft: opening editor window\n");
        let Ok(mut window) = amiga_gfopenlayout::Window::open(runtime, c"VectorCraft — Untitled", EDITOR_MARKUP, EDITOR_CSS, 900, 620) else {
            runtime.put_str(c"VectorCraft: could not open GFOpenLayout editor window\n");
            return 25;
        };
        runtime.put_str(c"VectorCraft: editor window open\n");
        window.run(runtime);
        runtime.put_str(c"VectorCraft: editor window closed\n");
        0
    })
}

#[cfg(target_arch = "m68k")]
#[allow(unsafe_code, reason = "compiler-builtins requires the C abort symbol at the executable ABI boundary")]
#[unsafe(no_mangle)]
pub extern "C" fn abort() -> ! {
    loop {
        core::hint::spin_loop();
    }
}

#[cfg(target_arch = "m68k")]
#[allow(unsafe_code, reason = "compiler-generated memory calls use the C ABI at this isolated boundary")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn memcpy(destination: *mut u8, source: *const u8, length: usize) -> *mut u8 {
    for offset in 0..length {
        unsafe { destination.add(offset).write_volatile(source.add(offset).read_volatile()) };
    }
    destination
}

#[cfg(target_arch = "m68k")]
#[allow(unsafe_code, reason = "compiler-generated memory calls use the C ABI at this isolated boundary")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn memset(destination: *mut u8, value: i32, length: usize) -> *mut u8 {
    for offset in 0..length {
        unsafe { destination.add(offset).write_volatile(value as u8) };
    }
    destination
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
