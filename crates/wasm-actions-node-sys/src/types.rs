use core::f64;

use wasm_bindgen::prelude::wasm_bindgen;

/// Integer type
/// 
/// Primary usage is exporting Rust value into Node world.
/// For most cases, it is prefered using i32 or u32 for integer parameter.
/// Some Node library function, however, accept safe-integer and infinite as integer.
/// Integer covers such cases.
#[repr(C)]
#[wasm_bindgen]
#[derive(Clone, Copy)]
pub struct Integer(f64);

const INTEGER_SAFE_MAX: f64 = 9007199254740991.0;
const INTEGER_SAFE_MIN: f64 = -9007199254740991.0;

impl Integer {
    /// Lossy converts f64 into integer value.
    /// 
    /// - value out of safe-integer range converted into positive/negative infinity.
    /// - value is truncated (loses its fractional part).
    pub fn from_f64_lossy(f: f64) -> Self {
        if f.is_nan() {
            Self(f)
        } else if f < INTEGER_SAFE_MIN {
            Self(f64::NEG_INFINITY)
        } else if f > INTEGER_SAFE_MAX {
            Self(f64::INFINITY)
        } else {
            Self(f.trunc())
        }
    }

    pub fn is_nan(self) -> bool {
        self.0.is_nan()
    }

    pub fn is_infinite(self) -> bool {
        self.0.is_infinite()
    }

    pub fn signum(self) -> f64 {
        self.0.signum()
    }
}

impl Default for Integer {
    /// Default value is 0.
    fn default() -> Self {
        Self(0.0)
    }
}

macro_rules! safe_integers {
    ($t:ident) => {
        impl From< $t > for Integer {
            fn from(value: $t) -> Self {
                Self(value.into())
            }
        }
    };
    ($t:ident, $($tx:ident),*) => {
        safe_integers!($t);
        safe_integers!($($tx),*);
    };
}

safe_integers!(
    i32, u32, i16, u16, i8, u8
);

impl Into<f64> for Integer {
    fn into(self) -> f64 {
        self.0
    }
}