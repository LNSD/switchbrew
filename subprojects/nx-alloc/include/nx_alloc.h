/**
 * @file nx_alloc.h
 * @brief Heap memory allocation (dlmalloc-backed) exposed by the nx-alloc crate.
 *        These functions are implemented in Rust (see ffi.rs) and provide
 *        malloc/free style primitives for use from C/C++ code.
 * @author LNSD
 * @copyright libnx Authors
 */
#pragma once

#include <stddef.h>

#ifdef __cplusplus
extern "C" {
#endif

/**
 * @brief Allocates a block of memory.
 * @param size Size of the memory block in bytes.
 * @return Pointer to the allocated memory, or NULL on failure.
 */
void* __nx_alloc_malloc(size_t size);

/**
 * @brief Allocates a block of memory with the specified alignment.
 * @param alignment Alignment, which must be a power of two.
 * @param size Size of the memory block in bytes. Must be a multiple of @p alignment.
 * @return Pointer to the allocated memory, or NULL on failure.
 */
void* __nx_alloc_aligned_alloc(size_t alignment, size_t size);

/**
 * @brief Allocates a zero-initialized block of memory for an array.
 * @param nmemb Number of elements.
 * @param size Size of each element in bytes.
 * @return Pointer to the allocated memory, or NULL on failure.
 */
void* __nx_alloc_calloc(size_t nmemb, size_t size);

/**
 * @brief Changes the size of the memory block pointed to by ptr.
 * @param ptr Pointer to the memory block to resize (may be NULL).
 * @param new_size New size in bytes.
 * @return Pointer to the reallocated memory, or NULL on failure.
 *         If new_size is zero, the memory is freed and NULL is returned.
 */
void* __nx_alloc_realloc(void* ptr, size_t new_size);

/**
 * @brief Frees a previously allocated block of memory.
 * @param p Pointer returned by one of the __nx_alloc_* allocation functions.
 */
void __nx_alloc_free(void* p);

#ifdef __cplusplus
}
#endif
