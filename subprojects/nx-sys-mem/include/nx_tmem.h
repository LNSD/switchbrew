/**
 * @file tmem.h
 * @brief Transfer memory handling
 * @author plutoo
 * @copyright libnx Authors
 * @remark Transfer memory differs from shared memory in the fact that the user process (as opposed to the kernel) allocates and owns its backing memory.
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

/// Transfer memory information structure.
typedef struct {
    Handle      handle;   ///< Kernel object handle.
    size_t      size;     ///< Size of the transfer memory object.
    Permission  perm;     ///< Permissions of the transfer memory object.
    void*       src_addr; ///< Address of the source backing memory.
    void*       map_addr; ///< Address to which the transfer memory object is mapped.
} TransferMemory;

/**
 * @brief Creates a transfer memory object.
 * @param t Transfer memory information structure that will be filled in.
 * @param size Size of the transfer memory object to create.
 * @param perm Permissions with which to protect the transfer memory in the local process.
 * @return Result code.
 */
uint32_t __nx_tmem_create(TransferMemory* t, size_t size, Permission perm);

/**
 * @brief Creates a transfer memory object from existing memory.
 * @param t Transfer memory information structure that will be filled in.
 * @param buf Pointer to a page-aligned buffer.
 * @param size Size of the transfer memory object to create.
 * @param perm Permissions with which to protect the transfer memory in the local process.
 * @return Result code.
 */
uint32_t __nx_tmem_create_from_memory(TransferMemory* t, void* buf, size_t size, Permission perm);

/**
 * @brief Loads a transfer memory object coming from a remote process.
 * @param t Transfer memory information structure which will be filled in.
 * @param handle Handle of the transfer memory object.
 * @param size Size of the transfer memory object that is being loaded.
 * @param perm Permissions which the transfer memory is expected to have in the process that owns the memory.
 * @warning This is a privileged operation; in normal circumstances applications shouldn't use this function.
 */
void __nx_tmem_load_remote(TransferMemory* t, Handle handle, size_t size, Permission perm);

/**
 * @brief Maps a transfer memory object.
 * @param t Transfer memory information structure.
 * @return Result code.
 * @warning This is a privileged operation; in normal circumstances applications cannot use this function.
 */
uint32_t __nx_tmem_map(TransferMemory* t);

/**
 * @brief Unmaps a transfer memory object.
 * @param t Transfer memory information structure.
 * @return Result code.
 * @warning This is a privileged operation; in normal circumstances applications cannot use this function.
 */
uint32_t __nx_tmem_unmap(TransferMemory* t);

/**
 * @brief Retrieves the mapped address of a transfer memory object.
 * @param t Transfer memory information structure.
 * @return Mapped address of the transfer memory object.
 */
static inline void* __nx_tmem_get_addr(TransferMemory* t){
    return t->map_addr;
}

/**
 * @brief Closes handle of a transfer memory object.
 * @param t Transfer memory information structure.
 * @return Result code.
 */
uint32_t __nx_tmem_close_handle(TransferMemory* t);

/**
 * @brief Waits until source backing memory permissions match perm.
 * @param t Transfer memory information structure.
 * @param perm Permissions which the source backing memory is expected to have before return.
 * @return Result code.
 */
uint32_t __nx_tmem_wait_for_permission(TransferMemory* t, Permission perm);

/**
 * @brief Frees up resources used by a transfer memory object, unmapping and closing handles, etc.
 * @param t Transfer memory information structure.
 * @return Result code.
 */
uint32_t __nx_tmem_close(TransferMemory* t);
