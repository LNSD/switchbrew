#include <stdint.h>
#include <stdbool.h>

#include <switch.h>

#include "../harness.h"

/**
* @brief Sleeps the current thread for the given number of milliseconds.
* @param ms The number of milliseconds to sleep.
*/
static inline void threadSleepMs(int64_t ms) {
    svcSleepThread(ms * 1000000);
}

//<editor-fold desc="Test 0001: RwLock read lock single thread">

#define TEST_0001_THREAD_TAG 42

static RwLock g_test_0001_rwlock;
static int64_t g_test_0001_shared_tag = -1;

/**
 * Thread function for Test #0001
 */
void test_0001_rwlock_thread_func(void *arg) {
    const int64_t num = (int64_t) arg;

    rwlockReadLock(&g_test_0001_rwlock);
    g_test_0001_shared_tag = num;
    rwlockReadUnlock(&g_test_0001_rwlock);
}

/**
 * Test rwlock basic read lock functionality in a single thread.
 */
test_rc_t test_0001_rwlock_read_lock_single_thread(void) {
    Result rc = 0;

    //* Given
    // Initialize the test global rwlock
    rwlockInit(&g_test_0001_rwlock);

    // Create a thread
    Thread thread;
    rc = threadCreate(&thread, test_0001_rwlock_thread_func, (void *) TEST_0001_THREAD_TAG, NULL, 0x10000, 0x2C, -2);
    if (R_FAILED(rc)) {
        goto test_cleanup;
    }

    //* When
    // Start the thread
    rc = threadStart(&thread);
    if (R_FAILED(rc)) {
        goto test_cleanup;
    }

    // Wait for the thread to set the shared tag (10ms should be enough)
    threadSleepMs(10);

    uint64_t shared_tag = g_test_0001_shared_tag;

    //* Then
    // Assert that the shared tag is set to TEST_0001_THREAD_TAG
    if (shared_tag != TEST_0001_THREAD_TAG) {
        rc = TEST_ASSERTION_FAILED;
        goto test_cleanup;
    }

test_cleanup:
    threadWaitForExit(&thread);
    threadClose(&thread);

    return rc;
}

//</editor-fold>

//<editor-fold desc="Test 0002: RwLock write lock single thread">

#define TEST_0002_THREAD_TAG 84

static RwLock g_test_0002_rwlock;
static int64_t g_test_0002_shared_tag = -1;

/**
 * Thread function for Test #0002
 */
void test_0002_rwlock_thread_func(void *arg) {
    const int64_t num = (int64_t) arg;

    rwlockWriteLock(&g_test_0002_rwlock);
    g_test_0002_shared_tag = num;
    rwlockWriteUnlock(&g_test_0002_rwlock);
}

/**
 * Test rwlock basic write lock functionality in a single thread.
 */
test_rc_t test_0002_rwlock_write_lock_single_thread(void) {
    Result rc = 0;

    //* Given
    // Initialize the test global rwlock
    rwlockInit(&g_test_0002_rwlock);

    // Create a thread
    Thread thread;
    rc = threadCreate(&thread, test_0002_rwlock_thread_func, (void *) TEST_0002_THREAD_TAG, NULL, 0x10000, 0x2C, -2);
    if (R_FAILED(rc)) {
        goto test_cleanup;
    }

    //* When
    // Start the thread
    rc = threadStart(&thread);
    if (R_FAILED(rc)) {
        goto test_cleanup;
    }

    // Wait for the thread to set the shared tag (10ms should be enough)
    threadSleepMs(10);

    uint64_t shared_tag = g_test_0002_shared_tag;

    //* Then
    // Assert that the shared tag is set to TEST_0002_THREAD_TAG
    if (shared_tag != TEST_0002_THREAD_TAG) {
        rc = TEST_ASSERTION_FAILED;
        goto test_cleanup;
    }

test_cleanup:
    threadWaitForExit(&thread);
    threadClose(&thread);

    return rc;
}

//</editor-fold>

//<editor-fold desc="Test 0003: RwLock multiple readers concurrent">

#define TEST_0003_NUM_READERS 4
#define TEST_0003_READ_DELAY_MS 100

static RwLock g_test_0003_rwlock;
static Mutex g_test_0003_mutex;
static uint32_t g_test_0003_active_readers = 0;
static uint32_t g_test_0003_max_concurrent_readers = 0;
static uint32_t g_test_0003_completed_readers = 0;

/**
 * Thread function for Test #0003
 */
void test_0003_rwlock_reader_thread_func(void *arg) {
    // Acquire read lock
    rwlockReadLock(&g_test_0003_rwlock);
    
    // Update active readers count using mutex for thread safety
    mutexLock(&g_test_0003_mutex);
    g_test_0003_active_readers++;
    if (g_test_0003_active_readers > g_test_0003_max_concurrent_readers) {
        g_test_0003_max_concurrent_readers = g_test_0003_active_readers;
    }
    mutexUnlock(&g_test_0003_mutex);
    
    // Do some read work
    threadSleepMs(TEST_0003_READ_DELAY_MS);
    
    // Update counters using mutex for thread safety
    mutexLock(&g_test_0003_mutex);
    g_test_0003_active_readers--;
    g_test_0003_completed_readers++;
    mutexUnlock(&g_test_0003_mutex);
    
    // Release read lock
    rwlockReadUnlock(&g_test_0003_rwlock);
}

