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
#[derive(Clone, Copy, PartialEq, PartialOrd, Debug)]
pub struct Integer(f64);

const INTEGER_SAFE_MAX: f64 = 9007199254740991.0;
const INTEGER_SAFE_MIN: f64 = -9007199254740991.0;

impl Integer {
    /// Lossy converts f64 into integer value.
    ///
    /// - value out of safe-integer range converted into positive/negative infinity.
    /// - value is truncated (loses its fractional part).
    /// - keeps NaN.
    /// 
    /// ```
    /// # use wasm_actions_node_sys::Integer;
    /// assert_eq!(Integer::from_f64_lossy(f64::INFINITY), Integer::INFINITY);
    /// assert_eq!(Integer::from_f64_lossy(9007199254740992.0), Integer::INFINITY);
    /// assert_eq!(Integer::from_f64_lossy(9007199254740991.0), Integer::SAFE_MAX);
    /// assert_eq!(Integer::from_f64_lossy(0.5), Integer::from(0));
    /// assert_eq!(Integer::from_f64_lossy(-9007199254740991.0), Integer::SAFE_MIN);
    /// assert_eq!(Integer::from_f64_lossy(-9007199254740992.0), Integer::NEG_INFINITY);
    /// assert_eq!(Integer::from_f64_lossy(f64::NEG_INFINITY), Integer::NEG_INFINITY);
    /// assert!(Integer::from_f64_lossy(f64::NAN).is_nan());
    /// ```
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

    /// Clamps f64 into integer value.
    ///
    /// - value out of safe-integer range clamped to SAFE_MAX or SAFE_MIN.
    /// - NaN will be zero.
    /// - value is truncated (loses its fractional part).
    /// 
    /// ```
    /// # use wasm_actions_node_sys::Integer;
    /// assert_eq!(Integer::from_f64_clamping(f64::INFINITY), Integer::SAFE_MAX);
    /// assert_eq!(Integer::from_f64_clamping(9007199254740992.0), Integer::SAFE_MAX);
    /// assert_eq!(Integer::from_f64_clamping(-9007199254740992.0), Integer::SAFE_MIN);
    /// assert_eq!(Integer::from_f64_clamping(f64::NEG_INFINITY), Integer::SAFE_MIN);
    /// assert_eq!(Integer::from_f64_clamping(f64::NAN), Integer::ZERO);
    /// ```
    pub fn from_f64_clamping(f: f64) -> Self {
        if f.is_nan() {
            Self(0.0)
        } else if f < INTEGER_SAFE_MIN {
            Self(INTEGER_SAFE_MIN)
        } else if f > INTEGER_SAFE_MAX {
            Self(INTEGER_SAFE_MAX)
        } else {
            Self(f.trunc())
        }
    }

    /// Checks if the Integer is NaN.
    /// 
    /// ```
    /// # use wasm_actions_node_sys::Integer;
    /// assert!(Integer::from_f64_lossy(f64::NAN).is_nan());
    /// ```
    pub fn is_nan(self) -> bool {
        self.0.is_nan()
    }

    /// Checks if the Integer is finite (means within safe range).
    /// 
    /// ```
    /// # use wasm_actions_node_sys::Integer;
    /// assert!(Integer::from_f64_lossy(f64::INFINITY).is_infinite());
    /// assert!(Integer::from_f64_lossy(-9007199254740992.0).is_infinite());
    /// ```
    pub fn is_infinite(self) -> bool {
        self.0.is_infinite()
    }

    /// Gets the sign number of the Integer.
    /// 
    /// ```
    /// # use wasm_actions_node_sys::Integer;
    /// assert!(Integer::from(-123).signum() < 0.0);
    /// ```
    pub fn signum(self) -> f64 {
        self.0.signum()
    }

    /// Zero.
    pub const ZERO: Self = Self(0.0);
    /// NaN.
    pub const NAN: Self = Self(f64::NAN);
    /// Positive infinity.
    pub const INFINITY: Self = Self(f64::INFINITY);
    /// Negative infinity.
    pub const NEG_INFINITY: Self = Self(f64::NEG_INFINITY);
    /// Maximum value within safe range (2^53 - 1).
    pub const SAFE_MAX: Self = Self(INTEGER_SAFE_MAX);
    /// Minimum value within safe range -(2^53 - 1).
    pub const SAFE_MIN: Self = Self(INTEGER_SAFE_MIN);
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

safe_integers!(i32, u32, i16, u16, i8, u8);

impl From<Integer> for f64 {
    fn from(val: Integer) -> Self {
        val.0
    }
}

impl TryFrom<Integer> for usize {
    type Error = &'static str;

    fn try_from(value: Integer) -> Result<Self, Self::Error> {
        if value.is_nan() {
            Err("cannot cast NaN into usize")
        } else if value.is_infinite() {
            Err("cannot cast non-finite value into usize")
        } else if value.signum() < 0.0 {
            Err("cannot cast negative value into usize")
        } else {
            let value: f64 = value.into();
            Ok(value as usize)
        }
    }
}
