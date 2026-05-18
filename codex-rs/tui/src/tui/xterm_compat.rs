//! Helpers for xterm-compatible keyboard behavior in WSL/VS Code environments.

fn parse_bool_env(value: Option<&str>) -> Option<bool> {
    match value.map(str::trim) {
        Some("1") => Some(true),
        Some(value) if value.eq_ignore_ascii_case("true") => Some(true),
        Some(value) if value.eq_ignore_ascii_case("yes") => Some(true),
        Some("0") => Some(false),
        Some(value) if value.eq_ignore_ascii_case("false") => Some(false),
        Some(value) if value.eq_ignore_ascii_case("no") => Some(false),
        _ => None,
    }
}

pub(super) fn keyboard_enhancement_disabled_by_terminal(
    disable_env: Option<&str>,
    is_wsl: bool,
    is_vscode_terminal: bool,
) -> bool {
    if let Some(disabled) = parse_bool_env(disable_env) {
        return disabled;
    }

    // VS Code running a WSL shell can hide TERM_PROGRAM from the Linux process
    // environment, so `running_in_vscode_terminal` also probes the Windows-side
    // environment through WSL interop.
    is_wsl && is_vscode_terminal
}

pub(super) fn running_in_wsl() -> bool {
    #[cfg(target_os = "linux")]
    {
        crate::clipboard_paste::is_probably_wsl()
    }

    #[cfg(not(target_os = "linux"))]
    {
        false
    }
}

pub(super) fn running_in_vscode_terminal() -> bool {
    vscode_terminal_detected(
        std::env::var("TERM_PROGRAM").ok().as_deref(),
        windows_term_program().as_deref(),
    )
}

fn vscode_terminal_detected(
    linux_term_program: Option<&str>,
    windows_term_program: Option<&str>,
) -> bool {
    term_program_is_vscode(linux_term_program) || term_program_is_vscode(windows_term_program)
}

fn term_program_is_vscode(value: Option<&str>) -> bool {
    value.is_some_and(|value| value.eq_ignore_ascii_case("vscode"))
}

fn windows_term_program() -> Option<String> {
    #[cfg(target_os = "linux")]
    {
        static WINDOWS_TERM_PROGRAM: std::sync::OnceLock<Option<String>> =
            std::sync::OnceLock::new();
        WINDOWS_TERM_PROGRAM
            .get_or_init(read_windows_term_program)
            .clone()
    }

    #[cfg(not(target_os = "linux"))]
    {
        None
    }
}

#[cfg(target_os = "linux")]
fn read_windows_term_program() -> Option<String> {
    let output = std::process::Command::new("cmd.exe")
        .args(["/d", "/s", "/c", "set TERM_PROGRAM"])
        .stdin(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .output()
        .ok()?;

    if !output.status.success() {
        return None;
    }

    String::from_utf8_lossy(&output.stdout)
        .lines()
        .find_map(|line| {
            line.trim_end_matches('\r')
                .strip_prefix("TERM_PROGRAM=")
                .map(str::to_string)
        })
        .filter(|value| !value.trim().is_empty())
}

#[cfg(test)]
mod tests {
    use super::keyboard_enhancement_disabled_by_terminal;

    #[test]
    fn keyboard_enhancement_auto_disables_for_vscode_in_wsl() {
        assert!(keyboard_enhancement_disabled_by_terminal(
            /*disable_env*/ None, /*is_wsl*/ true, /*is_vscode_terminal*/ true
        ));
    }

    #[test]
    fn keyboard_enhancement_auto_disable_requires_wsl_and_vscode() {
        assert!(!keyboard_enhancement_disabled_by_terminal(
            /*disable_env*/ None, /*is_wsl*/ true, /*is_vscode_terminal*/ false
        ));
        assert!(!keyboard_enhancement_disabled_by_terminal(
            /*disable_env*/ None, /*is_wsl*/ false, /*is_vscode_terminal*/ true
        ));
    }

    #[test]
    fn keyboard_enhancement_env_override_takes_priority() {
        assert!(keyboard_enhancement_disabled_by_terminal(
            Some("1"),
            /*is_wsl*/ false,
            /*is_vscode_terminal*/ false
        ));
        assert!(!keyboard_enhancement_disabled_by_terminal(
            Some("0"),
            /*is_wsl*/ true,
            /*is_vscode_terminal*/ true
        ));
    }
}
