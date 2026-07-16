use codex_utils_absolute_path::AbsolutePathBuf;

/// Returns the path to the Codex DIY configuration directory.
///
/// This build resolves the configuration root to the directory that contains
/// the currently running executable.
pub fn find_codex_home() -> std::io::Result<AbsolutePathBuf> {
    let exe_path = std::env::current_exe()?;
    let exe_dir = exe_path
        .parent()
        .ok_or_else(|| std::io::Error::other("Could not determine executable directory"))?;
    AbsolutePathBuf::from_absolute_path(exe_dir.to_path_buf())
}

#[cfg(test)]
mod tests {
    use super::find_codex_home;
    use codex_utils_absolute_path::AbsolutePathBuf;
    use pretty_assertions::assert_eq;

    #[test]
    fn find_codex_home_uses_current_exe_directory() {
        let resolved = find_codex_home().expect("Codex DIY home");
        let expected = std::env::current_exe()
            .expect("current exe")
            .parent()
            .expect("exe dir")
            .to_path_buf();
        let expected = AbsolutePathBuf::from_absolute_path(expected).expect("absolute exe dir");
        assert_eq!(resolved, expected);
    }
}
