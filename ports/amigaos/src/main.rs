// Copyright (c) 2026 AmiCradle Team. MIT OR Apache-2.0.
#![cfg_attr(target_arch = "m68k", no_main)]
#![cfg_attr(target_arch = "m68k", no_std)]
#![deny(clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::unimplemented, clippy::todo, clippy::unreachable)]

extern crate alloc;

#[cfg(any(target_arch = "m68k", test))]
pub mod model;
#[cfg(any(target_arch = "m68k", test))]
#[cfg(test)]
mod platform;

#[cfg(target_arch = "m68k")]
const EDITOR_MARKUP: &core::ffi::CStr = c"<column id=app><row id=toolbar><button id=new title='Start a blank document'>New</button><button id=open title='Open SYS:VectorCraft.vcf'>Open</button><button id=save title='Save SYS:VectorCraft.vcf'>Save</button><div class=rule></div><button id=undo disabled>Undo</button><button id=redo disabled>Redo</button><div class=spacer></div><label for=zoom>Zoom</label><select id=zoom disabled title='Interactive zoom is planned for RC2'><option selected>100%</option></select></row><row id=workspace><column id=tools><div class=heading>Tools</div><button id=select class=active>Select</button><button id=rectangle>Add Rect</button><button id=ellipse>Add Ellipse</button><button id=text>Add Label</button></column><column id=center><row id=documentbar><div class=tab>VectorCraft.vcf</div><div class=spacer></div><button id=preview>Preview</button></row><div id=viewport><div id=artboard><div id=shape1 class=shape>Canvas preview</div><div id=shape2 class=shape2>Vector paths</div><div id=shape3 class=shape3 hidden>Shape palette</div><div id=shape4 class=shape4 hidden>Stroke sample</div><div id=shape5 class=label hidden>Made on Amiga</div></div></div></column><column id=inspector><div class=heading>Layers</div><div id=layer1 class='layer selected'><span class=eye>*</span> Active object</div><div id=layer2 class=layer><span class=eye>*</span> Rectangle</div><div id=layer3 class=layer hidden><span class=eye>*</span> Ellipse</div><div class=heading>Properties</div><row><label>X</label><input value='72'></row><row><label>Y</label><input value='72'></row><row><label>W</label><input value='468'></row><row><label>H</label><input value='648'></row><div class=heading>Appearance</div><row><label>Fill</label><input value='#5577cc'></row><row><label>Stroke</label><input value='#203050'></row></column></row><row id=status><div id=status_message>Ready - choose a tool to edit the document</div><div class=spacer></div><div>Native 68k</div></row></column>";

#[cfg(target_arch = "m68k")]
const EDITOR_CSS: &core::ffi::CStr = c"*{box-sizing:border-box}#app{width:100%;height:100%;background:#c8cbd0;color:#20242a}#toolbar{height:44px;padding:6px;gap:5px;background:#e4e6e9;border-bottom:1px solid #727780;align-items:center}#toolbar button{min-width:58px;height:30px}.spacer{flex:1}.rule{width:1px;height:28px;background:#9298a2;margin:0 3px}#workspace{flex:1;min-height:0}#tools{width:108px;padding:8px;gap:7px;background:#d7dade;border-right:1px solid #777d86}#tools button{height:36px;text-align:left}#tools .active{background:#496da8;color:#ffffff}#center{flex:1;min-width:0;background:#777d86}#documentbar{height:36px;padding:4px 7px;background:#d8dbe0;border-bottom:1px solid #666c74;align-items:center}.tab{padding:6px 16px;background:#f4f5f6;border:1px solid #858b94;border-bottom:0}#viewport{flex:1;padding:20px;background:#6f757d;align-items:center;justify-content:center;overflow:auto}#artboard{width:420px;height:500px;background:#ffffff;border:1px solid #34383e;box-shadow:3px 4px 8px #3e4248;position:relative}.shape,.shape2,.shape3,.shape4{position:absolute;padding:8px;text-align:center;color:#203050;background:#eef3ff;border:3px solid #5577cc}.shape{left:64px;top:62px;width:286px;height:150px}.shape2{left:155px;top:190px;width:170px;height:106px;border-radius:53px;background:#fff3d3;border-color:#c48724}.shape3{left:45px;top:310px;width:150px;height:85px;background:#e7f5ec;border-color:#3c8c61}.shape4{left:245px;top:330px;width:120px;height:75px;border-radius:38px;background:#fbe7eb;border-color:#b54b5a}.label{position:absolute;left:105px;top:430px;font-size:25px;font-weight:bold;color:#263650}#inspector{width:218px;padding:8px;gap:5px;background:#eceef0;border-left:1px solid #686e77}.heading{font-weight:bold;padding:7px 4px 4px;border-bottom:1px solid #8d939b}.layer{padding:6px;background:#ffffff;border:1px solid #b1b6bd}.layer.selected{background:#6f8fbe;color:#ffffff}.eye{margin-right:7px}#inspector row{height:30px;gap:5px;align-items:center}#inspector label{width:48px}#inspector input{flex:1;min-width:0}#status{height:30px;padding:6px 9px;background:#e2e4e7;border-top:1px solid #777d86}";

