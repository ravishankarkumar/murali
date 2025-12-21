//! LRU cache for compiled text rasters.

use std::sync::Arc;
use parking_lot::RwLock;
use lru::LruCache;
use std::num::NonZeroUsize;

/// Raster + metadata stored in the cache.
#[derive(Debug, Clone)]
pub struct TypstRaster {
    pub rgba: Vec<u8>,
    pub width: u32,
    pub height: u32,
    pub svg: Option<String>,
}

#[derive(Debug)]
pub struct TypstRasterCache {
    inner: RwLock<LruCache<String, Arc<TypstRaster>>>,
}

impl TypstRasterCache {
    pub fn new(capacity: usize) -> Self {
        Self {
            inner: RwLock::new(LruCache::new(NonZeroUsize::new(capacity).expect("Cache capacity must be non-zero"))),
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