/**
 * Test multiple readers can acquire read locks concurrently.
 */
test_rc_t test_0003_rwlock_multiple_readers_concurrent(void) {
    Result rc = 0;

    //* Given
    // Initialize the test global rwlock and mutex
    rwlockInit(&g_test_0003_rwlock);
    mutexInit(&g_test_0003_mutex);

    // Create reader threads
    Thread threads[TEST_0003_NUM_READERS];
    for (int i = 0; i < TEST_0003_NUM_READERS; i++) {
        rc = threadCreate(&threads[i], test_0003_rwlock_reader_thread_func, NULL, NULL, 0x10000, 0x2C, -2);
        if (R_FAILED(rc)) {
            goto test_cleanup;
        }
    }

    //* When
    // Start all threads simultaneously
    for (int i = 0; i < TEST_0003_NUM_READERS; i++) {
        rc = threadStart(&threads[i]);
        if (R_FAILED(rc)) {
            goto test_cleanup;
        }
    }
    
    const int64_t t0 = 0;

    // T1: Sleep briefly to allow threads to acquire read locks
    const int64_t t1 = t0 + 10; // t0 + 10ms
    threadSleepMs(t1 - t0);

    // Check concurrent readers
    mutexLock(&g_test_0003_mutex);
    const uint32_t active_readers_t1 = g_test_0003_active_readers;
    const uint32_t completed_readers_t1 = g_test_0003_completed_readers;
    mutexUnlock(&g_test_0003_mutex);
    
    // T2: Wait for all readers to complete
    const int64_t t2 = t1 + TEST_0003_READ_DELAY_MS + 10; // t1 + 100ms + 10ms
    threadSleepMs(t2 - t1);

    mutexLock(&g_test_0003_mutex);
    const uint32_t active_readers_t2 = g_test_0003_active_readers;
    const uint32_t completed_readers_t2 = g_test_0003_completed_readers;
    const uint32_t max_concurrent_readers = g_test_0003_max_concurrent_readers;
    mutexUnlock(&g_test_0003_mutex);

    //* Then
    // - T1
    // Assert that all readers are active concurrently
    if (active_readers_t1 != TEST_0003_NUM_READERS) {
        rc = TEST_ASSERTION_FAILED;
        goto test_cleanup;
    }
    
    // Assert that no readers have completed yet
    if (completed_readers_t1 != 0) {
        rc = TEST_ASSERTION_FAILED;
        goto test_cleanup;
    }

    // - T2
    // Assert that no readers are active after completion
    if (active_readers_t2 != 0) {
        rc = TEST_ASSERTION_FAILED;
        goto test_cleanup;
    }
    
    // Assert that all readers have completed
    if (completed_readers_t2 != TEST_0003_NUM_READERS) {
        rc = TEST_ASSERTION_FAILED;
        goto test_cleanup;
    }
    
    // Assert that all readers were concurrent at some point
    if (max_concurrent_readers != TEST_0003_NUM_READERS) {
        rc = TEST_ASSERTION_FAILED;
        goto test_cleanup;
    }

    //* Clean-up
test_cleanup:
    for (int i = 0; i < TEST_0003_NUM_READERS; i++) {
        threadWaitForExit(&threads[i]);
        threadClose(&threads[i]);
    }

    return rc;
}

//</editor-fold>

//<editor-fold desc="Test 0004: RwLock write lock exclusive">

#define TEST_0004_NUM_READERS 3
#define TEST_0004_WRITER_START_DELAY_MS 50
#define TEST_0004_WRITER_WORK_DELAY_MS 150
#define TEST_0004_READER_START_DELAY_MS 100

static RwLock g_test_0004_rwlock;
static Mutex g_test_0004_mutex;
static uint32_t g_test_0004_active_readers = 0;
static bool g_test_0004_writer_active = false;
static uint32_t g_test_0004_completed_readers = 0;
static bool g_test_0004_writer_completed = false;

/**
 * Writer thread function for Test #0004
 */
void test_0004_rwlock_writer_thread_func(void *arg) {
    threadSleepMs(TEST_0004_WRITER_START_DELAY_MS);
    
    // Acquire write lock
    rwlockWriteLock(&g_test_0004_rwlock);
    
    // Mark writer as active
    mutexLock(&g_test_0004_mutex);
    g_test_0004_writer_active = true;
    mutexUnlock(&g_test_0004_mutex);
    
    // Do some write work
    threadSleepMs(TEST_0004_WRITER_WORK_DELAY_MS);
    
    // Mark writer as completed
    mutexLock(&g_test_0004_mutex);
    g_test_0004_writer_active = false;
    g_test_0004_writer_completed = true;
    mutexUnlock(&g_test_0004_mutex);
    
    // Release write lock
    rwlockWriteUnlock(&g_test_0004_rwlock);
}

/**
 * Reader thread function for Test #0004
 */
