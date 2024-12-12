use super::PatternList;

pub const GCC: PatternList = &[
    r#"^(?<file>.+):(?<line>\d+):(?<col>\d+): (?<type>error|warning): (?<msg>[^\n]+)"#,
];

#[cfg(test)]
mod test {
    use regex::Regex;
    use super::GCC as PATTERN;

    const ERR_MSG: &str = r#"main.c:4:29: error: expected ‘;’ before ‘return’
    4 |     printf("Hello World!\n")
      |                             ^
      |                             ;
    5 |     return 1;
      |     ~~~~~~
      "#;

    const WARN_MSG: &str = r#"main.c:7:13: warning: format ‘%s’ expects argument of type ‘char *’, but argument 2 has type ‘int’ [-Wformat=]
    7 |     printf (fmt, 123);
      |             ^~~  ~~~
      |                  |
      |                  int"#;

    #[test]
    fn pattern_gcc_err() {
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
            Some("expected ‘;’ before ‘return’")
        );

        assert_eq!(
            captures.name("file").map(|x| x.as_str()),
            Some("main.c")
        );

        assert_eq!(
            captures.name("line").map(|x| x.as_str()),
            Some("4")
        );

        assert_eq!(
            captures.name("col").map(|x| x.as_str()),
            Some("29")
        );
    }

    #[test]
    fn pattern_gcc_warn() {
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
            Some("format ‘%s’ expects argument of type ‘char *’, but argument 2 has type ‘int’ [-Wformat=]")
        );

        assert_eq!(
            captures.name("file").map(|x| x.as_str()),
            Some("main.c")
        );

        assert_eq!(
            captures.name("line").map(|x| x.as_str()),
            Some("7")
        );

        assert_eq!(
            captures.name("col").map(|x| x.as_str()),
            Some("13")
        );
    }
}
