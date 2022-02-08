/// Compiler configuration
struct Config {
    /// Use Linux-stable labels for executables
    linux_labels: bool,

    /// Verbose mode
    verbose: bool,

    /// Stop at generation of assembly
    stop_at_assembly: bool,
}