void test_0004_rwlock_reader_thread_func(void *arg) {
    threadSleepMs(TEST_0004_READER_START_DELAY_MS);
    
    // Acquire read lock (should block while writer is active)
    rwlockReadLock(&g_test_0004_rwlock);
    
    // Update active readers count using mutex for thread safety
    mutexLock(&g_test_0004_mutex);
    g_test_0004_active_readers++;
    mutexUnlock(&g_test_0004_mutex);
    
    // Do some read work
    threadSleepMs(50);
    
    // Update counters using mutex for thread safety
    mutexLock(&g_test_0004_mutex);
    g_test_0004_active_readers--;
    g_test_0004_completed_readers++;
    mutexUnlock(&g_test_0004_mutex);
    
    // Release read lock
    rwlockReadUnlock(&g_test_0004_rwlock);
}

/**
 * Test write lock excludes all other access (readers and writers).
 */
test_rc_t test_0004_rwlock_write_lock_exclusive(void) {
    Result rc = 0;

    //* Given
    // Initialize the test global rwlock and mutex
    rwlockInit(&g_test_0004_rwlock);
    mutexInit(&g_test_0004_mutex);

    // Create writer thread
    Thread writer_thread;
    rc = threadCreate(&writer_thread, test_0004_rwlock_writer_thread_func, NULL, NULL, 0x10000, 0x2C, -2);
    if (R_FAILED(rc)) {
        goto test_cleanup;
    }

    // Create reader threads
    Thread reader_threads[TEST_0004_NUM_READERS];
    for (int i = 0; i < TEST_0004_NUM_READERS; i++) {
        rc = threadCreate(&reader_threads[i], test_0004_rwlock_reader_thread_func, NULL, NULL, 0x10000, 0x2C, -2);
        if (R_FAILED(rc)) {
            goto test_cleanup;
        }
    }

    //* When
    // Start writer thread first
    rc = threadStart(&writer_thread);
    if (R_FAILED(rc)) {
        goto test_cleanup;
    }

    // Start all reader threads
    for (int i = 0; i < TEST_0004_NUM_READERS; i++) {
        rc = threadStart(&reader_threads[i]);
        if (R_FAILED(rc)) {
            goto test_cleanup;
        }
    }
    
    const int64_t t0 = 0;

    // T1: Check state during writer activity
    const int64_t t1 = t0 + TEST_0004_WRITER_START_DELAY_MS + 50; // t0 + 100ms
    threadSleepMs(t1 - t0);

    mutexLock(&g_test_0004_mutex);
    const bool writer_active_t1 = g_test_0004_writer_active;
    const uint32_t active_readers_t1 = g_test_0004_active_readers;
    const uint32_t completed_readers_t1 = g_test_0004_completed_readers;
    mutexUnlock(&g_test_0004_mutex);
    
    // T2: Check state after writer completes
    const int64_t t2 = t0 + TEST_0004_WRITER_START_DELAY_MS + TEST_0004_WRITER_WORK_DELAY_MS + 50; // t0 + 250ms
    threadSleepMs(t2 - t1);

    mutexLock(&g_test_0004_mutex);
    const bool writer_active_t2 = g_test_0004_writer_active;
    const bool writer_completed_t2 = g_test_0004_writer_completed;
    const uint32_t active_readers_t2 = g_test_0004_active_readers;
    const uint32_t completed_readers_t2 = g_test_0004_completed_readers;
    mutexUnlock(&g_test_0004_mutex);

    // T3: Wait for all readers to complete
    const int64_t t3 = t2 + 100; // t2 + 100ms
    threadSleepMs(t3 - t2);

    mutexLock(&g_test_0004_mutex);
    const uint32_t active_readers_t3 = g_test_0004_active_readers;
    const uint32_t completed_readers_t3 = g_test_0004_completed_readers;
    mutexUnlock(&g_test_0004_mutex);

    //* Then
    // - T1
    // Assert that writer is active
    if (!writer_active_t1) {
        rc = TEST_ASSERTION_FAILED;
        goto test_cleanup;
    }
    
    // Assert that no readers are active (blocked by writer)
    if (active_readers_t1 != 0) {
        rc = TEST_ASSERTION_FAILED;
        goto test_cleanup;
    }
    
    // Assert that no readers have completed yet
    if (completed_readers_t1 != 0) {
        rc = TEST_ASSERTION_FAILED;
        goto test_cleanup;
    }

    // - T2
    // Assert that writer is no longer active
    if (writer_active_t2) {
        rc = TEST_ASSERTION_FAILED;
        goto test_cleanup;
    }
    
    // Assert that writer has completed
    if (!writer_completed_t2) {
        rc = TEST_ASSERTION_FAILED;
        goto test_cleanup;
    }
    
    // Assert that all readers are now active (unblocked after writer)
    if (active_readers_t2 != TEST_0004_NUM_READERS) {
        rc = TEST_ASSERTION_FAILED;
        goto test_cleanup;
    }

    // - T3
    // Assert that no readers are active
    if (active_readers_t3 != 0) {
        rc = TEST_ASSERTION_FAILED;
        goto test_cleanup;
    }
    
    // Assert that all readers have completed
    if (completed_readers_t3 != TEST_0004_NUM_READERS) {
        rc = TEST_ASSERTION_FAILED;
        goto test_cleanup;
    }

    //* Clean-up
test_cleanup:
    threadWaitForExit(&writer_thread);
    threadClose(&writer_thread);
    
    for (int i = 0; i < TEST_0004_NUM_READERS; i++) {
        threadWaitForExit(&reader_threads[i]);
        threadClose(&reader_threads[i]);
    }

    return rc;
}

