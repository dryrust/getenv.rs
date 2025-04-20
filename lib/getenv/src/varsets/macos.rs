// This is free and unencumbered software released into the public domain.

use crate::var;

/// See: https://developer.apple.com/library/archive/documentation/DeveloperTools/Conceptual/DynamicLibraries/100-Articles/UsingDynamicLibraries.html
pub fn ld_library_path() -> Option<String> {
    var("LD_LIBRARY_PATH")
}

/// See: https://developer.apple.com/library/archive/documentation/MacOSX/Conceptual/BPFrameworks/Tasks/CreatingFrameworks.html
pub fn dyld_framework_path() -> Option<String> {
    var("DYLD_FRAMEWORK_PATH")
}

/// See: https://developer.apple.com/library/archive/documentation/DeveloperTools/Conceptual/DynamicLibraries/100-Articles/UsingDynamicLibraries.html
pub fn dyld_library_path() -> Option<String> {
    var("DYLD_LIBRARY_PATH")
}

/// See: https://developer.apple.com/library/archive/documentation/DeveloperTools/Conceptual/DynamicLibraries/100-Articles/UsingDynamicLibraries.html
pub fn dyld_fallback_library_path() -> Option<String> {
    var("DYLD_FALLBACK_LIBRARY_PATH")
}
