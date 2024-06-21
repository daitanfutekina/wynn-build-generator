/// Generates an implementation of `std::convert::TryFrom<T>` `and std::convert::Into<T>` for a given enum
/// 
/// Note this requires From<base_type> to already be implemented for the given enum.
/// 
/// The first argument represents the enum
/// <br>The second argument represents the underlying type of the enum (ie, the `#[repr=T]`) of the enum
/// <br>All following arguments represent the types you want to generate `TryFrom` and `Into` for
/// # Example
/// ```
/// // Given the enums::Skill enum with an underlying representation of `u8`, implements From<u16>, From<i16>, Into<u16>, Into<i16> for it.
/// enum_from_into!(enums::Skill, u8, u16, i16);
/// 
/// // Implements From<i16> for enums::Atrs enum
/// enum_from_into!(enums::Atrs, u8, i16);
/// ```
/// should use a proc_macro instead and `#[derive]` it, but procedural macros are confusing and im too lazy to make one.
#[macro_export]
macro_rules! enum_from_into(
    ($enm: ident, $base_type: ty, $($for_type: ty),*) => {
        impl std::convert::Into<$base_type> for $enm{
            fn into(self) -> $base_type {
                self as $base_type
            }
        }
        $(
            impl std::convert::TryFrom<$for_type> for $enm{
                type Error = TryIntoWynnEnumError<$for_type,$enm>;
                fn try_from(value: $for_type) -> Result<Self, Self::Error> {
                    match value.try_into(){
                        Ok(n) => match <Self as std::convert::TryFrom<$base_type>>::try_from(n) {Ok(r) => Ok(r), Err(_) => Err(TryIntoWynnEnumError{from: value, to: Self::default()})},
                        Err(_) => Err(TryIntoWynnEnumError{from: value, to: Self::default()})
                    }
                }
            }
            impl std::convert::Into<$for_type> for $enm{
                fn into(self) -> $for_type {
                    self as $for_type
                }
            }
        )*
    }
);

/// In rust, fn (ident: type) is not the same as fn (ident: &type).<br>
/// For example:
/// ```
/// fn foo(data: u32){}
/// foo(10); // this will compile fine
/// foo(&10); // this will error!
/// ```
/// This is because functions often require either ownership of `data` or a reference to `data`
/// 
/// But this can be quite annoying for simple functions where this difference does not matter. 
/// Rust's type inferencing may also allows the same function to work no matter whether ownership or references are given.
/// 
/// This macro creates two trait implementation of a given function for a given struct, 
/// allowing for the function to be called using either values or references. 
/// 
/// Use a `*` to specify the argument you want to allow references or values for. <br>
/// Note that only one argument (with a *) is allowed in the function. <br>
/// If you want the function to use `self`, then you must explicitly type self `fn foo(self: &mut Self, *data: u32)`
/// # Example
/// ```
/// struct Bar{}
/// ref_irrelevent_struct_func!(Bar, FooTrait,
///     fn foo(*a: u32, b: u32) -> u32{
///         a+b
///     }
/// )
/// Bar::foo(5,10); // compiles fine
/// Bar::foo(&5,10); // also works!
/// Bar::foo(5,&10); // this will not work.
/// ```
#[macro_export]
macro_rules! ref_irrelevent_struct_func(
    ($struc: ident, $tvis: vis $trai: ident, $(#[$($docs: meta)*])* fn $func_name: tt($($func_idents1: ident: $func_types1: ty,)* *$fident: ident: &[$ref_type: ty] $(,$func_idents2: ident: $func_types2: ty)*) $(-> $res_ty: ty)? $func: block) => {
        $tvis trait $trai<T>{
            $(#[$($docs)*])*
            fn $func_name($($func_idents1: $func_types1,)* $fident: &[T], $($func_idents2: $func_types2,)*)$(-> $res_ty)?;
        }
        impl $trai<$ref_type> for $struc{
            fn $func_name($($func_idents1: $func_types1,)* $fident: &[$ref_type], $($func_idents2: $func_types2,)*)$(-> $res_ty)?{
                $func
            }
        }
        impl $trai<&$ref_type> for $struc{
            fn $func_name($($func_idents1: $func_types1,)* $fident: &[&$ref_type], $($func_idents2: $func_types2,)*)$(-> $res_ty)?{
                $func
            }
        }
    };
    ($struc: ident, $tvis: vis $trai: ident, fn $func_name: tt($($func_idents1: ident: $func_types1: ty,)* *$fident: ident: $ref_type: ty $(,$func_idents2: ident: $func_types2: ty)*) $(-> $res_ty: ty)? $func: block) => {
        $tvis trait $trai<T>{
            fn $func_name($($func_idents1: $func_types1,)* $fident: T, $($func_idents2: $func_types2,)*)$(-> $res_ty)?;
        }
        impl $trai<$ref_type> for $struc{
            fn $func_name($($func_idents1: $func_types1,)* $fident: $ref_type, $($func_idents2: $func_types2,)*)$(-> $res_ty)?{
                $func
            }
        }
        impl $trai<&$ref_type> for $struc{
            fn $func_name($($func_idents1: $func_types1,)* $fident: &$ref_type, $($func_idents2: $func_types2,)*)$(-> $res_ty)?{
                $func
            }
        }
    }
);