//</editor-fold>

//<editor-fold desc="Test 0005: RwLock reader writer priority">

#define TEST_0005_NUM_READERS 3
#define TEST_0005_NUM_WRITERS 2
#define TEST_0005_WORK_DELAY_MS 100

static RwLock g_test_0005_rwlock;
static Mutex g_test_0005_mutex;
static uint32_t g_test_0005_active_readers = 0;
static uint32_t g_test_0005_active_writers = 0;
static uint32_t g_test_0005_completed_readers = 0;
static uint32_t g_test_0005_completed_writers = 0;
static int64_t g_test_0005_execution_order[TEST_0005_NUM_READERS + TEST_0005_NUM_WRITERS];
static uint32_t g_test_0005_execution_index = 0;

typedef struct {
    int64_t thread_id;
    bool is_writer;
    int64_t start_delay_ms;
} Test0005_ThreadArgs;

/**
 * Reader thread function for Test #0005
 */
void test_0005_rwlock_reader_thread_func(void *arg) {
    const Test0005_ThreadArgs *args = arg;
    
    threadSleepMs(args->start_delay_ms);
    
    // Acquire read lock
    rwlockReadLock(&g_test_0005_rwlock);
    
    // Record execution order and update counters
    mutexLock(&g_test_0005_mutex);
    g_test_0005_execution_order[g_test_0005_execution_index++] = args->thread_id;
    g_test_0005_active_readers++;
    mutexUnlock(&g_test_0005_mutex);
    
    // Do some read work
    threadSleepMs(TEST_0005_WORK_DELAY_MS);
    
    // Update counters
    mutexLock(&g_test_0005_mutex);
    g_test_0005_active_readers--;
    g_test_0005_completed_readers++;
    mutexUnlock(&g_test_0005_mutex);
    
    // Release read lock
    rwlockReadUnlock(&g_test_0005_rwlock);
}

/**
 * Writer thread function for Test #0005
 */
void test_0005_rwlock_writer_thread_func(void *arg) {
    const Test0005_ThreadArgs *args = arg;
    
    threadSleepMs(args->start_delay_ms);
    
    // Acquire write lock
    rwlockWriteLock(&g_test_0005_rwlock);
    
    // Record execution order and update counters
    mutexLock(&g_test_0005_mutex);
    g_test_0005_execution_order[g_test_0005_execution_index++] = args->thread_id;
    g_test_0005_active_writers++;
    mutexUnlock(&g_test_0005_mutex);
    
    // Do some write work
    threadSleepMs(TEST_0005_WORK_DELAY_MS);
    
    // Update counters
    mutexLock(&g_test_0005_mutex);
    g_test_0005_active_writers--;
    g_test_0005_completed_writers++;
    mutexUnlock(&g_test_0005_mutex);
    
    // Release write lock
    rwlockWriteUnlock(&g_test_0005_rwlock);
}

/**
 * Test reader-writer priority scenarios and starvation prevention.
 */
