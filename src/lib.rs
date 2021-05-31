#![no_std]

// Things to add.
// - Checking for over or underflow in operations Ex: <_, i8::MAX>::new_max() + 1; I think is currently in range but should not be ... maybe?
// - Documentation
// - Fill out tests
// - Idents may be weird, is there a way to do this with types
// - Add serde feature

//! restricted-nums contains all core num types(not floats) but in restricted form. Meaning they can be limited by const values on deceleration.
//!
//! Due to this most operations return Option<Self>, with only a few that are guaranteed to succeed returning Self.
//! ```
//!     use restricted_nums::{RNumi8, RNumi32};
//!
//!     let my_num = RNumi8::<1, 20>::new_min(); //Is 1
//!     let complicated_num: i32 = -5;
//!     let new_value = my_num + complicated_num;
//!     assert!(new_value == None);
//!
//!     fn calc(value : impl AsRef<i8>){
//!         print!("{}", value.as_ref());
//!     }
//!
//!     calc(my_num);
//! ```
//! Most useful traits have been implemented on each type.
//! Each trait should be what you think but double check the documentation of each function to confirm. Ex: 

// We use paste to concat idents. There may be a better way to do this.
use paste::paste;

// May be worth changing the Error type here
use core::convert::TryFrom;
macro_rules! auto_struct_try_from {
    ($cur:ident, $($t:ident)*) => {
        paste! { $(
            impl<const MIN: $cur, const MAX: $cur> TryFrom<$t> for [<RNum $cur>]<MIN, MAX> {
                type Error = &'static str;

                fn try_from(value: $t) -> Result<Self, Self::Error> {
                    if let Ok(new_val) = $cur::try_from(value) {
                        if Self::in_range(new_val){
                            return Ok(Self{value: new_val})
                        }
                    }
                    Err("value Out of range")
                }
            }
        )*

        #[test]
        fn [<rnum $cur _fail_from>]() {
            let value = [<RNum $cur>]::<1, 120>::try_from(240 + 1);
            assert!(value.ok() == None);
            let value = [<RNum $cur>]::<1, 120>::try_from(0);
            assert!(value.ok() == None);
            let value = [<RNum $cur>]::<1, 120>::try_from(7);
            assert!(value.ok().is_some());
        }}
    };
}

use core::cmp::Ordering;
macro_rules! auto_struct_partial_cmp {
    ($cur:ident, $($t:ident)*) => {
        paste! { $(
            impl<const MIN: $cur, const MAX: $cur> PartialEq<$t> for [<RNum $cur>]<MIN, MAX> {
                fn eq(&self, other: &$t) -> bool {
                    // If its not in the same range it cannot be equal
                    if let Ok(other_to_us) = Self::try_from(*other){
                        return self.value.eq(&other_to_us.value);
                    }

                    false
                }
            }

            impl<const MIN: $cur, const MAX: $cur> PartialOrd<$t> for [<RNum $cur>]<MIN, MAX> {
                fn partial_cmp(&self, other: &$t) -> Option<Ordering> {
                    // Its not clear what to do in this case, do we return None if other is out of range?
                    if let Ok(other_to_us) = Self::try_from(*other){
                        return Some(self.value.cmp(&other_to_us.value));
                    }
                    // For now lets do that
                    None
                }
            }
        )*

        #[test]
        fn [<rnum $cur _partial_cmp>]() {

        }}
    };
}

use core::ops::Add;
macro_rules! auto_struct_add {
    ($cur:ident, $($t:ident)*) => {
        paste! { $(
            impl<const MIN: $cur, const MAX: $cur> Add<$t> for [<RNum $cur>]<MIN, MAX> {
                type Output = Option<Self>;

                fn add(self, other: $t) -> Self::Output {
                    if let Ok(new_other) = Self::try_from(other){
                        // Change this to an add i can check
                        let new = self.value + new_other.value;
                        return Self::try_from(new).ok();
                    }
                    None
                }
            }
        )*

        #[test]
        fn [<rnum $cur _add>]() {

        }}
    };
}

use core::ops::Mul;
macro_rules! auto_struct_mul {
    ($cur:ident, $($t:ident)*) => {
        paste! { $(
            impl<const MIN: $cur, const MAX: $cur> Mul<$t> for [<RNum $cur>]<MIN, MAX> {
                type Output = Option<Self>;

                fn mul(self, other: $t) -> Self::Output {
                    if let Ok(new_other) = Self::try_from(other){
                        // Change this to an add i can check
                        let new = self.value * new_other.value;
                        return Self::try_from(new).ok();
                    }
                    None
                }
            }
        )*

        #[test]
        fn [<rnum $cur _mul>]() {

        }}
    };
}

use core::ops::Div;
macro_rules! auto_struct_div {
    ($cur:ident, $($t:ident)*) => {
        paste! { $(
            impl<const MIN: $cur, const MAX: $cur> Div<$t> for [<RNum $cur>]<MIN, MAX> {
                type Output = Option<Self>;

                fn div(self, other: $t) -> Self::Output {
                    if let Ok(new_other) = Self::try_from(other){
                        // Change this to an add i can check
                        let new = self.value / new_other.value;
                        return Self::try_from(new).ok();
                    }
                    None
                }
            }
        )*

        #[test]
        fn [<rnum $cur _div>]() {

        }}
    };
}

