/**
 * @file mutex.h
 * @brief Mutex synchronization primitive.
 * @author plutoo
 * @copyright libnx Authors
 */
#pragma once

#include <stdbool.h>
#include <sys/lock.h>

/// Mutex datatype (defined in newlib).
typedef _LOCK_T Mutex;

// TODO: Port RMutex (Reentrant Mutex) to Rust
//  Replicate naming like the reentrant mutex in parking_lot crate
// /// Recursive mutex datatype, defined in newlib.
// typedef _LOCK_RECURSIVE_T RMutex;

/**
 * @brief Initializes a mutex.
 * @param m Mutex object.
 * @note A mutex can also be statically initialized by assigning 0 to it.
 */
void __nx_sync_mutex_init(Mutex* m);

/**
 * @brief Locks a mutex.
 * @param m Mutex object.
 */
void __nx_sync_mutex_lock(Mutex* m);

/**
 * @brief Attempts to lock a mutex without waiting.
 * @param m Mutex object.
 * @return 1 if the mutex has been acquired successfully, and 0 on contention.
 */
bool __nx_sync_mutex_try_lock(Mutex* m);

/**
 * @brief Unlocks a mutex.
 * @param m Mutex object.
 */
void __nx_sync_mutex_unlock(Mutex* m);

/**
 * @brief Gets whether the current thread owns the mutex.
 * @param m Mutex object.
 * @return 1 if the mutex is locked by the current thread, and 0 otherwise.
 */
bool __nx_sync_mutex_is_locked_by_current_thread(const Mutex* m);

// TODO: Port RMutex (Reentrant Mutex) to Rust
//  Replicate naming like the reentrant mutex in parking_lot crate
///**
// * @brief Initializes a recursive mutex.
// * @param m Recursive mutex object.
// * @note A recursive mutex can also be statically initialized by assigning {0,0,0} to it.
// */
//static inline void rmutexInit(RMutex* m)
//{
//    m->lock = 0;
//    m->thread_tag = 0;
//    m->counter = 0;
//}
//
///**
// * @brief Locks a recursive mutex.
// * @param m Recursive mutex object.
// */
//void rmutexLock(RMutex* m);
//
///**
// * @brief Attempts to lock a recursive mutex without waiting.
// * @param m Recursive mutex object.
// * @return 1 if the mutex has been acquired successfully, and 0 on contention.
// */
//bool rmutexTryLock(RMutex* m);
//
///**
// * @brief Unlocks a recursive mutex.
// * @param m Recursive mutex object.
// */
//void rmutexUnlock(RMutex* m);
