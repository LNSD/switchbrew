project('nx-svc', version : '0.1.0')

cargo = find_program('cargo', required : true)

## Static library
# Source files
inc = include_directories('include')

# Target
nx_svc_tgt = custom_target(
    'nx-svc',
    command : [
        cargo, 'build',
        '--package', meson.project_name(),
        '--profile', get_option('buildtype') == 'release' ? 'release' : 'dev',
        '--target-dir', '@PRIVATE_DIR@',
        '--artifact-dir', '@OUTDIR@',
    ],
    output : 'libnx_svc.a',
    build_by_default : true,
    build_always_stale: true,
)

nx_svc_dep = declare_dependency(
    include_directories : inc,
    link_with : nx_svc_tgt[0],
)