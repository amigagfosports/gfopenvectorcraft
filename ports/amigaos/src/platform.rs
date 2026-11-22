// Copyright (c) 2026 AmiCradle Team. MIT OR Apache-2.0.
use core::ffi::CStr;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Service {
    UserInterface,
    Rendering,
    Media,
    Updates,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Binding {
    pub service: Service,
    pub backend: &'static str,
}

pub const BINDINGS: [Binding; 4] = [
    Binding { service: Service::UserInterface, backend: "GFOpenLayout and OpenGadTools" },
    Binding { service: Service::Rendering, backend: "OpenRTG and OpenGPU" },
    Binding { service: Service::Media, backend: "GFOpenDataTypes" },
    Binding { service: Service::Updates, backend: "GFOpenUp" },
];

pub const STARTUP_LINES: [&CStr; 6] = [
    c"GFOpenVectorCraft native AmigaOS 68k port\n",
    c"platform boundaries:\n",
    c"  UI: GFOpenLayout + OpenGadTools\n",
    c"  render: OpenRTG + OpenGPU\n",
    c"  media: GFOpenDataTypes\n",
    c"  updates: GFOpenUp\n",
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_service_has_one_native_binding() {
        assert_eq!(STARTUP_LINES.len(), BINDINGS.len() + 2);
        for (index, binding) in BINDINGS.iter().enumerate() {
            assert!(!binding.backend.is_empty());
            for other in BINDINGS.iter().skip(index + 1) {
                assert_ne!(binding.service, other.service);
            }
        }
    }
}