test_rc_t test_0005_rwlock_reader_writer_priority(void) {
    Result rc = 0;

    //* Given
    // Initialize the test global rwlock and mutex
    rwlockInit(&g_test_0005_rwlock);
    mutexInit(&g_test_0005_mutex);

    // Create thread arguments with staggered start times
    Test0005_ThreadArgs reader_args[TEST_0005_NUM_READERS] = {
        {.thread_id = 1, .is_writer = false, .start_delay_ms = 50},  // Reader 1
        {.thread_id = 2, .is_writer = false, .start_delay_ms = 200}, // Reader 2 (after writer 1)
        {.thread_id = 3, .is_writer = false, .start_delay_ms = 350}  // Reader 3 (after writer 2)
    };
    
    Test0005_ThreadArgs writer_args[TEST_0005_NUM_WRITERS] = {
        {.thread_id = 101, .is_writer = true, .start_delay_ms = 100}, // Writer 1 (after reader 1)
        {.thread_id = 102, .is_writer = true, .start_delay_ms = 250}  // Writer 2 (after reader 2)
    };

    // Create reader threads
    Thread reader_threads[TEST_0005_NUM_READERS];
    for (int i = 0; i < TEST_0005_NUM_READERS; i++) {
        rc = threadCreate(&reader_threads[i], test_0005_rwlock_reader_thread_func, &reader_args[i], NULL, 0x10000, 0x2C, -2);
        if (R_FAILED(rc)) {
            goto test_cleanup;
        }
    }

    // Create writer threads
    Thread writer_threads[TEST_0005_NUM_WRITERS];
    for (int i = 0; i < TEST_0005_NUM_WRITERS; i++) {
        rc = threadCreate(&writer_threads[i], test_0005_rwlock_writer_thread_func, &writer_args[i], NULL, 0x10000, 0x2C, -2);
        if (R_FAILED(rc)) {
            goto test_cleanup;
        }
    }

    //* When
    // Start all threads
    for (int i = 0; i < TEST_0005_NUM_READERS; i++) {
        rc = threadStart(&reader_threads[i]);
        if (R_FAILED(rc)) {
            goto test_cleanup;
        }
    }
    
    for (int i = 0; i < TEST_0005_NUM_WRITERS; i++) {
        rc = threadStart(&writer_threads[i]);
        if (R_FAILED(rc)) {
            goto test_cleanup;
        }
    }
    
    // Wait for all threads to complete
    threadSleepMs(600); // Enough time for all operations
    
    mutexLock(&g_test_0005_mutex);
    const uint32_t completed_readers = g_test_0005_completed_readers;
    const uint32_t completed_writers = g_test_0005_completed_writers;
    const uint32_t execution_index = g_test_0005_execution_index;
    mutexUnlock(&g_test_0005_mutex);

    //* Then
    // Assert that all readers completed
    if (completed_readers != TEST_0005_NUM_READERS) {
        rc = TEST_ASSERTION_FAILED;
        goto test_cleanup;
    }
    
    // Assert that all writers completed
    if (completed_writers != TEST_0005_NUM_WRITERS) {
        rc = TEST_ASSERTION_FAILED;
        goto test_cleanup;
    }
    
    // Assert that execution order is recorded
    if (execution_index != TEST_0005_NUM_READERS + TEST_0005_NUM_WRITERS) {
        rc = TEST_ASSERTION_FAILED;
        goto test_cleanup;
    }
    
    // Verify basic ordering: reader 1 should execute first (smallest delay)
    if (g_test_0005_execution_order[0] != 1) {
        rc = TEST_ASSERTION_FAILED;
        goto test_cleanup;
    }

    //* Clean-up
test_cleanup:
    for (int i = 0; i < TEST_0005_NUM_READERS; i++) {
        threadWaitForExit(&reader_threads[i]);
        threadClose(&reader_threads[i]);
    }
    
    for (int i = 0; i < TEST_0005_NUM_WRITERS; i++) {
        threadWaitForExit(&writer_threads[i]);
        threadClose(&writer_threads[i]);
    }

    return rc;
}

//</editor-fold>

//<editor-fold desc="Test 0006: RwLock try operations">

#define TEST_0006_HOLDING_DELAY_MS 150

static RwLock g_test_0006_rwlock;
static bool g_test_0006_read_try_success = false;
static bool g_test_0006_write_try_success = false;
static bool g_test_0006_read_try_blocked = false;
static bool g_test_0006_write_try_blocked = false;

/**
 * Writer thread function for Test #0006 - holds write lock
 */
void test_0006_rwlock_writer_holding_thread_func(void *arg) {
    // Acquire write lock and hold it
    rwlockWriteLock(&g_test_0006_rwlock);
    
    // Hold the lock for a while
    threadSleepMs(TEST_0006_HOLDING_DELAY_MS);
    
    // Release write lock
    rwlockWriteUnlock(&g_test_0006_rwlock);
}

/**
 * Thread function for Test #0006 - tries read lock
 */
void test_0006_rwlock_try_read_thread_func(void *arg) {
    threadSleepMs(50); // Ensure writer has the lock
    
    // Try to acquire read lock (should fail while writer holds it)
    if (rwlockTryReadLock(&g_test_0006_rwlock)) {
        g_test_0006_read_try_success = true;
        rwlockReadUnlock(&g_test_0006_rwlock);
    } else {
        g_test_0006_read_try_blocked = true;
    }
}

/**
 * Thread function for Test #0006 - tries write lock
 */
void test_0006_rwlock_try_write_thread_func(void *arg) {
    threadSleepMs(75); // Ensure writer has the lock
    
    // Try to acquire write lock (should fail while another writer holds it)
    if (rwlockTryWriteLock(&g_test_0006_rwlock)) {
        g_test_0006_write_try_success = true;
        rwlockWriteUnlock(&g_test_0006_rwlock);
    } else {
        g_test_0006_write_try_blocked = true;
    }
}

/**
 * Test non-blocking try operations for both read and write locks.
 */
