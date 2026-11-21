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
const EDITOR_MARKUP: &core::ffi::CStr = c"<column id=app><row id=toolbar><button id=new title='Start a blank document'>New</button><button id=open disabled title='File open is planned for RC2'>Open</button><button id=save disabled title='File save is planned for RC2'>Save</button><div class=rule></div><button id=undo disabled>Undo</button><button id=redo disabled>Redo</button><div class=spacer></div><label for=zoom>Zoom</label><select id=zoom disabled title='Interactive zoom is planned for RC2'><option selected>100%</option></select></row><row id=workspace><column id=tools><div class=heading>Tools</div><button id=select class=active>Select</button><button id=rectangle>Add Rect</button><button id=ellipse>Add Ellipse</button><button id=text>Add Label</button></column><column id=center><row id=documentbar><div class=tab>Untitled *</div><div class=spacer></div><button id=preview>Preview</button></row><div id=viewport><div id=artboard><div id=shape1 class=shape>Canvas preview</div><div id=shape2 class=shape2>Vector paths</div><div id=shape3 class=shape3 hidden>Shape palette</div><div id=shape4 class=shape4 hidden>Stroke sample</div><div id=shape5 class=label hidden>Made on Amiga</div></div></div></column><column id=inspector><div class=heading>Layers</div><div id=layer1 class='layer selected'><span class=eye>*</span> Active object</div><div id=layer2 class=layer><span class=eye>*</span> Rectangle</div><div id=layer3 class=layer hidden><span class=eye>*</span> Ellipse</div><div class=heading>Properties</div><row><label>X</label><input value='72'></row><row><label>Y</label><input value='72'></row><row><label>W</label><input value='468'></row><row><label>H</label><input value='648'></row><div class=heading>Appearance</div><row><label>Fill</label><input value='#5577cc'></row><row><label>Stroke</label><input value='#203050'></row></column></row><row id=status><div id=status_message>Ready - choose a tool to edit the document</div><div class=spacer></div><div>Native 68k</div></row></column>";

#[cfg(target_arch = "m68k")]
const EDITOR_CSS: &core::ffi::CStr = c"*{box-sizing:border-box}#app{width:100%;height:100%;background:#c8cbd0;color:#20242a}#toolbar{height:44px;padding:6px;gap:5px;background:#e4e6e9;border-bottom:1px solid #727780;align-items:center}#toolbar button{min-width:58px;height:30px}.spacer{flex:1}.rule{width:1px;height:28px;background:#9298a2;margin:0 3px}#workspace{flex:1;min-height:0}#tools{width:108px;padding:8px;gap:7px;background:#d7dade;border-right:1px solid #777d86}#tools button{height:36px;text-align:left}#tools .active{background:#496da8;color:#ffffff}#center{flex:1;min-width:0;background:#777d86}#documentbar{height:36px;padding:4px 7px;background:#d8dbe0;border-bottom:1px solid #666c74;align-items:center}.tab{padding:6px 16px;background:#f4f5f6;border:1px solid #858b94;border-bottom:0}#viewport{flex:1;padding:20px;background:#6f757d;align-items:center;justify-content:center;overflow:auto}#artboard{width:420px;height:500px;background:#ffffff;border:1px solid #34383e;box-shadow:3px 4px 8px #3e4248;position:relative}.shape,.shape2,.shape3,.shape4{position:absolute;padding:8px;text-align:center;color:#203050;background:#eef3ff;border:3px solid #5577cc}.shape{left:64px;top:62px;width:286px;height:150px}.shape2{left:155px;top:190px;width:170px;height:106px;border-radius:53px;background:#fff3d3;border-color:#c48724}.shape3{left:45px;top:310px;width:150px;height:85px;background:#e7f5ec;border-color:#3c8c61}.shape4{left:245px;top:330px;width:120px;height:75px;border-radius:38px;background:#fbe7eb;border-color:#b54b5a}.label{position:absolute;left:105px;top:430px;font-size:25px;font-weight:bold;color:#263650}#inspector{width:218px;padding:8px;gap:5px;background:#eceef0;border-left:1px solid #686e77}.heading{font-weight:bold;padding:7px 4px 4px;border-bottom:1px solid #8d939b}.layer{padding:6px;background:#ffffff;border:1px solid #b1b6bd}.layer.selected{background:#6f8fbe;color:#ffffff}.eye{margin-right:7px}#inspector row{height:30px;gap:5px;align-items:center}#inspector label{width:48px}#inspector input{flex:1;min-width:0}#status{height:30px;padding:6px 9px;background:#e2e4e7;border-top:1px solid #777d86}";

#[cfg(target_arch = "m68k")]
struct EditorState {
    document: model::Document,
    can_undo: bool,
    redo: Option<model::Path>,
    preview: bool,
    title: &'static core::ffi::CStr,
    status: &'static core::ffi::CStr,
}

#[cfg(target_arch = "m68k")]
impl EditorState {
    fn new() -> Result<Self, model::ModelError> {
        let mut document = model::Document::new("Untitled", "Artboard 1", 420.0, 500.0)?;
        let artboard = document.artboards()[0].id;
        document.add_path(
            artboard,
            &[
                model::Point { x: 64.0, y: 62.0 },
                model::Point { x: 350.0, y: 62.0 },
                model::Point { x: 350.0, y: 212.0 },
                model::Point { x: 64.0, y: 212.0 },
            ],
            true,
        )?;
        document.add_path(
            artboard,
            &[
                model::Point { x: 155.0, y: 190.0 },
                model::Point { x: 325.0, y: 190.0 },
                model::Point { x: 325.0, y: 296.0 },
                model::Point { x: 155.0, y: 296.0 },
            ],
            true,
        )?;
        Ok(Self {
            document,
            can_undo: false,
            redo: None,
            preview: false,
            title: c"VectorCraft - Ready (2 objects)",
            status: c"Ready - choose a tool to edit the document",
        })
    }

