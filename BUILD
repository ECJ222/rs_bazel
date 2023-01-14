# Loads the Rust rules and the `rust_binary` function definition.
load("@rules_rust//rust:defs.bzl", "rust_binary")

# Declares a Rust binary target with the given name.
rust_binary(
    name = "rs_bazel",
    
    # Specifies the source file for the binary.
    srcs = ["src/main.rs"],
    
    # Specifies dependencies for the binary.
    deps = [
        # Depend on the `substring_library` library we previously declared.
        '//substring-library:substring_library'
    ],
    
    # Specifies the Rust edition to use for this binary.
    edition = "2021"
)