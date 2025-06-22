/* Static linker script for thread function symbols redirection */
/* Redirects libnx thread functions to nx-sys-mem __nx_sys_mem_* implementations */

/* Virtual Memory Management (VMM) */
EXTERN(__nx_virtmem_lock);
EXTERN(__nx_virtmem_unlock);
EXTERN(__nx_virtmem_find_aslr);
EXTERN(__nx_virtmem_find_stack);
EXTERN(__nx_virtmem_find_code_memory);
EXTERN(__nx_virtmem_add_reservation);
EXTERN(__nx_virtmem_remove_reservation);

virtmemLock              = __nx_virtmem_lock;
virtmemUnlock            = __nx_virtmem_unlock;
virtmemFindAslr          = __nx_virtmem_find_aslr;
virtmemFindStack         = __nx_virtmem_find_stack;
virtmemFindCodeMemory    = __nx_virtmem_find_code_memory;
virtmemAddReservation    = __nx_virtmem_add_reservation;
virtmemRemoveReservation = __nx_virtmem_remove_reservation;

EXTERN(__nx_virtmem_setup);
virtmemSetup = __nx_virtmem_setup;

/* Shared Memory (Shmem) */
EXTERN(__nx_shmem_create);
EXTERN(__nx_shmem_load_remote);
EXTERN(__nx_shmem_map);
EXTERN(__nx_shmem_unmap);
EXTERN(__nx_shmem_get_addr);
EXTERN(__nx_shmem_close);

shmemCreate     = __nx_shmem_create;
shmemLoadRemote = __nx_shmem_load_remote;
shmemMap        = __nx_shmem_map;
shmemUnmap      = __nx_shmem_unmap;
shmemGetAddr    = __nx_shmem_get_addr;
shmemClose      = __nx_shmem_close;

/* Transfer Memory (Tmem) */
EXTERN(__nx_tmem_create);
EXTERN(__nx_tmem_create_from_memory);
EXTERN(__nx_tmem_load_remote);
EXTERN(__nx_tmem_map);
EXTERN(__nx_tmem_unmap);
EXTERN(__nx_tmem_close_handle);
EXTERN(__nx_tmem_wait_for_permission);
EXTERN(__nx_tmem_close);

tmemCreate            = __nx_tmem_create;
tmemCreateFromMemory  = __nx_tmem_create_from_memory;
tmemLoadRemote        = __nx_tmem_load_remote;
tmemMap               = __nx_tmem_map;
tmemUnmap             = __nx_tmem_unmap;
tmemCloseHandle       = __nx_tmem_close_handle;
tmemWaitForPermission = __nx_tmem_wait_for_permission;
tmemClose             = __nx_tmem_close;
