#[cfg(target_os = "windows")]
pub const JAVA_ROOTS: &[&str] = &[
    r"C:\Program Files",
    r"C:\Program Files (x86)",
    r"D:\Program Files",
    r"D:\Program Files (x86)",
];

#[cfg(target_os = "linux")]
pub const JAVA_ROOTS: &[&str] = &[
    "/usr/lib/jvm",
    "/usr/java",
    "/opt",
    "/opt/java",
    "/usr/local",
    "/usr/local/java",
    "/snap",
];

#[cfg(target_os = "macos")]
pub const JAVA_ROOTS: &[&str] = &[
    "/Library/Java/JavaVirtualMachines",
    "/System/Library/Java/JavaVirtualMachines",
    "/opt/homebrew/opt",
    "/usr/local/opt",
];
