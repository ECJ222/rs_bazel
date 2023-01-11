# Loads the `http_archive` function definition
load("@bazel_tools//tools/build_defs/repo:http.bzl", "http_archive")

# Downloads and extracts a tar file from the specified URL and creates a new repository rule with the given name
http_archive(
    name = "rules_rust",
    
    # Specifies the SHA-256 hash of the tar file. This is used to verify the integrity of the downloaded tar file.
    sha256 = "aaaa4b9591a5dad8d8907ae2dbe6e0eb49e6314946ce4c7149241648e56a1277",
    
    # Specifies the URL of the tar file to download
    urls = ["https://github.com/bazelbuild/rules_rust/releases/download/0.16.1/rules_rust-v0.16.1.tar.gz"],
)

# Loads the `rules_rust_dependencies` and `rust_register_toolchains` function definitions
load("@rules_rust//rust:repositories.bzl", "rules_rust_dependencies", "rust_register_toolchains")

# Adds the necessary dependencies for the Rust rules
rules_rust_dependencies()

# Registers Rust toolchains with the given versions and editions
rust_register_toolchains(
    versions=["1.66.0"],
    
    # Specifies the Rust edition to use for the registered toolchains
    edition = "2021",
)
