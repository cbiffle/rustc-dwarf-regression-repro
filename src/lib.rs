use core::sync::atomic::{AtomicU32, Ordering};

static THE_STATIC: AtomicU32 = AtomicU32::new(0);

pub fn read() -> u32 {
    THE_STATIC.load(Ordering::Relaxed)
}

pub fn write(x: u32) {
    THE_STATIC.store(x, Ordering::Relaxed);
}

