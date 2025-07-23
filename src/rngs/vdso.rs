// Copyright 2025 Developers of the Rand project.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Linux vDSO interface

use core::fmt;
use rand_core::TryRngCore;
use vdso_rng::{LocalState, Pool};

fn global_pool() -> &'static Pool {
    static GLOBAL_STATE: std::sync::LazyLock<Pool> =
        std::sync::LazyLock::new(|| Pool::new().expect("Failed to create global pool"));
    &GLOBAL_STATE
}

/// A vDSO handle
pub struct VdsoRng {
    state: vdso_rng::LocalState<'static>,
}

/// Debug implementation does not leak internal state
impl fmt::Debug for VdsoRng {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "VdsoRng {{ .. }}")
    }
}

impl TryRngCore for VdsoRng {
    type Error = vdso_rng::Error;

    fn try_next_u32(&mut self) -> Result<u32, Self::Error> {
        let mut buf = [0; 4];
        self.try_fill_bytes(&mut buf)?;
        Ok(u32::from_le_bytes(buf))
    }

    fn try_next_u64(&mut self) -> Result<u64, Self::Error> {
        let mut buf = [0; 8];
        self.try_fill_bytes(&mut buf)?;
        Ok(u64::from_le_bytes(buf))
    }

    fn try_fill_bytes(&mut self, dst: &mut [u8]) -> Result<(), Self::Error> {
        self.state.fill(dst, 0)
    }
}

impl VdsoRng {
    /// Construct a [`VdsoRng`]
    pub fn new() -> Result<Self, vdso_rng::Error> {
        Ok(VdsoRng {
            state: LocalState::new(global_pool())?,
        })
    }
}