#[cfg(target_arch = "m68k")]
struct EditorState<'runtime> {
    runtime: &'runtime amiga_runtime::Runtime,
    document: model::Document,
    can_undo: bool,
    redo: Option<model::Path>,
    preview: bool,
    title: &'static core::ffi::CStr,
    status: &'static core::ffi::CStr,
}

#[cfg(target_arch = "m68k")]
fn model_error_text(error: model::ModelError) -> &'static core::ffi::CStr {
    use model::ModelError;

    match error {
        ModelError::EmptyName => c"VectorCraft: model error: empty name\n",
        ModelError::NameTooLong => c"VectorCraft: model error: name too long\n",
        ModelError::InvalidExtent => c"VectorCraft: model error: invalid extent\n",
        ModelError::InvalidPoint => c"VectorCraft: model error: invalid point\n",
        ModelError::NotEnoughPoints => c"VectorCraft: model error: not enough points\n",
        ModelError::MissingArtboard => c"VectorCraft: model error: missing artboard\n",
        ModelError::ArtboardLimit => c"VectorCraft: model error: artboard limit\n",
        ModelError::PathLimit => c"VectorCraft: model error: path limit\n",
        ModelError::PointLimit => c"VectorCraft: model error: point limit\n",
        ModelError::IdExhausted => c"VectorCraft: model error: ID exhausted\n",
        ModelError::OutOfMemory(_) => c"VectorCraft: model error: reserve failed\n",
        ModelError::InvalidFormat(_) => c"VectorCraft: model error: invalid VCF1 document\n",
        ModelError::BufferTooSmall => c"VectorCraft: model error: VCF1 buffer too small\n",
    }
}

#[cfg(target_arch = "m68k")]
fn put_u32(runtime: &amiga_runtime::Runtime, value: u32) {
    let mut buffer = [0_u8; 11];
    let mut cursor = buffer.len() - 1;
    let mut remaining = value;
    buffer[cursor] = 0;
    if remaining == 0 {
        cursor -= 1;
        buffer[cursor] = b'0';
    } else {
        while remaining != 0 {
            cursor -= 1;
            buffer[cursor] = b'0' + (remaining % 10) as u8;
            remaining /= 10;
        }
    }
    if let Ok(text) = core::ffi::CStr::from_bytes_until_nul(&buffer[cursor..]) {
        runtime.put_str(text);
    }
}

