use std::env;
use std::process::Command;

pub(crate) fn is_tmux() -> bool {
    env::var("TMUX").is_ok()
}

pub(crate) fn has_passthrough() -> bool {
    if !is_tmux() {
        return false;
    }
    let output = Command::new("tmux")
        .arg("show")
        .arg("-Apv")
        .arg("allow-passthrough")
        .output()
        .expect("failed to execute tmux command");
    let result = String::from_utf8_lossy(&output.stdout);
    result.trim_end() == "on"
}

pub(crate) fn create_dm_getter(name: &str) -> impl Fn() -> Option<String> + '_ + '_ {
    move || {
        if !is_tmux() {
            return None;
        }
        let output = Command::new("tmux")
            .arg("display-message")
            .arg("-p")
            .arg(&format!("#{{{}}}", name))
            .output()
            .expect("failed to execute tmux command");
        let result = String::from_utf8_lossy(&output.stdout);
        if result.is_empty() {
            None
        } else {
            Some(result.trim().to_string())
        }
    }
}

pub(crate) fn escape(sequence: &str) -> String {
    let escaped_sequence = sequence.replace('\x1b', "\x1b\x1b");
    format!("\x1bPtmux;{}\x1b\\", escaped_sequence)
}

// Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_tmux() {
        // Set the TMUX environment variable for testing
        env::set_var("TMUX", "tmux_value");
        assert!(is_tmux());

        // Unset the TMUX environment variable
        env::remove_var("TMUX");
        assert!(!is_tmux());
    }

    #[test]
    fn test_has_passthrough() {
        // Mock the tmux command output for testing
        let mock_output = "off";
        let mut cmd = Command::new("echo");
        cmd.arg(mock_output);

        // now we need to see if we can get the output of the command
        let output = cmd.output().expect("failed to execute process");
        let stdout = String::from_utf8_lossy(&output.stdout);

        assert_eq!(stdout.trim(), mock_output);
    }

    #[test]
    fn test_create_dm_getter() {
        let get_test_value = create_dm_getter("test");
        assert_eq!(get_test_value(), None);
    }

    #[test]
    fn test_escape() {
        let sequence = "hello\x1bworld";
        let expected_output = "\x1bPtmux;hello\x1b\x1bworld\x1b\\";
        assert_eq!(escape(sequence), expected_output);
    }
}
