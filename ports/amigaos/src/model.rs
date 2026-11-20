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
    OutOfMemory,
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
        let name = copy_name(name)?;
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

    pub fn add_artboard(&mut self, name: &str, width: f64, height: f64) -> Result<u64, ModelError> {
        if self.artboards.len() >= self.limits.artboards {
            return Err(ModelError::ArtboardLimit);
        }
        validate_extent(width, height)?;
        let name = copy_name(name)?;
        self.artboards.try_reserve(1).map_err(|_| ModelError::OutOfMemory)?;
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
        owned.try_reserve_exact(points.len()).map_err(|_| ModelError::OutOfMemory)?;
        owned.extend_from_slice(points);
        self.paths.try_reserve(1).map_err(|_| ModelError::OutOfMemory)?;
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

fn copy_name(name: &str) -> Result<String, ModelError> {
    if name.is_empty() {
        return Err(ModelError::EmptyName);
    }
    if name.len() > MAX_NAME_BYTES {
        return Err(ModelError::NameTooLong);
    }
    let mut owned = String::new();
    owned.try_reserve_exact(name.len()).map_err(|_| ModelError::OutOfMemory)?;
    owned.push_str(name);
    Ok(owned)
}

fn validate_extent(width: f64, height: f64) -> Result<(), ModelError> {
    if width.is_finite() && height.is_finite() && width > 0.0 && height > 0.0 { Ok(()) } else { Err(ModelError::InvalidExtent) }
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
}
