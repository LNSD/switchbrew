project('nx-cpu', version : '0.1.0')

cargo = find_program('cargo', required : true)

## Static library
# Target
nx_cpu_tgt = custom_target(
    'nx-cpu',
    command : [
        cargo, 'build',
        '--package', meson.project_name(),
        '--profile', get_option('buildtype') == 'release' ? 'release' : 'dev',
        '--target-dir', '@PRIVATE_DIR@',
        '--artifact-dir', '@OUTDIR@',
    ],
    output : 'libnx_cpu.a',
    build_by_default : true,
    build_always_stale: true,
)

nx_cpu_dep = declare_dependency(
    link_with : nx_cpu_tgt[0],
)
