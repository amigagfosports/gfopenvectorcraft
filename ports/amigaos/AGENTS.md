# AmigaOS port boundary

This is a native AmigaOS 3.x 68k port. Do not replace it with a host wrapper,
webview, JavaScript shell or remote UI.

Only `src/main.rs` may allow the `no_mangle` entry attribute. Application code
must contain no unsafe block; Amiga ABI calls and ownership stay behind the
pinned GFOpenRust safe callback API.
