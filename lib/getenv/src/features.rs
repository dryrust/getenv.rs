// This is free and unencumbered software released into the public domain.

/// The set of features that are enabled in this build of the crate.
pub static FEATURES: &[&str] = &[
    #[cfg(feature = "aws")]
    "aws",
    #[cfg(feature = "cargo")]
    "cargo",
    #[cfg(feature = "conda")]
    "conda",
    #[cfg(feature = "cuda")]
    "cuda",
    #[cfg(feature = "docker")]
    "docker",
    #[cfg(feature = "git")]
    "git",
    #[cfg(feature = "go")]
    "go",
    #[cfg(feature = "homebrew")]
    "homebrew",
    #[cfg(feature = "java")]
    "java",
    #[cfg(feature = "locale")]
    "locale",
    #[cfg(feature = "macos")]
    "macos",
    #[cfg(feature = "near")]
    "near",
    #[cfg(feature = "node")]
    "node",
    #[cfg(feature = "openssl")]
    "openssl",
    #[cfg(feature = "posix")]
    "posix",
    #[cfg(feature = "proxy")]
    "proxy",
    #[cfg(feature = "python")]
    "python",
    #[cfg(feature = "ruby")]
    "ruby",
    #[cfg(feature = "rust")]
    "rust",
    #[cfg(feature = "ssh")]
    "ssh",
    #[cfg(feature = "windows")]
    "windows",
    #[cfg(feature = "xdg")]
    "xdg",
];
