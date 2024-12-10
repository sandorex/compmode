use super::PatternList;

pub const CARGO: PatternList = &[
    r#"(?<type>error|warning): (?<msg>.+)\n *--> *(?<file>.+):(?<line>\d+):(?<col>\d+)"#,
];

#[cfg(test)]
mod test {
    use regex::Regex;
    use super::CARGO as PATTERN;

    const ERR_MSG: &str = r#"Compiling compmode v0.1.0 (/home/sandorex/ws/compmode)
error: expected one of `!` or `::`, found `<eof>`
  --> src/groups.rs:33:1
   |
33 | hehe
   | ^^^^ expected one of `!` or `::`

error: could not compile `compmode` (bin "compmode") due to 1 previous error
"#;

    const WARN_MSG: &str = r#"warning: constant `x` should have an upper case name
  --> src/groups.rs:33:7
   |
33 | const x: u8 = 2;
   |       ^ help: convert the identifier to upper case (notice the capitalization): `X`
   |
   = note: `#[warn(non_upper_case_globals)]` on by default

warning: `compmode` (bin "compmode") generated 1 warning
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.39s
"#;

    #[test]
    fn pattern_cargo_err() {
        let re = Regex::new(PATTERN[0]);
        assert!(re.is_ok(), "Pattern failed to compile");
        let re = re.unwrap();

        let captures = re.captures(ERR_MSG);
        assert!(captures.is_some(), "Pattern failed to match");
        let captures = captures.unwrap();

        assert_eq!(
            captures.name("type").map(|x| x.as_str()),
            Some("error")
        );

        assert_eq!(
            captures.name("msg").map(|x| x.as_str()),
            Some("expected one of `!` or `::`, found `<eof>`")
        );

        assert_eq!(
            captures.name("file").map(|x| x.as_str()),
            Some("src/groups.rs")
        );

        assert_eq!(
            captures.name("line").map(|x| x.as_str()),
            Some("33")
        );

        assert_eq!(
            captures.name("col").map(|x| x.as_str()),
            Some("1")
        );
    }

    #[test]
    fn pattern_cargo_warn() {
        let re = Regex::new(PATTERN[0]);
        assert!(re.is_ok(), "Pattern failed to compile");

        let re = re.unwrap();

        let captures = re.captures(WARN_MSG);
        assert!(captures.is_some(), "Pattern failed to match");
        let captures = captures.unwrap();

        assert_eq!(
            captures.name("type").map(|x| x.as_str()),
            Some("warning")
        );

        assert_eq!(
            captures.name("msg").map(|x| x.as_str()),
            Some("constant `x` should have an upper case name")
        );

        assert_eq!(
            captures.name("file").map(|x| x.as_str()),
            Some("src/groups.rs")
        );

        assert_eq!(
            captures.name("line").map(|x| x.as_str()),
            Some("33")
        );

        assert_eq!(
            captures.name("col").map(|x| x.as_str()),
            Some("7")
        );
    }
}

