// Copyright (c) 2026 AmiCradle Team. MIT OR Apache-2.0.
//! Bounded native subset of VectorCraft's document and path concepts.

use alloc::string::String;
use alloc::vec::Vec;

pub const MAX_NAME_BYTES: usize = 256;
pub const MAX_ARTBOARDS: usize = 64;
pub const MAX_PATHS: usize = 4_096;
pub const MAX_POINTS_PER_PATH: usize = 16_384;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Limits {
    pub artboards: usize,
    pub paths: usize,
    pub points_per_path: usize,
}

impl Default for Limits {
    fn default() -> Self {
        Self { artboards: MAX_ARTBOARDS, paths: MAX_PATHS, points_per_path: MAX_POINTS_PER_PATH }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AllocationSite {
    DocumentName,
    ArtboardName,
    Artboards,
    PathPoints,
    Paths,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FormatStage {
    UnsupportedDocument,
    Magic,
    DocumentName,
    ArtboardName,
    Width,
    Height,
    PathCount,
    ClosedFlag,
    PointCount,
    PointX,
    PointY,
    TrailingData,
    Framing,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ModelError {
    EmptyName,
    NameTooLong,
    InvalidExtent,
    InvalidPoint,
    NotEnoughPoints,
    MissingArtboard,
    ArtboardLimit,
    PathLimit,
    PointLimit,
    IdExhausted,
    OutOfMemory(AllocationSite),
    InvalidFormat(FormatStage),
    BufferTooSmall,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Artboard {
    pub id: u64,
    pub name: String,
    pub width: f64,
    pub height: f64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Path {
    pub id: u64,
    pub artboard_id: u64,
    pub points: Vec<Point>,
    pub closed: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Document {
    name: String,
    artboards: Vec<Artboard>,
    paths: Vec<Path>,
    next_id: u64,
    limits: Limits,
}

/// One bounded file buffer and its exact populated length.
///
/// Passing this descriptor by one thin reference avoids the experimental
/// m68k backend's broken multi-argument ABI for a fixed buffer plus `usize`.
pub struct DecodeSource<'input> {
    pub bytes: &'input [u8],
    pub length: usize,
}

impl Document {
    pub fn new(name: &str, artboard_name: &str, width: f64, height: f64) -> Result<Self, ModelError> {
        Self::with_limits(name, artboard_name, width, height, Limits::default())
    }

    pub fn with_limits(name: &str, artboard_name: &str, width: f64, height: f64, mut limits: Limits) -> Result<Self, ModelError> {
        limits.artboards = limits.artboards.min(MAX_ARTBOARDS);
        limits.paths = limits.paths.min(MAX_PATHS);
        limits.points_per_path = limits.points_per_path.min(MAX_POINTS_PER_PATH);
        if limits.artboards == 0 {
            return Err(ModelError::ArtboardLimit);
        }
        let name = copy_name(name, AllocationSite::DocumentName)?;
        let mut document = Self { name, artboards: Vec::new(), paths: Vec::new(), next_id: 1, limits };
        document.add_artboard(artboard_name, width, height)?;
        Ok(document)
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn artboards(&self) -> &[Artboard] {
        &self.artboards
    }

    pub fn paths(&self) -> &[Path] {
        &self.paths
    }

    /// Encode the native editor subset without allocating.
    ///
    /// VCF1 deliberately covers one artboard, which is the bounded surface
    /// exposed by the native 68k editor. Multi-artboard desktop documents are
    /// rejected instead of being silently flattened.
    pub fn encode(&self, output: &mut [u8]) -> Result<usize, ModelError> {
        if self.artboards.len() != 1 || self.paths.len() > u16::MAX as usize {
            return Err(ModelError::InvalidFormat(FormatStage::UnsupportedDocument));
        }
        let artboard = &self.artboards[0];
        if !self.name.is_ascii() || !artboard.name.is_ascii() {
            return Err(ModelError::InvalidFormat(FormatStage::UnsupportedDocument));
        }
        if self.paths.iter().any(|path| path.artboard_id != artboard.id || path.points.len() > u16::MAX as usize) {
            return Err(ModelError::InvalidFormat(FormatStage::UnsupportedDocument));
        }
        let mut writer = Writer::new(output);
        writer.bytes(b"VCF1")?;
        writer.text(&self.name)?;
        writer.text(&artboard.name)?;
        writer.u64(artboard.width.to_bits())?;
        writer.u64(artboard.height.to_bits())?;
        writer.u16(self.paths.len() as u16)?;
        for path in &self.paths {
            writer.u8(u8::from(path.closed))?;
            writer.u16(path.points.len() as u16)?;
            for point in &path.points {
                writer.u64(point.x.to_bits())?;
                writer.u64(point.y.to_bits())?;
            }
        }
        Ok(writer.len())
    }

    /// Decode one complete bounded VCF1 document.
    pub fn decode(source: &DecodeSource<'_>) -> Result<Self, ModelError> {
        let input = source.bytes;
        let length = source.length;
        if length > input.len() {
            return Err(ModelError::InvalidFormat(FormatStage::Framing));
        }
        if length < 4 || input[0] != b'V' || input[1] != b'C' || input[2] != b'F' || input[3] != b'1' {
            return Err(ModelError::InvalidFormat(FormatStage::Magic));
        }
        let mut cursor = 6_usize;
        macro_rules! read_byte {
            ($stage:expr) => {{
                if cursor >= length {
                    return Err(ModelError::InvalidFormat($stage));
                }
                let value = input[cursor];
                cursor += 1;
                value
            }};
        }
        macro_rules! read_u16 {
            ($stage:expr) => {{
                let high = read_byte!($stage) as u16;
                let low = read_byte!($stage) as u16;
                (high << 8) | low
            }};
        }
        macro_rules! read_u64 {
            ($stage:expr) => {{
                let end = cursor.checked_add(8).ok_or(ModelError::InvalidFormat($stage))?;
                let bytes = input.get(cursor..end).ok_or(ModelError::InvalidFormat($stage))?;
                cursor = end;
                u64::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7]])
            }};
        }

        if length < 6 {
            return Err(ModelError::InvalidFormat(FormatStage::DocumentName));
        }
        let name_length = ((input[4] as usize) << 8) | input[5] as usize;
        if name_length == 0 || name_length > MAX_NAME_BYTES {
            return Err(ModelError::InvalidFormat(FormatStage::DocumentName));
        }
        let mut name = String::new();
        name.try_reserve_exact(name_length).map_err(|_| ModelError::OutOfMemory(AllocationSite::DocumentName))?;
        let name_end = cursor.checked_add(name_length).ok_or(ModelError::InvalidFormat(FormatStage::DocumentName))?;
        if name_end > length {
            return Err(ModelError::InvalidFormat(FormatStage::DocumentName));
        }
        while cursor < name_end {
            let byte = input[cursor];
            if !byte.is_ascii() {
                return Err(ModelError::InvalidFormat(FormatStage::DocumentName));
            }
            name.push(char::from(byte));
            cursor += 1;
        }

        if cursor.checked_add(2).is_none_or(|end| end > length) {
            return Err(ModelError::InvalidFormat(FormatStage::ArtboardName));
        }
        let artboard_name_length = ((input[cursor] as usize) << 8) | input[cursor + 1] as usize;
        cursor += 2;
        if artboard_name_length == 0 || artboard_name_length > MAX_NAME_BYTES {
            return Err(ModelError::InvalidFormat(FormatStage::ArtboardName));
        }
        let mut artboard_name = String::new();
        artboard_name.try_reserve_exact(artboard_name_length).map_err(|_| ModelError::OutOfMemory(AllocationSite::ArtboardName))?;
        let artboard_name_end = cursor.checked_add(artboard_name_length).ok_or(ModelError::InvalidFormat(FormatStage::ArtboardName))?;
        if artboard_name_end > length {
            return Err(ModelError::InvalidFormat(FormatStage::ArtboardName));
        }
        while cursor < artboard_name_end {
            let byte = input[cursor];
            if !byte.is_ascii() {
                return Err(ModelError::InvalidFormat(FormatStage::ArtboardName));
            }
            artboard_name.push(char::from(byte));
            cursor += 1;
        }

        let width = f64::from_bits(read_u64!(FormatStage::Width));
        let height = f64::from_bits(read_u64!(FormatStage::Height));
        let path_count = read_u16!(FormatStage::PathCount) as usize;
        if path_count > MAX_PATHS {
            return Err(ModelError::PathLimit);
        }
        validate_extent(width, height)?;
        let limits = Limits::default();
        let mut artboards = Vec::new();
        artboards.try_reserve_exact(1).map_err(|_| ModelError::OutOfMemory(AllocationSite::Artboards))?;
        let artboard_id = 1;
        artboards.push(Artboard { id: artboard_id, name: artboard_name, width, height });
        let mut document = Self { name, artboards, paths: Vec::new(), next_id: 2, limits };
        for _ in 0..path_count {
            let closed = match read_byte!(FormatStage::ClosedFlag) {
                0 => false,
                1 => true,
                _ => return Err(ModelError::InvalidFormat(FormatStage::ClosedFlag)),
            };
            let point_count = read_u16!(FormatStage::PointCount) as usize;
            if point_count > document.limits.points_per_path {
                return Err(ModelError::PointLimit);
            }
            let mut points = Vec::new();
            points.try_reserve_exact(point_count).map_err(|_| ModelError::OutOfMemory(AllocationSite::PathPoints))?;
            for _ in 0..point_count {
                let x = f64::from_bits(read_u64!(FormatStage::PointX));
                let y = f64::from_bits(read_u64!(FormatStage::PointY));
                points.push(Point { x, y });
            }
            document.add_path(artboard_id, &points, closed)?;
        }
        if cursor != length {
            return Err(ModelError::InvalidFormat(FormatStage::TrailingData));
        }
        Ok(document)
    }

    pub fn remove_last_path(&mut self) -> Option<Path> {
        self.paths.pop()
    }

    pub fn restore_path(&mut self, path: Path) -> Result<(), ModelError> {
        if !self.artboards.iter().any(|artboard| artboard.id == path.artboard_id) {
            return Err(ModelError::MissingArtboard);
        }
        if self.paths.len() >= self.limits.paths {
            return Err(ModelError::PathLimit);
        }
        if path.id == 0 || path.id >= self.next_id || self.paths.iter().any(|existing| existing.id == path.id) {
            return Err(ModelError::IdExhausted);
        }
        let minimum = if path.closed { 3 } else { 2 };
        if path.points.len() < minimum {
            return Err(ModelError::NotEnoughPoints);
        }
        if path.points.len() > self.limits.points_per_path {
            return Err(ModelError::PointLimit);
        }
        if path.points.iter().any(|point| !point.x.is_finite() || !point.y.is_finite()) {
            return Err(ModelError::InvalidPoint);
        }
        self.paths.try_reserve_exact(1).map_err(|_| ModelError::OutOfMemory(AllocationSite::Paths))?;
        self.paths.push(path);
        Ok(())
    }

    pub fn add_artboard(&mut self, name: &str, width: f64, height: f64) -> Result<u64, ModelError> {
        if self.artboards.len() >= self.limits.artboards {
            return Err(ModelError::ArtboardLimit);
        }
        validate_extent(width, height)?;
        let name = copy_name(name, AllocationSite::ArtboardName)?;
        self.artboards.try_reserve_exact(1).map_err(|_| ModelError::OutOfMemory(AllocationSite::Artboards))?;
        let id = self.take_id()?;
        self.artboards.push(Artboard { id, name, width, height });
        Ok(id)
    }

    pub fn add_path(&mut self, artboard_id: u64, points: &[Point], closed: bool) -> Result<u64, ModelError> {
        if !self.artboards.iter().any(|artboard| artboard.id == artboard_id) {
            return Err(ModelError::MissingArtboard);
        }
        if self.paths.len() >= self.limits.paths {
            return Err(ModelError::PathLimit);
        }
        let minimum = if closed { 3 } else { 2 };
        if points.len() < minimum {
            return Err(ModelError::NotEnoughPoints);
        }
        if points.len() > self.limits.points_per_path {
            return Err(ModelError::PointLimit);
        }
        if points.iter().any(|point| !point.x.is_finite() || !point.y.is_finite()) {
            return Err(ModelError::InvalidPoint);
        }
        let mut owned = Vec::new();
        owned.try_reserve_exact(points.len()).map_err(|_| ModelError::OutOfMemory(AllocationSite::PathPoints))?;
        owned.extend_from_slice(points);
        self.paths.try_reserve_exact(1).map_err(|_| ModelError::OutOfMemory(AllocationSite::Paths))?;
        let id = self.take_id()?;
        self.paths.push(Path { id, artboard_id, points: owned, closed });
        Ok(id)
    }

    fn take_id(&mut self) -> Result<u64, ModelError> {
        let id = self.next_id;
        self.next_id = self.next_id.checked_add(1).ok_or(ModelError::IdExhausted)?;
        Ok(id)
    }
}

fn copy_name(name: &str, site: AllocationSite) -> Result<String, ModelError> {
    if name.is_empty() {
        return Err(ModelError::EmptyName);
    }
    if name.len() > MAX_NAME_BYTES {
        return Err(ModelError::NameTooLong);
    }
    let mut owned = String::new();
    owned.try_reserve_exact(name.len()).map_err(|_| ModelError::OutOfMemory(site))?;
    owned.push_str(name);
    Ok(owned)
}

fn validate_extent(width: f64, height: f64) -> Result<(), ModelError> {
    if width.is_finite() && height.is_finite() && width > 0.0 && height > 0.0 { Ok(()) } else { Err(ModelError::InvalidExtent) }
}

struct Writer<'buffer> {
    output: &'buffer mut [u8],
    cursor: usize,
}

impl<'buffer> Writer<'buffer> {
    fn new(output: &'buffer mut [u8]) -> Self {
        Self { output, cursor: 0 }
    }

    fn len(&self) -> usize {
        self.cursor
    }

    fn bytes(&mut self, bytes: &[u8]) -> Result<(), ModelError> {
        let end = self.cursor.checked_add(bytes.len()).ok_or(ModelError::BufferTooSmall)?;
        let target = self.output.get_mut(self.cursor..end).ok_or(ModelError::BufferTooSmall)?;
        target.copy_from_slice(bytes);
        self.cursor = end;
        Ok(())
    }

    fn u8(&mut self, value: u8) -> Result<(), ModelError> {
        self.bytes(&[value])
    }

    fn u16(&mut self, value: u16) -> Result<(), ModelError> {
        self.bytes(&value.to_be_bytes())
    }

    fn u64(&mut self, value: u64) -> Result<(), ModelError> {
        self.bytes(&value.to_be_bytes())
    }

    fn text(&mut self, value: &str) -> Result<(), ModelError> {
        let length = u16::try_from(value.len()).map_err(|_| ModelError::NameTooLong)?;
        self.u16(length)?;
        self.bytes(value.as_bytes())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builds_a_bounded_vector_document_with_stable_ids() -> Result<(), ModelError> {
        let mut document = Document::new("Poster", "Artboard 1", 612.0, 792.0)?;
        let first_artboard = document.artboards()[0].id;
        let path = document.add_path(
            first_artboard,
            &[Point { x: 0.0, y: 0.0 }, Point { x: 100.0, y: 0.0 }, Point { x: 100.0, y: 100.0 }, Point { x: 0.0, y: 100.0 }],
            true,
        )?;
        let second_artboard = document.add_artboard("Artboard 2", 320.0, 200.0)?;
        assert_eq!(document.name(), "Poster");
        assert_eq!((first_artboard, path, second_artboard), (1, 2, 3));
        assert_eq!(document.paths()[0].points.len(), 4);
        Ok(())
    }

    #[test]
    fn invalid_geometry_fails_closed() -> Result<(), ModelError> {
        assert_eq!(Document::new("", "Artboard", 1.0, 1.0), Err(ModelError::EmptyName));
        assert_eq!(Document::new("Doc", "Artboard", 0.0, 1.0), Err(ModelError::InvalidExtent));
        let mut document = Document::new("Doc", "Artboard", 1.0, 1.0)?;
        assert_eq!(document.add_path(999, &[Point { x: 0.0, y: 0.0 }, Point { x: 1.0, y: 1.0 }], false), Err(ModelError::MissingArtboard));
        let artboard = document.artboards()[0].id;
        assert_eq!(document.add_path(artboard, &[Point { x: f64::NAN, y: 0.0 }, Point { x: 1.0, y: 1.0 }], false), Err(ModelError::InvalidPoint));
        Ok(())
    }

    #[test]
    fn configured_limits_are_enforced_atomically() -> Result<(), ModelError> {
        let limits = Limits { artboards: 1, paths: 1, points_per_path: 3 };
        let mut document = Document::with_limits("Doc", "Only", 10.0, 10.0, limits)?;
        assert_eq!(document.add_artboard("Too many", 1.0, 1.0), Err(ModelError::ArtboardLimit));
        let artboard = document.artboards()[0].id;
        let triangle = [Point { x: 0.0, y: 0.0 }, Point { x: 1.0, y: 0.0 }, Point { x: 0.0, y: 1.0 }];
        assert_eq!(document.add_path(artboard, &triangle, true), Ok(2));
        assert_eq!(document.add_path(artboard, &triangle, true), Err(ModelError::PathLimit));
        assert_eq!(document.paths().len(), 1);
        Ok(())
    }

    #[test]
    fn last_path_can_be_undone_and_restored_without_changing_its_id() -> Result<(), ModelError> {
        let mut document = Document::new("Doc", "Artboard", 100.0, 100.0)?;
        let artboard = document.artboards()[0].id;
        let points = [Point { x: 1.0, y: 1.0 }, Point { x: 9.0, y: 1.0 }, Point { x: 9.0, y: 9.0 }];
        let id = document.add_path(artboard, &points, true)?;
        let removed = document.remove_last_path().ok_or(ModelError::MissingArtboard)?;
        assert_eq!(removed.id, id);
        assert!(document.paths().is_empty());
        document.restore_path(removed)?;
        assert_eq!(document.paths()[0].id, id);
        Ok(())
    }

    #[test]
    fn vcf1_round_trips_the_native_document_subset() -> Result<(), ModelError> {
        let mut document = Document::new("Poster", "Board", 420.0, 500.0)?;
        let artboard = document.artboards()[0].id;
        document.add_path(artboard, &[Point { x: 1.0, y: 2.0 }, Point { x: 30.5, y: 4.25 }, Point { x: 5.0, y: 60.0 }], true)?;
        let mut bytes = [0_u8; 1024];
        let length = document.encode(&mut bytes)?;
        let decoded = Document::decode(&DecodeSource { bytes: &bytes, length })?;
        assert_eq!(decoded.name(), "Poster");
        assert_eq!(decoded.artboards()[0].name, "Board");
        assert_eq!((decoded.artboards()[0].width, decoded.artboards()[0].height), (420.0, 500.0));
        assert_eq!(decoded.paths()[0].points, document.paths()[0].points);
        assert!(decoded.paths()[0].closed);
        Ok(())
    }

    #[test]
    fn vcf1_rejects_truncation_and_trailing_data() -> Result<(), ModelError> {
        let document = Document::new("Doc", "Board", 1.0, 1.0)?;
        let mut bytes = [0_u8; 128];
        let length = document.encode(&mut bytes)?;
        assert_eq!(Document::decode(&DecodeSource { bytes: &bytes, length: length - 1 }), Err(ModelError::InvalidFormat(FormatStage::PathCount)));
        bytes[length] = 0;
        assert_eq!(Document::decode(&DecodeSource { bytes: &bytes, length: length + 1 }), Err(ModelError::InvalidFormat(FormatStage::TrailingData)));
        Ok(())
    }

    #[test]
    fn restore_rejects_paths_that_did_not_come_from_this_document() -> Result<(), ModelError> {
        let mut document = Document::new("Doc", "Artboard", 100.0, 100.0)?;
        let artboard = document.artboards()[0].id;
        let foreign = Path { id: 99, artboard_id: artboard, points: alloc::vec![Point { x: 1.0, y: 1.0 }, Point { x: 2.0, y: 2.0 }], closed: false };
        assert_eq!(document.restore_path(foreign), Err(ModelError::IdExhausted));
        assert!(document.paths().is_empty());
        Ok(())
    }
}
