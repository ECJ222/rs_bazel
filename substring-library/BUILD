# Loads the Rust rules and the `rust_library` and `rust_test` function definitions.
load("@rules_rust//rust:defs.bzl", "rust_library", "rust_test")

# Set the default visibility for the package to be public.
package(default_visibility = ["//visibility:public"])

# Declares a Rust library target with the given name.
rust_library(
    name = "substring_library",
    
    # Specifies the source files for the library.
    srcs = ["src/lib.rs"],
    
    # Specifies the Rust edition to use for this library.
    edition = "2021"
)

# Declares a Rust test target with the given name.
rust_test(
    name = "substring_library_test",
    
    # Specifies the source file for the test.
    srcs = ["src/lib.rs"],
    
    # Specifies dependencies for the test.
    deps = [
        # Depend on the library we just declared.
        ":substring_library",
    ],
    
    # Specifies the Rust edition to use for this test.
    edition = "2021"
)