use core::convert::AsRef;
use const_fn_assert::cfn_assert;
use core::ops::Deref;
use core::fmt::{self, Display};
macro_rules! auto_struct {
    ($($t:ident)*) => {
        paste! { $(
            /// A restricted integer value from [MIN, MAX]
            #[repr(transparent)]
            #[derive(Clone, Copy, Debug)]
            pub struct [<RNum $t>]<const MIN: $t, const MAX: $t> {
                value: $t,
            }

            impl<const MIN: $t, const MAX: $t> [<RNum $t>]<MIN, MAX>{

                const fn in_range(value: $t) -> bool{
                    cfn_assert!(MIN < MAX);
                    value >= MIN && value <= MAX
                }

                pub const fn new(value: $t) -> Option<Self>{
                    cfn_assert!(MIN < MAX);
                    if Self::in_range(value){
                        Some(Self{value})
                    }else {
                        None
                    }
                }

                ///# Safety
                /// This will not check to make sure your value is within [MIN, MAX]
                pub const unsafe fn new_uncheck(value: $t) ->Self {
                    cfn_assert!(MIN < MAX);
                    Self{value}
                }

                pub const fn new_min() -> Self{
                    cfn_assert!(MIN < MAX);
                    Self{value: MIN}
                }

                pub const fn new_max() -> Self{
                    cfn_assert!(MIN < MAX);
                    Self{value: MIN}
                }
            }

            impl<const MIN: $t, const MAX: $t> From<[<RNum $t>]<MIN, MAX>> for $t {
                fn from(val: [<RNum $t>]<MIN, MAX>) -> Self {
                    val.value
                }
            }

            impl<const MIN: $t, const MAX: $t> PartialEq for [<RNum $t>]<MIN, MAX> {
                fn eq(&self, other: &Self) -> bool {
                    self.value == other.value
                }
            }

            impl<const MIN: $t, const MAX: $t> PartialOrd for [<RNum $t>]<MIN, MAX> {
                fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
                    Some(self.value.cmp(&other.value))
                }
            }

            impl<const MIN: $t, const MAX: $t> Deref for [<RNum $t>]<MIN, MAX> {
                type Target = $t;

                #[inline]
                fn deref(&self) -> &Self::Target {
                    &self.value
                }
            }

            impl<const MIN: $t, const MAX: $t> Display for [<RNum $t>]<MIN, MAX> {
                fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                    write!(f, "{}", self.value)
                }
            }

            impl<const MIN: $t, const MAX: $t> Add for [<RNum $t>]<MIN, MAX> {
                type Output = Option<Self>;

                fn add(self, other: Self) -> Self::Output {
                    let new = self.value + other.value;
                    Self::try_from(new).ok()
                }
            }

            impl<const MIN: $t, const MAX: $t> Mul for [<RNum $t>]<MIN, MAX> {
                type Output = Option<Self>;

                fn mul(self, other: Self) -> Self::Output {
                    let new = self.value * other.value;
                    Self::try_from(new).ok()
                }
            }
            
            impl<const MIN: $t, const MAX: $t> Div for [<RNum $t>]<MIN, MAX> {
                type Output = Option<Self>;

                fn div(self, other: Self) -> Self::Output {
                    let new = self.value / other.value;
                    Self::try_from(new).ok()
                }
            }

            impl<const MIN: $t, const MAX: $t> AsRef<$t> for [<RNum $t>]<MIN, MAX> {

                #[inline]
                fn as_ref(&self) -> &$t {
                    &self.value
                }
            }

            auto_struct_try_from! { $t, usize u8 u16 u32 u64 u128 isize i8 i16 i32 i64 i128 }
            auto_struct_partial_cmp! { $t, usize u8 u16 u32 u64 u128 isize i8 i16 i32 i64 i128 }
            auto_struct_add! { $t, usize u8 u16 u32 u64 u128 isize i8 i16 i32 i64 i128 }
            auto_struct_div! { $t, usize u8 u16 u32 u64 u128 isize i8 i16 i32 i64 i128 }
            auto_struct_mul! { $t, usize u8 u16 u32 u64 u128 isize i8 i16 i32 i64 i128 }

        )*}
    };
}

// Can only be run with whole numeric types(NO FLOAT AT THIS POINT)
auto_struct! { usize u8 u16 u32 u64 u128 isize i8 i16 i32 i64 i128 }

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let val = super::RNumu8::<13, 55>::new(12);
        assert!(val == None);
        let val = unsafe {super::RNumi16::<20, 500>::new_uncheck(25)};
        let val1: i16 = val.into();
        assert!(val1 == 25);
        let val2 = val + val1;
        assert!(val2.is_some());
        assert!(val2.unwrap() == 50);
    }
}