test_rc_t test_0006_rwlock_try_operations(void) {
    Result rc = 0;

    //* Given
    // Initialize the test global rwlock
    rwlockInit(&g_test_0006_rwlock);

    // Create threads
    Thread writer_holding_thread;
    Thread try_read_thread;
    Thread try_write_thread;

    rc = threadCreate(&writer_holding_thread, test_0006_rwlock_writer_holding_thread_func, NULL, NULL, 0x10000, 0x2C, -2);
    if (R_FAILED(rc)) {
        goto test_cleanup;
    }

    rc = threadCreate(&try_read_thread, test_0006_rwlock_try_read_thread_func, NULL, NULL, 0x10000, 0x2C, -2);
    if (R_FAILED(rc)) {
        goto test_cleanup;
    }

    rc = threadCreate(&try_write_thread, test_0006_rwlock_try_write_thread_func, NULL, NULL, 0x10000, 0x2C, -2);
    if (R_FAILED(rc)) {
        goto test_cleanup;
    }

    //* When
    // Start all threads
    rc = threadStart(&writer_holding_thread);
    if (R_FAILED(rc)) {
        goto test_cleanup;
    }

    rc = threadStart(&try_read_thread);
    if (R_FAILED(rc)) {
        goto test_cleanup;
    }

    rc = threadStart(&try_write_thread);
    if (R_FAILED(rc)) {
        goto test_cleanup;
    }

    // Wait for all threads to complete
    threadSleepMs(TEST_0006_HOLDING_DELAY_MS + 100);

    //* Then
    // Assert that try operations failed (lock was held)
    if (g_test_0006_read_try_success) {
        rc = TEST_ASSERTION_FAILED;
        goto test_cleanup;
    }

    if (g_test_0006_write_try_success) {
        rc = TEST_ASSERTION_FAILED;
        goto test_cleanup;
    }

    // Assert that try operations were properly blocked
    if (!g_test_0006_read_try_blocked) {
        rc = TEST_ASSERTION_FAILED;
        goto test_cleanup;
    }

    if (!g_test_0006_write_try_blocked) {
        rc = TEST_ASSERTION_FAILED;
        goto test_cleanup;
    }

    // Test successful try operations when no lock is held
    if (!rwlockTryReadLock(&g_test_0006_rwlock)) {
        rc = TEST_ASSERTION_FAILED;
        goto test_cleanup;
    }
    rwlockReadUnlock(&g_test_0006_rwlock);

    if (!rwlockTryWriteLock(&g_test_0006_rwlock)) {
        rc = TEST_ASSERTION_FAILED;
        goto test_cleanup;
    }
    rwlockWriteUnlock(&g_test_0006_rwlock);

    //* Clean-up
test_cleanup:
    threadWaitForExit(&writer_holding_thread);
    threadClose(&writer_holding_thread);
    
    threadWaitForExit(&try_read_thread);
    threadClose(&try_read_thread);
    
    threadWaitForExit(&try_write_thread);
    threadClose(&try_write_thread);

    return rc;
}

//</editor-fold>

//<editor-fold desc="Test 0007: RwLock write lock first unlock then read locks">

static RwLock g_test_0007_rwlock;
static Mutex g_test_0007_mutex;
static bool g_test_0007_write_acquired = false;
static uint32_t g_test_0007_read_acquired_count = 0;
static bool g_test_0007_all_locks_released = false;
static bool g_test_0007_success = false;

/**
 * Thread function for Test #0007 - Release write lock first, then read locks
 */
void test_0007_thread_func(void *arg) {
    // Acquire write lock
    rwlockWriteLock(&g_test_0007_rwlock);
    
    mutexLock(&g_test_0007_mutex);
    g_test_0007_write_acquired = true;
    mutexUnlock(&g_test_0007_mutex);
    
    // Acquire multiple read locks while holding write lock
    rwlockReadLock(&g_test_0007_rwlock);  // Read lock 1
    rwlockReadLock(&g_test_0007_rwlock);  // Read lock 2
    
    mutexLock(&g_test_0007_mutex);
    g_test_0007_read_acquired_count = 2;
    mutexUnlock(&g_test_0007_mutex);
    
    // Release write lock first
    rwlockWriteUnlock(&g_test_0007_rwlock);
    
    // Then release read locks
    rwlockReadUnlock(&g_test_0007_rwlock);  // Read unlock 1
    rwlockReadUnlock(&g_test_0007_rwlock);  // Read unlock 2
    
    mutexLock(&g_test_0007_mutex);
    g_test_0007_all_locks_released = true;
    g_test_0007_success = true;
    mutexUnlock(&g_test_0007_mutex);
}

/**
 * Test read locks while holding write lock - unlock write first.
 */
