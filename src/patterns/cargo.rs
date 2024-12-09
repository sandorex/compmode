define! {
    PATTERN = [
        r#"(?<type>error|warning): (?<msg>.+)\n *--> *(?<file>.+):(?<line>\d+):(?<col>\d+)"#,
    ];
}

#[cfg(test)]
mod test {
    use super::super::tests_prelude::*;
    use super::PATTERN;

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
        let re = Regex::new(PATTERN.1[0]);
        assert!(re.is_ok(), "Pattern failed to compile");

        let re = re.unwrap();

        let captures = re.captures(ERR_MSG);
        assert!(captures.is_some(), "Pattern failed to match");

        let captures = captures.unwrap();

        let r#type = get_group(&captures, "type");
        assert_eq!(r#type.as_str(), "error");

        let msg = get_group(&captures, "msg");
        assert_eq!(msg.as_str(), "expected one of `!` or `::`, found `<eof>`");

        let file = get_group(&captures, "file");
        assert_eq!(file.as_str(), "src/groups.rs");

        let line = get_group(&captures, "line");
        assert_eq!(line.as_str(), "33");

        let column = get_group(&captures, "col");
        assert_eq!(column.as_str(), "1");
    }

    #[test]
    fn pattern_cargo_warn() {
        let re = Regex::new(PATTERN.1[0]);
        assert!(re.is_ok(), "Pattern failed to compile");

        let re = re.unwrap();

        let captures = re.captures(WARN_MSG);
        assert!(captures.is_some(), "Pattern failed to match");
        let captures = captures.unwrap();

        let r#type = get_group(&captures, "type");
        assert_eq!(r#type.as_str(), "warning");

        let msg = get_group(&captures, "msg");
        assert_eq!(msg.as_str(), "constant `x` should have an upper case name");

        let file = get_group(&captures, "file");
        assert_eq!(file.as_str(), "src/groups.rs");

        let line = get_group(&captures, "line");
        assert_eq!(line.as_str(), "33");

        let column = get_group(&captures, "col");
        assert_eq!(column.as_str(), "7");
    }
}

