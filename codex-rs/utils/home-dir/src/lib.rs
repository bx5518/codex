use codex_utils_absolute_path::AbsolutePathBuf;
use dirs::home_dir;

/// Returns the path to the Codex DIY configuration directory.
///
/// This build intentionally uses only `~/.codexdiy` so it can coexist with an
/// official Codex installation without reading or writing the official
/// `~/.codex` directory.
pub fn find_codex_home() -> std::io::Result<AbsolutePathBuf> {
    let mut path = home_dir().ok_or_else(|| {
        std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "Could not find home directory",
        )
    })?;
    path.push(".codexdiy");
    AbsolutePathBuf::from_absolute_path(path)
}

#[cfg(test)]
mod tests {
    use super::find_codex_home;
    use codex_utils_absolute_path::AbsolutePathBuf;
    use dirs::home_dir;
    use pretty_assertions::assert_eq;

    #[test]
    fn find_codex_home_uses_diy_home_dir() {
        let resolved = find_codex_home().expect("Codex DIY home");
        let mut expected = home_dir().expect("home dir");
        expected.push(".codexdiy");
        let expected = AbsolutePathBuf::from_absolute_path(expected).expect("absolute home");
        assert_eq!(resolved, expected);
    }
}