test_rc_t test_0007_rwlock_write_first_unlock(void) {
    Result rc = 0;

    //* Given
    // Initialize the test global rwlock and mutex
    rwlockInit(&g_test_0007_rwlock);
    mutexInit(&g_test_0007_mutex);
    
    // Reset state
    g_test_0007_write_acquired = false;
    g_test_0007_read_acquired_count = 0;
    g_test_0007_all_locks_released = false;
    g_test_0007_success = false;
    
    // Create thread
    Thread test_thread;
    rc = threadCreate(&test_thread, test_0007_thread_func, NULL, NULL, 0x10000, 0x2C, -2);
    if (R_FAILED(rc)) {
        goto test_cleanup;
    }

    //* When
    // Start the thread
    rc = threadStart(&test_thread);
    if (R_FAILED(rc)) {
        threadClose(&test_thread);
        goto test_cleanup;
    }

    // Wait for thread to complete all operations
    threadSleepMs(100);

    //* Then
    mutexLock(&g_test_0007_mutex);
    const bool write_acquired = g_test_0007_write_acquired;
    const uint32_t read_count = g_test_0007_read_acquired_count;
    const bool all_released = g_test_0007_all_locks_released;
    const bool success = g_test_0007_success;
    mutexUnlock(&g_test_0007_mutex);

    // Assert that write lock was acquired
    if (!write_acquired) {
        rc = TEST_ASSERTION_FAILED;
        threadWaitForExit(&test_thread);
        threadClose(&test_thread);
        goto test_cleanup;
    }

    // Assert that read locks were acquired
    if (read_count != 2) {
        rc = TEST_ASSERTION_FAILED;
        threadWaitForExit(&test_thread);
        threadClose(&test_thread);
        goto test_cleanup;
    }

    // Assert that all locks were properly released
    if (!all_released) {
        rc = TEST_ASSERTION_FAILED;
        threadWaitForExit(&test_thread);
        threadClose(&test_thread);
        goto test_cleanup;
    }

    // Assert that test completed successfully
    if (!success) {
        rc = TEST_ASSERTION_FAILED;
        threadWaitForExit(&test_thread);
        threadClose(&test_thread);
        goto test_cleanup;
    }

    // Clean up thread
    threadWaitForExit(&test_thread);
    threadClose(&test_thread);

    // Verify lock is available
    if (!rwlockTryWriteLock(&g_test_0007_rwlock)) {
        rc = TEST_ASSERTION_FAILED;
        goto test_cleanup;
    }
    rwlockWriteUnlock(&g_test_0007_rwlock);

test_cleanup:
    return rc;
}

//</editor-fold>

//<editor-fold desc="Test 0008: RwLock read locks first unlock then write lock">

static RwLock g_test_0008_rwlock;
static Mutex g_test_0008_mutex;
static bool g_test_0008_write_acquired = false;
static uint32_t g_test_0008_read_acquired_count = 0;
static bool g_test_0008_all_locks_released = false;
static bool g_test_0008_success = false;

/**
 * Thread function for Test #0008 - Release read locks first, then write lock
 */
void test_0008_thread_func(void *arg) {
    // Acquire write lock
    rwlockWriteLock(&g_test_0008_rwlock);
    
    mutexLock(&g_test_0008_mutex);
    g_test_0008_write_acquired = true;
    mutexUnlock(&g_test_0008_mutex);
    
    // Acquire multiple read locks while holding write lock
    rwlockReadLock(&g_test_0008_rwlock);  // Read lock 1
    rwlockReadLock(&g_test_0008_rwlock);  // Read lock 2
    rwlockReadLock(&g_test_0008_rwlock);  // Read lock 3
    
    mutexLock(&g_test_0008_mutex);
    g_test_0008_read_acquired_count = 3;
    mutexUnlock(&g_test_0008_mutex);
    
    // Release read locks first
    rwlockReadUnlock(&g_test_0008_rwlock);  // Read unlock 1
    rwlockReadUnlock(&g_test_0008_rwlock);  // Read unlock 2
    rwlockReadUnlock(&g_test_0008_rwlock);  // Read unlock 3
    
    // Then release write lock
    rwlockWriteUnlock(&g_test_0008_rwlock);
    
    mutexLock(&g_test_0008_mutex);
    g_test_0008_all_locks_released = true;
    g_test_0008_success = true;
    mutexUnlock(&g_test_0008_mutex);
}

/**
 * Test read locks while holding write lock - unlock reads first.
 */
test_rc_t test_0008_rwlock_reads_first_unlock(void) {
    Result rc = 0;

    //* Given
    // Initialize the test global rwlock and mutex
    rwlockInit(&g_test_0008_rwlock);
    mutexInit(&g_test_0008_mutex);
    
    // Reset state
    g_test_0008_write_acquired = false;
    g_test_0008_read_acquired_count = 0;
    g_test_0008_all_locks_released = false;
    g_test_0008_success = false;
    
    // Create thread
    Thread test_thread;
    rc = threadCreate(&test_thread, test_0008_thread_func, NULL, NULL, 0x10000, 0x2C, -2);
    if (R_FAILED(rc)) {
        goto test_cleanup;
    }

    //* When
    // Start the thread
    rc = threadStart(&test_thread);
    if (R_FAILED(rc)) {
        threadClose(&test_thread);
        goto test_cleanup;
    }

    // Wait for thread to complete all operations
    threadSleepMs(100);

    //* Then
    mutexLock(&g_test_0008_mutex);
    const bool write_acquired = g_test_0008_write_acquired;
    const uint32_t read_count = g_test_0008_read_acquired_count;
    const bool all_released = g_test_0008_all_locks_released;
    const bool success = g_test_0008_success;
    mutexUnlock(&g_test_0008_mutex);

    // Assert that write lock was acquired
    if (!write_acquired) {
        rc = TEST_ASSERTION_FAILED;
        threadWaitForExit(&test_thread);
        threadClose(&test_thread);
        goto test_cleanup;
    }

    // Assert that read locks were acquired
    if (read_count != 3) {
        rc = TEST_ASSERTION_FAILED;
        threadWaitForExit(&test_thread);
        threadClose(&test_thread);
        goto test_cleanup;
    }

    // Assert that all locks were properly released
    if (!all_released) {
        rc = TEST_ASSERTION_FAILED;
        threadWaitForExit(&test_thread);
        threadClose(&test_thread);
        goto test_cleanup;
    }

    // Assert that test completed successfully
    if (!success) {
        rc = TEST_ASSERTION_FAILED;
        threadWaitForExit(&test_thread);
        threadClose(&test_thread);
        goto test_cleanup;
    }

    // Clean up thread
    threadWaitForExit(&test_thread);
    threadClose(&test_thread);

    // Verify lock is available
    if (!rwlockTryReadLock(&g_test_0008_rwlock)) {
        rc = TEST_ASSERTION_FAILED;
        goto test_cleanup;
    }
    rwlockReadUnlock(&g_test_0008_rwlock);

test_cleanup:
    return rc;
}