#[cfg(target_arch = "m68k")]
impl<'runtime> EditorState<'runtime> {
    fn new(runtime: &'runtime amiga_runtime::Runtime) -> Result<Self, model::ModelError> {
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
            runtime,
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
impl amiga_gfopenlayout::EventSink for EditorState<'_> {
    fn on_event(&mut self, event: amiga_gfopenlayout::Event<'_>) -> bool {
        use amiga_gfopenlayout::EventKind;

        if event.kind != EventKind::Click {
            return false;
        }
        if event.id == c"open" {
            let mut bytes = alloc::vec::Vec::new();
            if bytes.try_reserve_exact(65_536).is_err() {
                self.title = c"VectorCraft - Open failed: buffer reserve";
                self.status = c"Not enough memory for the bounded VCF1 buffer";
                return true;
            }
            bytes.resize(65_536, 0);
            let Ok(input) = self.runtime.read_file(c"SYS:VectorCraft.vcf", bytes.as_mut_slice()) else {
                self.title = c"VectorCraft - Open failed";
                self.status = c"SYS:VectorCraft.vcf could not be read";
                return true;
            };
            let input_length = input.len();
            let source = model::DecodeSource { bytes: bytes.as_slice(), length: input_length };
            let decoded = model::Document::decode(&source);
            let document = match decoded {
                Ok(document) => document,
                Err(model::ModelError::InvalidExtent) => {
                    self.title = c"VectorCraft - Open failed: invalid extent";
                    self.status = c"VCF1 artboard extent was rejected";
                    return true;
                }
                Err(model::ModelError::InvalidPoint) => {
                    self.title = c"VectorCraft - Open failed: invalid point";
                    self.status = c"VCF1 point coordinates were rejected";
                    return true;
                }
                Err(model::ModelError::NotEnoughPoints) => {
                    self.title = c"VectorCraft - Open failed: short path";
                    self.status = c"VCF1 path did not contain enough points";
                    return true;
                }
                Err(model::ModelError::PathLimit) => {
                    self.title = c"VectorCraft - Open failed: path limit";
                    self.status = c"VCF1 path count exceeded the native limit";
                    return true;
                }
                Err(model::ModelError::PointLimit) => {
                    self.title = c"VectorCraft - Open failed: point limit";
                    self.status = c"VCF1 point count exceeded the native limit";
                    return true;
                }
                Err(model::ModelError::InvalidFormat(stage)) => {
                    use model::FormatStage;
                    self.title = match stage {
                        FormatStage::UnsupportedDocument => c"VectorCraft - Open failed: unsupported document",
                        FormatStage::Magic => c"VectorCraft - Open failed: magic",
                        FormatStage::DocumentName => c"VectorCraft - Open failed: document name",
                        FormatStage::ArtboardName => c"VectorCraft - Open failed: artboard name",
                        FormatStage::Width => c"VectorCraft - Open failed: width",
                        FormatStage::Height => c"VectorCraft - Open failed: height",
                        FormatStage::PathCount => c"VectorCraft - Open failed: path count",
                        FormatStage::ClosedFlag => c"VectorCraft - Open failed: closed flag",
                        FormatStage::PointCount => c"VectorCraft - Open failed: point count",
                        FormatStage::PointX => c"VectorCraft - Open failed: point X",
                        FormatStage::PointY => c"VectorCraft - Open failed: point Y",
                        FormatStage::TrailingData => c"VectorCraft - Open failed: trailing data",
                        FormatStage::Framing => c"VectorCraft - Open failed: framing",
                    };
                    self.status = c"VCF1 parser rejected a bounded field";
                    return true;
                }
                Err(model::ModelError::OutOfMemory(model::AllocationSite::Artboards)) => {
                    self.title = c"VectorCraft - Open failed: artboard reserve";
                    self.status = c"VCF1 artboard allocation was rejected";
                    return true;
                }
                Err(model::ModelError::OutOfMemory(model::AllocationSite::PathPoints)) => {
                    self.title = c"VectorCraft - Open failed: point reserve";
                    self.status = c"VCF1 point allocation was rejected";
                    return true;
                }
                Err(model::ModelError::OutOfMemory(model::AllocationSite::Paths)) => {
                    self.title = c"VectorCraft - Open failed: path reserve";
                    self.status = c"VCF1 path allocation was rejected";
                    return true;
                }
                Err(model::ModelError::OutOfMemory(model::AllocationSite::DocumentName)) => {
                    self.title = c"VectorCraft - Open failed: document name reserve";
                    self.status = c"VCF1 document name allocation was rejected";
                    return true;
                }
                Err(model::ModelError::OutOfMemory(model::AllocationSite::ArtboardName)) => {
                    self.title = c"VectorCraft - Open failed: artboard name reserve";
                    self.status = c"VCF1 artboard name allocation was rejected";
                    return true;
                }
                Err(model::ModelError::EmptyName) => {
                    self.title = c"VectorCraft - Open failed: empty name";
                    self.status = c"VCF1 contained an empty required name";
                    return true;
                }
                Err(model::ModelError::NameTooLong) => {
                    self.title = c"VectorCraft - Open failed: name limit";
                    self.status = c"VCF1 name exceeded the native limit";
                    return true;
                }
                Err(model::ModelError::ArtboardLimit) => {
                    self.title = c"VectorCraft - Open failed: artboard limit";
                    self.status = c"VCF1 artboard limit was rejected";
                    return true;
                }
                Err(model::ModelError::MissingArtboard) => {
                    self.title = c"VectorCraft - Open failed: missing artboard";
                    self.status = c"VCF1 path referenced no artboard";
                    return true;
                }
                Err(model::ModelError::IdExhausted) => {
                    self.title = c"VectorCraft - Open failed: ID limit";
                    self.status = c"VCF1 object IDs were exhausted";
                    return true;
                }
                Err(model::ModelError::BufferTooSmall) => {
                    self.title = c"VectorCraft - Open failed: buffer limit";
                    self.status = c"VCF1 exceeded the native input buffer";
                    return true;
                }
            };
            self.document = document;
            self.can_undo = false;
            self.redo = None;
            self.title = c"VectorCraft - Document opened";
            self.status = c"SYS:VectorCraft.vcf opened";
        } else if event.id == c"save" {
            let mut bytes = alloc::vec::Vec::new();
            if bytes.try_reserve_exact(65_536).is_err() {
                self.title = c"VectorCraft - Save failed: buffer reserve";
                self.status = c"Not enough memory for the bounded VCF1 buffer";
                return true;
            }
            bytes.resize(65_536, 0);
            let Ok(length) = self.document.encode(bytes.as_mut_slice()) else {
                self.title = c"VectorCraft - Save refused";
                self.status = c"Document exceeds the bounded native VCF1 format";
                return true;
            };
            if self.runtime.write_file(c"SYS:VectorCraft.vcf", &bytes[..length]).is_err() {
                self.title = c"VectorCraft - Save failed";
                self.status = c"SYS:VectorCraft.vcf could not be written";
                return true;
            }
            self.title = c"VectorCraft - Document saved";
            self.status = c"SYS:VectorCraft.vcf saved";
        } else if event.id == c"new" {
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
        &[c"new", c"open", c"save", c"undo", c"redo", c"select", c"rectangle", c"ellipse", c"text", c"preview"]
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
        let editor_state = EditorState::new(runtime);
        runtime.put_str(c"VectorCraft: opening editor window\n");
        let Ok(mut window) = amiga_gfopenlayout::Window::open(runtime, c"VectorCraft - Untitled", EDITOR_MARKUP, EDITOR_CSS, 900, 760) else {
            runtime.put_str(c"VectorCraft: could not open GFOpenLayout editor window\n");
            return 25;
        };
        runtime.put_str(c"VectorCraft: editor window open\n");
        let mut editor = match editor_state {
            Ok(editor) => editor,
            Err(error) => {
                let (bytes, had_sys_base) = runtime.allocation_diagnostic();
                let mut diagnostic = [0_u8; 24];
                diagnostic[0..4].copy_from_slice(&bytes.to_be_bytes());
                diagnostic[4..8].copy_from_slice(&(core::mem::size_of::<model::Artboard>() as u32).to_be_bytes());
                diagnostic[8..12].copy_from_slice(&(core::mem::align_of::<model::Artboard>() as u32).to_be_bytes());
                diagnostic[12..16].copy_from_slice(&(isize::MAX as u32).to_be_bytes());
                diagnostic[16..20].copy_from_slice(&(usize::MAX as u32).to_be_bytes());
                let site = match error {
                    model::ModelError::OutOfMemory(model::AllocationSite::DocumentName) => 1_u32,
                    model::ModelError::OutOfMemory(model::AllocationSite::ArtboardName) => 2,
                    model::ModelError::OutOfMemory(model::AllocationSite::Artboards) => 3,
                    model::ModelError::OutOfMemory(model::AllocationSite::PathPoints) => 4,
                    model::ModelError::OutOfMemory(model::AllocationSite::Paths) => 5,
                    _ => 0,
                };
                diagnostic[20..24].copy_from_slice(&site.to_be_bytes());
                let _ = runtime.write_file(c"SYS:vector.alloc", &diagnostic);
                let title = match error {
                    model::ModelError::InvalidExtent => c"VectorCraft - model rejected floating-point extent",
                    model::ModelError::InvalidPoint => c"VectorCraft - model rejected floating-point point",
                    model::ModelError::OutOfMemory(model::AllocationSite::DocumentName) => c"VectorCraft - reserve failed: document name",
                    model::ModelError::OutOfMemory(model::AllocationSite::ArtboardName) => c"VectorCraft - reserve failed: artboard name",
                    model::ModelError::OutOfMemory(model::AllocationSite::Artboards) => c"VectorCraft - reserve failed: artboard list",
                    model::ModelError::OutOfMemory(model::AllocationSite::PathPoints) => c"VectorCraft - reserve failed: path points",
                    model::ModelError::OutOfMemory(model::AllocationSite::Paths) => c"VectorCraft - reserve failed: path list",
                    _ => c"VectorCraft - model construction failed",
                };
                window.set_title(title);
                runtime.put_str(c"VectorCraft: last allocation bytes: ");
                put_u32(runtime, bytes);
                runtime.put_str(c"\n");
                runtime.put_str(if had_sys_base {
                    c"VectorCraft: allocator had ExecBase\n"
                } else {
                    c"VectorCraft: allocator did not have ExecBase\n"
                });
                runtime.put_str(c"VectorCraft: could not create bounded editor state\n");
                runtime.put_str(model_error_text(error));
                window.run(runtime);
                return 27;
            }
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
