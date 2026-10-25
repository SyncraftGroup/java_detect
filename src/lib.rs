mod platform;

use std::path::{Path, PathBuf};

use java_runtimes::detector;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::platform::JAVA_ROOTS;

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone)]
pub struct JavaInstallation {
    pub path: PathBuf,
    pub version: u8,
}

impl std::fmt::Display for JavaInstallation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Version {} ({})",
            self.version,
            self.path.to_string_lossy()
        )
    }
}

/// Find a Java installation that can run a version requiring `version`.
///
/// Prefers the closest available major version at or above the requirement
/// rather than demanding an exact match (Minecraft 1.21 runs on Java 21, 22…).
pub fn get_java(version: u8) -> Option<JavaInstallation> {
    detect()
        .into_iter()
        .filter(|java| java.version >= version)
        .min_by_key(|java| java.version)
}

pub fn detect() -> Vec<JavaInstallation> {
    let mut result = Vec::new();

    let paths: Vec<_> = JAVA_ROOTS.iter().map(Path::new).collect();
    let runtimes = detector::detect_java_in_paths(&paths, 5);
    for runtime in &runtimes {
        if runtime.is_same_os()
            && runtime.is_available()
            && let Some(version) = parse_java_version(runtime.get_version_string())
        {
            result.push(JavaInstallation {
                path: runtime.get_executable().into(),
                version,
            });
        }
    }
    result
}

fn parse_java_version(text: &str) -> Option<u8> {
    if let Some(rest) = text.strip_prefix("1.") {
        // Java 8: 1.8.0_421 -> 8
        rest.split('.').next()?.parse().ok()
    } else {
        // Java 17: 17.0.12 -> 17
        // Java 21: 21.0.2 -> 21
        text.split('.').next()?.parse().ok()
    }
}
