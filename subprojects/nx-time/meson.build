project('nx-time', version : '0.1.0')

cargo = find_program('cargo', required : true)

#------------------------------------------------
# Dependencies
#------------------------------------------------
# Rust dependencies here ar just informative so Mesom can build the dependencies in the correct order
# nx-cpu
nx_cpu_proj = subproject('nx-cpu')
nx_cpu_dep = nx_cpu_proj.get_variable('nx_cpu_dep')

# Dependencies list
deps = [
    nx_cpu_dep,
]

#------------------------------------------------
# Static library
#------------------------------------------------
# Include directories
inc = include_directories('include')

# Target
nx_time_tgt = custom_target(
    'nx-time',
    command : [
        cargo, 'build',
        '--package', meson.project_name(),
        '--profile', get_option('buildtype') == 'release' ? 'release' : 'dev',
        '--target-dir', '@PRIVATE_DIR@',
        '--artifact-dir', '@OUTDIR@',
    ],
    output : 'libnx_time.a',
    console: true,
    build_by_default : true,
    build_always_stale: true,
)

nx_time_dep = declare_dependency(
    include_directories : inc,
    link_with : nx_time_tgt[0],
    dependencies : deps,
)

# Linker script override for time functions
nx_time_ld_override = meson.current_source_dir() / 'time_redirect.ld'
