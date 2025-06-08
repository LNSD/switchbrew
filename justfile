# List available tasks
default:
    @just --list

# Build directory (can be overridden with just builddir=<path> <task>)
build_dir := "build"

# Setup meson build directory
meson-setup:
    meson setup --cross-file devkitpro.txt --cross-file cross.txt {{build_dir}}

alias setup := meson-setup

# Wipe build directory (preserve configuration)
meson-wipe:
    meson setup --cross-file devkitpro.txt --cross-file cross.txt {{build_dir}} --wipe

# Compile the project
meson-compile:
    meson compile -C {{build_dir}}

alias build := meson-compile

# Clean workspace
clean:
    @rm -rf {{build_dir}}  # Remove meson build directory
    @rm -rf target  # Remove cargo build directory
