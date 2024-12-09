#![allow(dead_code)]

type Pattern = (&'static str, &'static [&'static str]);

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

/// Module for common test functionality
#[cfg(test)]
#[allow(dead_code, unused_imports)]
pub mod tests_prelude {
    pub use regex::{Captures, Match, Regex};

    // TODO maybe replace this with a generic or macro?
    pub fn get_group<'a>(captures: &'a Captures, group: &str) -> Match<'a> {
        let g = captures.name(group);
        assert!(g.is_some(), "Pattern failed to match {:?} group", group);
        g.unwrap()
    }
}

pub mod cargo;

pub const GROUPS: &[&Pattern] = &[
    &cargo::PATTERN,
];

