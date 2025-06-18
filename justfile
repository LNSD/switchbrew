# List available tasks
default:
    @just --list

# Build directory (can be overridden with just builddir=<path> <task>)
build_dir := "buildDir"

# Format Rust code (cargo fmt)
format:
    cargo +nightly fmt --all

alias fmt := format

# Check Rust code format (cargo fmt --check)
check-format:
    cargo +nightly fmt --all -- --check

# Check Rust code (cargo check)
check *EXTRA_FLAGS:
    cargo check {{EXTRA_FLAGS}}

# Setup meson build directory (meson setup)
meson-setup *EXTRA_FLAGS:
    meson setup --cross-file devkitpro.txt --cross-file cross.txt {{build_dir}} {{EXTRA_FLAGS}}

alias setup := meson-setup

# Compile the project (meson compile)
meson-compile *TARGETS:
    meson compile -C {{build_dir}} {{TARGETS}}

alias build := meson-compile

# Clean the build directory
meson-clean:
    meson compile -C {{build_dir}} --clean

# Clean workspace
clean:
    @rm -rf {{build_dir}}

# Install Git hooks
install-git-hooks:
    #!/usr/bin/env bash
    set -e # Exit on error

    # Check if pre-commit is installed
    if ! command -v "pre-commit" &> /dev/null; then
        >&2 echo "=============================================================="
        >&2 echo "Required command 'pre-commit' not available ❌"
        >&2 echo ""
        >&2 echo "Please install pre-commit using your preferred package manager"
        >&2 echo "  pip install pre-commit"
        >&2 echo "  pacman -S pre-commit"
        >&2 echo "  apt-get install pre-commit"
        >&2 echo "  brew install pre-commit"
        >&2 echo "=============================================================="
        exit 1
    fi

    # Install the pre-commit hooks
    pre-commit install --config .github/pre-commit-config.yaml

# Remove Git hooks
remove-git-hooks:
    #!/usr/bin/env bash
    set -e # Exit on error

    # Check if pre-commit is installed
    if ! command -v "pre-commit" &> /dev/null; then
        >&2 echo "=============================================================="
        >&2 echo "Required command 'pre-commit' not available ❌"
        >&2 echo ""
        >&2 echo "Please install pre-commit using your preferred package manager"
        >&2 echo "  pip install pre-commit"
        >&2 echo "  pacman -S pre-commit"
        >&2 echo "  apt-get install pre-commit"
        >&2 echo "  brew install pre-commit"
        >&2 echo "=============================================================="
        exit 1
    fi

    # Remove the pre-commit hooks
    pre-commit uninstall --config .github/pre-commit-config.yaml