//</editor-fold>

//<editor-fold desc="Test 0009: RwLock mixed unlock order">

static RwLock g_test_0009_rwlock;
static Mutex g_test_0009_mutex;
static bool g_test_0009_write_acquired = false;
static uint32_t g_test_0009_read_acquired_count = 0;
static bool g_test_0009_all_locks_released = false;
static bool g_test_0009_success = false;

/**
 * Thread function for Test #0009 - Mixed release order
 */
void test_0009_thread_func(void *arg) {
    // Acquire write lock
    rwlockWriteLock(&g_test_0009_rwlock);
    
    mutexLock(&g_test_0009_mutex);
    g_test_0009_write_acquired = true;
    mutexUnlock(&g_test_0009_mutex);
    
    // Acquire read locks while holding write lock
    rwlockReadLock(&g_test_0009_rwlock);  // Read lock 1
    rwlockReadLock(&g_test_0009_rwlock);  // Read lock 2
    
    mutexLock(&g_test_0009_mutex);
    g_test_0009_read_acquired_count = 2;
    mutexUnlock(&g_test_0009_mutex);
    
    // Mixed release order: read, write, read
    rwlockReadUnlock(&g_test_0009_rwlock);  // Release read lock 1
    rwlockWriteUnlock(&g_test_0009_rwlock); // Release write lock
    rwlockReadUnlock(&g_test_0009_rwlock);  // Release read lock 2
    
    mutexLock(&g_test_0009_mutex);
    g_test_0009_all_locks_released = true;
    g_test_0009_success = true;
    mutexUnlock(&g_test_0009_mutex);
}

/**
 * Test read locks while holding write lock - mixed unlock order.
 */
test_rc_t test_0009_rwlock_mixed_unlock_order(void) {
    Result rc = 0;

    //* Given
    // Initialize the test global rwlock and mutex
    rwlockInit(&g_test_0009_rwlock);
    mutexInit(&g_test_0009_mutex);
    
    // Reset state
    g_test_0009_write_acquired = false;
    g_test_0009_read_acquired_count = 0;
    g_test_0009_all_locks_released = false;
    g_test_0009_success = false;
    
    // Create thread
    Thread test_thread;
    rc = threadCreate(&test_thread, test_0009_thread_func, NULL, NULL, 0x10000, 0x2C, -2);
    if (R_FAILED(rc)) {
        goto test_cleanup;
    }

    //* When
    // Start the thread
    rc = threadStart(&test_thread);
    if (R_FAILED(rc)) {
        threadClose(&test_thread);
        goto test_cleanup;
    }

    // Wait for thread to complete all operations
    threadSleepMs(100);

    //* Then
    mutexLock(&g_test_0009_mutex);
    const bool write_acquired = g_test_0009_write_acquired;
    const uint32_t read_count = g_test_0009_read_acquired_count;
    const bool all_released = g_test_0009_all_locks_released;
    const bool success = g_test_0009_success;
    mutexUnlock(&g_test_0009_mutex);

    // Assert that write lock was acquired
    if (!write_acquired) {
        rc = TEST_ASSERTION_FAILED;
        threadWaitForExit(&test_thread);
        threadClose(&test_thread);
        goto test_cleanup;
    }

    // Assert that read locks were acquired
    if (read_count != 2) {
        rc = TEST_ASSERTION_FAILED;
        threadWaitForExit(&test_thread);
        threadClose(&test_thread);
        goto test_cleanup;
    }

    // Assert that all locks were properly released
    if (!all_released) {
        rc = TEST_ASSERTION_FAILED;
        threadWaitForExit(&test_thread);
        threadClose(&test_thread);
        goto test_cleanup;
    }

    // Assert that test completed successfully
    if (!success) {
        rc = TEST_ASSERTION_FAILED;
        threadWaitForExit(&test_thread);
        threadClose(&test_thread);
        goto test_cleanup;
    }

    // Clean up thread
    threadWaitForExit(&test_thread);
    threadClose(&test_thread);

    // Verify lock is available
    if (!rwlockTryWriteLock(&g_test_0009_rwlock)) {
        rc = TEST_ASSERTION_FAILED;
        goto test_cleanup;
    }
    rwlockWriteUnlock(&g_test_0009_rwlock);

test_cleanup:
    return rc;
}

//</editor-fold>

//</editor-fold> 
