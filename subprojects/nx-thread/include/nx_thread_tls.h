#pragma once

#include <stddef.h>

/**
 * @brief Gets the thread local storage buffer.
 * @return The thread local storage buffer.
 */
void* __nx_thread_get_tls(void);

/**
 * @brief Get the TLS start offset for Thread Control Block (TCB) calculations.
 * @return The offset where the actual TLS data begins within a thread's TLS region.
 */
size_t __nx_thread_get_tls_start_offset(void);
