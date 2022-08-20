#[cfg(test)]
mod test {
    use globset::*;

    // This matches all absolute paths
    #[test]
    fn test_slash_star() {
        let glob = Glob::new("/*").unwrap().compile_matcher();

        assert!(glob.is_match("/hello"));
        assert!(!glob.is_match("./hello"));
        assert!(!glob.is_match("hello"));
    }

    // This matches all relative paths
    #[test]
    fn test_up_slash_star() {
        let glob = Glob::new("../*").unwrap().compile_matcher();

        assert!(glob.is_match("../hello"));
        assert!(!glob.is_match("/hello"));
        assert!(!glob.is_match("./hello"));
        assert!(!glob.is_match("hello"));
    }
}
