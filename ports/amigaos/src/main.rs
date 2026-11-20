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
const EDITOR_MARKUP: &core::ffi::CStr = c"<column id=app><row id=toolbar><button id=new title='Start a blank document'>New</button><button id=open disabled title='File open is planned for RC2'>Open</button><button id=save disabled title='File save is planned for RC2'>Save</button><div class=rule></div><button id=undo>Undo</button><button id=redo>Redo</button><div class=spacer></div><label for=zoom>Zoom</label><select id=zoom disabled title='Interactive zoom is planned for RC2'><option selected>100%</option></select></row><row id=workspace><column id=tools><div class=heading>Tools</div><button id=select class=active>Select</button><button id=rectangle>Add Rect</button><button id=ellipse>Add Ellipse</button><button id=text>Add Label</button></column><column id=center><row id=documentbar><div class=tab>Untitled *</div><div class=spacer></div><button id=preview>Preview</button></row><div id=viewport><div id=artboard><div class=shape>Canvas preview</div><div class=shape2>Vector paths</div><div class=shape3>Shape palette</div><div class=shape4>Stroke sample</div><div class=label>Made on Amiga</div></div></div></column><column id=inspector><div class=heading>Layers</div><div class='layer selected'><span class=eye>*</span> Active object</div><div class=layer><span class=eye>*</span> Rectangle</div><div class=layer><span class=eye>*</span> Ellipse</div><div class=heading>Properties</div><row><label>X</label><input value='72'></row><row><label>Y</label><input value='72'></row><row><label>W</label><input value='468'></row><row><label>H</label><input value='648'></row><div class=heading>Appearance</div><row><label>Fill</label><input value='#5577cc'></row><row><label>Stroke</label><input value='#203050'></row></column></row><row id=status><div>Use the tools - the latest action appears in the title bar</div><div class=spacer></div><div>Native 68k</div></row></column>";

#[cfg(target_arch = "m68k")]
const EDITOR_CSS: &core::ffi::CStr = c"*{box-sizing:border-box}#app{width:100%;height:100%;background:#c8cbd0;color:#20242a}#toolbar{height:44px;padding:6px;gap:5px;background:#e4e6e9;border-bottom:1px solid #727780;align-items:center}#toolbar button{min-width:58px;height:30px}.spacer{flex:1}.rule{width:1px;height:28px;background:#9298a2;margin:0 3px}#workspace{flex:1;min-height:0}#tools{width:108px;padding:8px;gap:7px;background:#d7dade;border-right:1px solid #777d86}#tools button{height:36px;text-align:left}#tools .active{background:#496da8;color:#ffffff}#center{flex:1;min-width:0;background:#777d86}#documentbar{height:36px;padding:4px 7px;background:#d8dbe0;border-bottom:1px solid #666c74;align-items:center}.tab{padding:6px 16px;background:#f4f5f6;border:1px solid #858b94;border-bottom:0}#viewport{flex:1;padding:20px;background:#6f757d;align-items:center;justify-content:center;overflow:auto}#artboard{width:420px;height:500px;background:#ffffff;border:1px solid #34383e;box-shadow:3px 4px 8px #3e4248;position:relative}.shape,.shape2,.shape3,.shape4{position:absolute;padding:8px;text-align:center;color:#203050;background:#eef3ff;border:3px solid #5577cc}.shape{left:64px;top:62px;width:286px;height:150px}.shape2{left:155px;top:190px;width:170px;height:106px;border-radius:53px;background:#fff3d3;border-color:#c48724}.shape3{left:45px;top:310px;width:150px;height:85px;background:#e7f5ec;border-color:#3c8c61}.shape4{left:245px;top:330px;width:120px;height:75px;border-radius:38px;background:#fbe7eb;border-color:#b54b5a}.label{position:absolute;left:105px;top:430px;font-size:25px;font-weight:bold;color:#263650}#inspector{width:218px;padding:8px;gap:5px;background:#eceef0;border-left:1px solid #686e77}.heading{font-weight:bold;padding:7px 4px 4px;border-bottom:1px solid #8d939b}.layer{padding:6px;background:#ffffff;border:1px solid #b1b6bd}.layer.selected{background:#6f8fbe;color:#ffffff}.eye{margin-right:7px}#inspector row{height:30px;gap:5px;align-items:center}#inspector label{width:48px}#inspector input{flex:1;min-width:0}#status{height:30px;padding:6px 9px;background:#e2e4e7;border-top:1px solid #777d86}";

