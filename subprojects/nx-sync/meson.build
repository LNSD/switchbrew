project('nx-sync', version : '0.1.0')

cargo = find_program('cargo', required : true)

#------------------------------------------------
# Dependencies
#------------------------------------------------
# Rust dependencies here ar just informative so Meson can build the dependencies in the correct order
# nx-svc
nx_svc_proj = subproject('nx-svc')
nx_svc_dep = nx_svc_proj.get_variable('nx_svc_dep')

# nx-thread
nx_thread_tls_proj = subproject('nx-thread-tls')
nx_thread_tls_dep = nx_thread_tls_proj.get_variable('nx_thread_tls_dep')

# Dependencies list
deps = [
    nx_svc_dep,
    nx_thread_tls_dep,
]

#------------------------------------------------
# Static library
#------------------------------------------------
# Include directories
inc = include_directories('include')

# Target
nx_sync_tgt = custom_target(
    'nx-sync',
    command : [
        cargo, 'build',
        '--package', meson.project_name(),
        '--profile', get_option('buildtype') == 'release' ? 'release' : 'dev',
        '--target-dir', meson.global_build_root() / 'cargo-target',
        '--artifact-dir', '@OUTDIR@',
    ],
    output : 'libnx_sync.a',
    console: true,
    build_by_default : true,
    build_always_stale: true,
)

nx_sync_dep = declare_dependency(
    include_directories : inc,
    link_with : nx_sync_tgt[0],
    dependencies : deps,
)

# Linker script override for sync functions
nx_sync_ld_override = meson.current_source_dir() / 'sync_redirect.ld'
