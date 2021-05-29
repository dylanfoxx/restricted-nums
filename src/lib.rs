   

// We use paste to concat idents. There may be a better way to do this
use paste::paste;

use std::convert::TryFrom;
macro_rules! auto_struct_try_from {
    ($cur:ident, $($t:ident)*) => {
        paste! { $(
            impl<const MIN: $cur, const MAX: $cur> TryFrom<$t> for [<RNum $cur>]<MIN, MAX> {
                type Error = &'static str;
    
                #[inline]
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
        fn [<RNum $cur _fail_from>]() {
                
        }}
    };
}
    
use std::cmp::Ordering;
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
        fn [<RNum $cur _partial_cmp>]() {
                
        }}
    };
}



use const_fn_assert::cfn_assert;
macro_rules! auto_struct {
    ($($t:ident)*) => {
        paste! { $(
            #[derive(Clone)]
            pub struct [<RNum $t>]<const MIN: $t, const MAX: $t> {
                value: $t,
            }
            
            impl<const MIN: $t, const MAX: $t> [<RNum $t>]<MIN, MAX>{

                const fn in_range(value: $t) -> bool{
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

                pub const unsafe fn new_uncheck(value: $t) ->Self {
                    cfn_assert!(MIN < MAX);
                    Self{value}
                }

                pub const fn new_min() -> Self{
                    Self{value: MIN}
                }
            
                pub const fn new_max() -> Self{
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

            auto_struct_try_from! { $t, usize u8 u16 u32 u64 u128 isize i8 i16 i32 i64 i128 }
            auto_struct_partial_cmp! { $t, usize u8 u16 u32 u64 u128 isize i8 i16 i32 i64 i128 }
            
        )*}
    };
}

// Can only be run with whole numeric types
auto_struct! { usize u8 u16 u32 u64 u128 isize i8 i16 i32 i64 i128 }


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let name = super::RNumu8::<13, 55>::new();
        let other: u8 = name.clone().into();
        let other2: u8 = 13;
        if name == other2 {
            println!("it is {}", other);
        }
        // assert_eq!(2 + 2, 4);
    }
}
