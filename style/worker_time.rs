//! Target-safe monotonic timing for style traversal diagnostics.

use std::ops::Sub;
use std::time::Duration;

#[cfg(not(target_arch = "wasm32"))]
pub(crate) use std::time::Instant;

#[cfg(target_arch = "wasm32")]
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub(crate) struct Instant(u64);

#[cfg(target_arch = "wasm32")]
impl Instant {
    pub(crate) fn now() -> Self {
        #[link(wasm_import_module = "env")]
        unsafe extern "C" {
            #[link_name = "worker_monotonic_now_ns"]
            fn worker_monotonic_now_ns() -> u64;
        }

        Self(unsafe { worker_monotonic_now_ns() })
    }

    pub(crate) fn duration_since(self, earlier: Self) -> Duration {
        Duration::from_nanos(self.0.saturating_sub(earlier.0))
    }
}

#[cfg(target_arch = "wasm32")]
impl Sub for Instant {
    type Output = Duration;

    fn sub(self, rhs: Self) -> Self::Output {
        self.duration_since(rhs)
    }
}
