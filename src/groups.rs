#![allow(dead_code)]

// simple macro to condense definition of regex groups
macro_rules! define {
    (
        $all_name:ident;

        $(
            $name:ident = [ $($val:literal,)* ];
        )*
    ) => {
        $(
            pub const $name: &[&str] = &[$($val,)*];
        )*

        pub const $all_name: &[&[&str]] = &[$(&$name,)*];
    };
}

define! {
    RE_ALL;

    RE_CARGO = [
        r#"(?<type>error|warning): (?<msg>.+)\n *--> *(?<file>.+):(?<line>\d+):(?<col>\d+)"#,
    ];

    // basically match any file path
    RE_GENERIC = [
        r#""#,
    ];
}

//    Compiling compmode v0.1.0 (/home/sandorex/ws/compmode)
// error: expected one of `!` or `::`, found `<eof>`
//   --> src/groups.rs:33:1
//    |
// 33 | hehe
//    | ^^^^ expected one of `!` or `::`
//
// error: could not compile `compmode` (bin "compmode") due to 1 previous error

// warning: constant `x` should have an upper case name
//   --> src/groups.rs:33:7
//    |
// 33 | const x: u8 = 2;
//    |       ^ help: convert the identifier to upper case (notice the capitalization): `X`
//    |
//    = note: `#[warn(non_upper_case_globals)]` on by default
//
// warning: `compmode` (bin "compmode") generated 1 warning
//     Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.39s

