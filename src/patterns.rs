#![allow(dead_code)]

pub type Pattern = (&'static str, &'static [&'static str]);

/// Defines new pattern const
macro_rules! define {
    (
        $(
            $name:ident = [ $($val:literal,)* ] ;
        )*
    ) => {
        $(
            pub const $name: $crate::patterns::Pattern = (stringify!($name), &[ $( $val, )* ]);
        )*
    }
}

pub mod cargo;

pub const GROUPS: &[&Pattern] = &[
    &cargo::PATTERN,
];

