// This is free and unencumbered software released into the public domain.

use crate::var;

/// See: https://docs.oracle.com/javase/tutorial/essential/environment/paths.html
/// See: https://docs.oracle.com/javase/7/docs/technotes/tools/windows/classpath.html
pub fn classpath() -> Option<String> {
    var("CLASSPATH")
}

/// See: https://www.baeldung.com/java-home-on-windows-mac-os-x-linux
/// See: https://docs.oracle.com/javase/tutorial/essential/environment/paths.html
pub fn java_home() -> Option<String> {
    var("JAVA_HOME")
}
