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
