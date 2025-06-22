/**
 * @file shmem.h
 * @brief Shared memory object handling
 * @author plutoo
 * @copyright libnx Authors
 * @remark Shared memory differs from transfer memory in the fact that the kernel (as opposed to the user process) allocates and owns its backing memory.
 */

#pragma once

#include <stdint.h>
#include <stddef.h>

#ifndef BIT
#define BIT(n) (1U<<(n))
#endif

typedef uint32_t Handle;   ///< Kernel object handle.

/// Memory permission bitmasks (copied from libnx types).
typedef enum {
    Perm_None     = 0,               ///< No permissions.
    Perm_R        = BIT(0),          ///< Read permission.
    Perm_W        = BIT(1),          ///< Write permission.
    Perm_X        = BIT(2),          ///< Execute permission.
    Perm_Rw       = Perm_R | Perm_W, ///< Read/write permissions.
    Perm_Rx       = Perm_R | Perm_X, ///< Read/execute permissions.
    Perm_DontCare = BIT(28),         ///< Don't care
} Permission;

/// Shared memory information structure.
typedef struct {
    Handle      handle;   ///< Kernel object handle.
    size_t      size;     ///< Size of the shared memory object.
    Permission  perm;     ///< Permissions.
    void*       map_addr; ///< Address to which the shared memory object is mapped.
} SharedMemory;

/**
 * @brief Creates a shared memory object.
 * @param s Shared memory information structure which will be filled in.
 * @param size Size of the shared memory object to create.
 * @param local_perm Permissions with which the shared memory object will be mapped in the local process.
 * @param remote_perm Permissions with which the shared memory object will be mapped in the remote process (can be Perm_DontCare).
 * @return Result code.
 * @warning This is a privileged operation; in normal circumstances applications cannot use this function.
 */
uint32_t __nx_shmem_create(SharedMemory* s, size_t size, Permission local_perm, Permission remote_perm);

/**
 * @brief Loads a shared memory object coming from a remote process.
 * @param s Shared memory information structure which will be filled in.
 * @param handle Handle of the shared memory object.
 * @param size Size of the shared memory object that is being loaded.
 * @param perm Permissions with which the shared memory object will be mapped in the local process.
 */
void __nx_shmem_load_remote(SharedMemory* s, Handle handle, size_t size, Permission perm);

/**
 * @brief Maps a shared memory object.
 * @param s Shared memory information structure.
 * @return Result code.
 */
uint32_t __nx_shmem_map(SharedMemory* s);

/**
 * @brief Unmaps a shared memory object.
 * @param s Shared memory information structure.
 * @return Result code.
 */
uint32_t __nx_shmem_unmap(SharedMemory* s);

/**
 * @brief Retrieves the mapped address of a shared memory object.
 * @param s Shared memory information structure.
 * @return Mapped address of the shared memory object.
 */
void* __nx_shmem_get_addr(SharedMemory* s);

/**
 * @brief Frees up resources used by a shared memory object, unmapping and closing handles, etc.
 * @param s Shared memory information structure.
 * @return Result code.
 */
uint32_t __nx_shmem_close(SharedMemory* s);
