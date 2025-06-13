/**
 * @file nx_rand.h
 * @brief OS-seeded pseudo-random number generation support (ChaCha algorithm).
 * @author LNSD
 * @copyright libnx Authors
 */
#pragma once

#include <stddef.h>
#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

/**
 * @brief Fills a buffer with random data.
 * @param buf Pointer to the buffer.
 * @param len Size of the buffer in bytes.
 */
void __nx_rand_get(void* buf, size_t len);

/**
 * @brief Returns a random 64-bit value.
 * @return Random value.
 */
uint64_t __nx_rand_get64(void);

#ifdef __cplusplus
}
#endif 