    fn add_object(&mut self, points: &[model::Point], title: &'static core::ffi::CStr, status: &'static core::ffi::CStr) {
        let artboard = self.document.artboards()[0].id;
        match self.document.add_path(artboard, points, true) {
            Ok(_) => {
                self.can_undo = true;
                self.redo = None;
                self.title = title;
                self.status = status;
            }
            Err(_) => {
                self.title = c"VectorCraft - Edit refused";
                self.status = c"The object could not be added within the document limits";
            }
        }
    }
}

#[cfg(target_arch = "m68k")]
impl amiga_gfopenlayout::EventSink for EditorState {
    fn on_event(&mut self, event: amiga_gfopenlayout::Event<'_>) -> bool {
        use amiga_gfopenlayout::EventKind;

        if event.kind != EventKind::Click {
            return false;
        }
        if event.id == c"new" {
            let Ok(document) = model::Document::new("Untitled", "Artboard 1", 420.0, 500.0) else {
                self.title = c"VectorCraft - New document failed";
                self.status = c"Not enough memory for a new document";
                return true;
            };
            self.document = document;
            self.can_undo = false;
            self.redo = None;
            self.title = c"VectorCraft - New blank document";
            self.status = c"Blank document created";
        } else if event.id == c"rectangle" {
            self.add_object(
                &[
                    model::Point { x: 45.0, y: 310.0 },
                    model::Point { x: 195.0, y: 310.0 },
                    model::Point { x: 195.0, y: 395.0 },
                    model::Point { x: 45.0, y: 395.0 },
                ],
                c"VectorCraft - Rectangle added",
                c"Rectangle added to the active artboard",
            );
        } else if event.id == c"ellipse" {
            self.add_object(
                &[
                    model::Point { x: 245.0, y: 330.0 },
                    model::Point { x: 365.0, y: 330.0 },
                    model::Point { x: 365.0, y: 405.0 },
                    model::Point { x: 245.0, y: 405.0 },
                ],
                c"VectorCraft - Ellipse added",
                c"Ellipse path added to the active artboard",
            );
        } else if event.id == c"text" {
            self.add_object(
                &[
                    model::Point { x: 105.0, y: 430.0 },
                    model::Point { x: 315.0, y: 430.0 },
                    model::Point { x: 315.0, y: 462.0 },
                    model::Point { x: 105.0, y: 462.0 },
                ],
                c"VectorCraft - Text label added",
                c"Text frame added to the active artboard",
            );
        } else if event.id == c"select" {
            self.title = c"VectorCraft - Selection tool active";
            self.status = c"Selection tool active";
        } else if event.id == c"undo" && self.can_undo {
            self.redo = self.document.remove_last_path();
            self.can_undo = false;
            self.title = c"VectorCraft - Last edit undone";
            self.status = c"Last object removed; Redo is available";
        } else if event.id == c"redo" {
            let Some(path) = self.redo.take() else {
                return false;
            };
            if self.document.restore_path(path).is_ok() {
                self.can_undo = true;
                self.title = c"VectorCraft - Edit restored";
                self.status = c"Last object restored";
            } else {
                self.title = c"VectorCraft - Redo failed";
                self.status = c"The object could not be restored within the document limits";
            }
        } else if event.id == c"preview" {
            self.preview = !self.preview;
            self.title = if self.preview { c"VectorCraft - Preview mode" } else { c"VectorCraft - Editor mode" };
            self.status = if self.preview { c"Preview mode hides editor guides" } else { c"Editor mode restored" };
        } else {
            return false;
        }
        true
    }

    fn click_ids(&self) -> &'static [&'static core::ffi::CStr] {
        &[c"new", c"undo", c"redo", c"select", c"rectangle", c"ellipse", c"text", c"preview"]
    }

    fn window_title(&self) -> Option<&core::ffi::CStr> {
        Some(self.title)
    }

    fn refresh(&mut self, ui: &mut amiga_gfopenlayout::Ui<'_, '_>) {
        let visible = self.document.paths().len();
        let _ = ui.set_hidden(c"shape1", visible < 1);
        let _ = ui.set_hidden(c"shape2", visible < 2);
        let _ = ui.set_hidden(c"shape3", visible < 3);
        let _ = ui.set_hidden(c"shape4", visible < 4);
        let _ = ui.set_hidden(c"shape5", visible < 5);
        let _ = ui.set_hidden(c"layer1", visible < 1);
        let _ = ui.set_hidden(c"layer2", visible < 2);
        let _ = ui.set_hidden(c"layer3", visible < 3);
        let _ = ui.set_disabled(c"undo", !self.can_undo);
        let _ = ui.set_disabled(c"redo", self.redo.is_none());
        let _ = ui.set_text(c"status_message", self.status);
        let artboard_style = if self.preview {
            c"background:#ffffff;border:0;box-shadow:none"
        } else {
            c"background:#ffffff;border:1px solid #34383e;box-shadow:3px 4px 8px #3e4248"
        };
        let _ = ui.set_style(c"artboard", artboard_style);
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
        let Ok(mut editor) = EditorState::new() else {
            runtime.put_str(c"VectorCraft: could not create bounded editor state\n");
            return 27;
        };
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
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {
        core::hint::spin_loop();
    }
}

#[cfg(not(target_arch = "m68k"))]
fn main() {}
