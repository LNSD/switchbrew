project('nx-arm', version : '0.1.0')

cargo = find_program('cargo', required : true)

## Static library
# Source files
inc = include_directories('include')

# Target
nx_arm_tgt = custom_target(
    'nx-arm',
    command : [
        cargo, 'build',
        '--package', meson.project_name(),
        '--profile', get_option('buildtype') == 'release' ? 'release' : 'dev',
        '--target-dir', '@PRIVATE_DIR@',
        '--artifact-dir', '@OUTDIR@',
    ],
    output : 'libnx_arm.a',
    build_by_default : true,
)

nx_arm_dep = declare_dependency(
    include_directories : inc,
    link_with : nx_arm_tgt[0],
)