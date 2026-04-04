// src/resources/typst/cache.rs
//! LRU cache for compiled text rasters.

use lru::LruCache;
use parking_lot::RwLock;
use std::num::NonZeroUsize;
use std::sync::Arc;

/// Raster + metadata stored in the cache.
#[derive(Debug, Clone)]
pub struct TypstRaster {
    pub rgba: Vec<u8>,
    pub width: u32,
    pub height: u32,
    pub normalized_height_px: f32,
    pub svg: Option<String>,
}

#[derive(Debug)]
pub struct TypstRasterCache {
    inner: RwLock<LruCache<String, Arc<TypstRaster>>>,
}

impl TypstRasterCache {
    pub fn new(capacity: usize) -> Self {
        Self {
            inner: RwLock::new(LruCache::new(
                NonZeroUsize::new(capacity).expect("Cache capacity must be non-zero"),
            )),
        }
    }

    pub fn get(&self, key: &str) -> Option<Arc<TypstRaster>> {
        let mut guard = self.inner.write();
        guard.get(key).cloned()
    }

    pub fn insert(&self, key: String, raster: TypstRaster) {
        let mut guard = self.inner.write();
        guard.put(key, Arc::new(raster));
    }
}