#[cfg(target_arch = "m68k")]
struct EditorState {
    visible: u8,
    undo: u8,
    redo: u8,
    can_undo: bool,
    can_redo: bool,
    preview: bool,
    title: &'static core::ffi::CStr,
}

#[cfg(target_arch = "m68k")]
impl EditorState {
    const fn new() -> Self {
        Self { visible: 0b00011, undo: 0, redo: 0, can_undo: false, can_redo: false, preview: false, title: c"VectorCraft - Ready (2 objects)" }
    }

    fn commit(&mut self, visible: u8, title: &'static core::ffi::CStr) {
        if visible != self.visible {
            self.undo = self.visible;
            self.visible = visible;
            self.can_undo = true;
            self.can_redo = false;
        }
        self.title = title;
    }
}

#[cfg(target_arch = "m68k")]
impl amiga_gfopenlayout::EventSink for EditorState {
    fn on_event(&mut self, event: amiga_gfopenlayout::Event<'_>) {
        use amiga_gfopenlayout::EventKind;

        if event.kind == EventKind::Click {
            if event.id == c"new" {
                self.commit(0, c"VectorCraft - New blank document");
            } else if event.id == c"rectangle" {
                let bit = if self.visible & 0b00001 == 0 { 0b00001 } else { 0b00100 };
                self.commit(self.visible | bit, c"VectorCraft - Rectangle added");
            } else if event.id == c"ellipse" {
                let bit = if self.visible & 0b00010 == 0 { 0b00010 } else { 0b01000 };
                self.commit(self.visible | bit, c"VectorCraft - Ellipse added");
            } else if event.id == c"text" {
                self.commit(self.visible | 0b10000, c"VectorCraft - Text label added");
            } else if event.id == c"select" {
                self.title = c"VectorCraft - Selection tool active";
            } else if event.id == c"undo" && self.can_undo {
                self.redo = self.visible;
                self.visible = self.undo;
                self.can_redo = true;
                self.can_undo = false;
                self.title = c"VectorCraft - Last edit undone";
            } else if event.id == c"redo" && self.can_redo {
                self.undo = self.visible;
                self.visible = self.redo;
                self.can_undo = true;
                self.can_redo = false;
                self.title = c"VectorCraft - Edit restored";
            } else if event.id == c"preview" {
                self.preview = !self.preview;
                self.title = if self.preview { c"VectorCraft - Preview mode" } else { c"VectorCraft - Editor mode" };
            }
        }
    }

    fn window_title(&self) -> Option<&core::ffi::CStr> {
        Some(self.title)
    }
}

#[cfg(target_arch = "m68k")]
#[allow(unsafe_code, reason = "the fixed AmigaDOS entry symbol is the port's isolated ABI boundary")]
#[unsafe(no_mangle)]
pub extern "C" fn gf_amiga_main(_args: *const u8, _length: i32) -> i32 {
    amiga_runtime::run(|runtime| {
        runtime.put_str(c"VectorCraft: native entry\n");
        runtime.put_str(c"VectorCraft: opening editor window\n");
        let Ok(window) = amiga_gfopenlayout::Window::open(runtime, c"VectorCraft - Untitled", EDITOR_MARKUP, EDITOR_CSS, 900, 760) else {
            runtime.put_str(c"VectorCraft: could not open GFOpenLayout editor window\n");
            return 25;
        };
        runtime.put_str(c"VectorCraft: editor window open\n");
        let mut editor = EditorState::new();
        if !window.run_with(runtime, &mut editor) {
            runtime.put_str(c"VectorCraft: event binding failed\n");
            return 26;
        }
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
