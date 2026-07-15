//! Which agent CLIs are installed. A node offers to run one of these; if it's missing, the
//! node says so instead of pretending to work.

use serde::Serialize;

#[derive(Serialize, Clone, Debug, PartialEq)]
pub struct AgentInfo {
    pub id: String,
    pub name: String,
    pub path: String,
    pub available: bool,
}

/// Agents Friday knows how to spawn. Just codex for now — add a row to grow the list.
const KNOWN: &[(&str, &str)] = &[("codex", "Codex")];

pub fn detect() -> Vec<AgentInfo> {
    KNOWN
        .iter()
        .map(|(bin, name)| {
            let found = which(bin);
            AgentInfo {
                id: (*bin).into(),
                name: (*name).into(),
                path: found.clone().unwrap_or_default(),
                available: found.is_some(),
            }
        })
        .collect()
}

/// First entry on PATH that's a file named `bin`. Doesn't check the exec bit —
/// good enough to tell "installed" from "missing"; tighten if a non-exec collision ever bites.
fn which(bin: &str) -> Option<String> {
    let path = std::env::var_os("PATH")?;
    std::env::split_paths(&path)
        .map(|dir| dir.join(bin))
        .find(|p| p.is_file())
        .map(|p| p.display().to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detects_a_binary_that_exists() {
        // `sh` is on PATH on every unix box the dev env runs on.
        assert!(which("sh").is_some());
        assert!(which("this-binary-does-not-exist-friday").is_none());
        assert_eq!(detect().len(), KNOWN.len());
    }
}
