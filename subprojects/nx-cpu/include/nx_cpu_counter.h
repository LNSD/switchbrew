/**
 * @file counter.h
 * @brief AArch64 system counter-timer.
 * @author fincs
 * @copyright libnx Authors
 */
#pragma once
#include <stdint.h>

/**
 * @brief Gets the current system tick.
 * @return The current system tick.
 */
uint64_t __nx_cpu_get_system_tick(void);

/**
 * @brief Gets the system counter-timer frequency
 * @return The system counter-timer frequency, in Hz.
 */
uint64_t __nx_cpu_get_system_tick_freq(void);

/**
 * @brief Converts from nanoseconds to CPU ticks unit.
 * @param ns Time in nanoseconds.
 * @return Time in CPU ticks.
 */
uint64_t __nx_cpu_ns_to_ticks(uint64_t ns);

/**
 * @brief Converts from CPU ticks unit to nanoseconds.
 * @param tick Time in ticks.
 * @return Time in nanoseconds.
 */
uint64_t __nx_cpu_ticks_to_ns(uint64_t tick);

