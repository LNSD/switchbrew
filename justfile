# List available tasks
default:
    @just --list

# Setup meson build directory
meson-setup:
    meson setup --cross-file devkitpro.txt --cross-file cross.txt build

alias setup := meson-setup

# Compile the project
meson-compile:
    meson compile -C build

alias build := meson-compile
