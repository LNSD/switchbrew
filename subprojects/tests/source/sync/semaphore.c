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

//<editor-fold desc="Test 0001: Semaphore wait signal single thread">

static Semaphore g_test_0001_semaphore;
static bool g_test_0001_task_completed = false;

/**
 * Thread function for Test #0001
 */
static void test_0001_thread_func(void *arg) {
    // Wait on the semaphore
    semaphoreWait(&g_test_0001_semaphore);
    
    // Set the task completed flag
    g_test_0001_task_completed = true;

    // Signal the semaphore again
    semaphoreSignal(&g_test_0001_semaphore);
}

/**
 * Test semaphore wait and signal in a single thread.
 */
test_rc_t test_0001_semaphore_wait_signal_single_thread(void) {
    Result rc = 0;

    //* Given
    // Initialize the test global semaphore with count 0
    semaphoreInit(&g_test_0001_semaphore, 0);

    // Create a thread
    Thread thread;
    rc = threadCreate(&thread, test_0001_thread_func, NULL, NULL, 0x10000, 0x2C, -2);
    if (R_FAILED(rc)) {
        goto test_cleanup;
    }

    //* When
    // Start the thread
    rc = threadStart(&thread);
    if (R_FAILED(rc)) {
        goto test_cleanup;
    }


    const int64_t t0 = 0;

    // T1: Sleep briefly to ensure thread is waiting on semaphore
    const int64_t t1 = t0 + 10; // t0 + 10ms
    threadSleepMs(t1 - t0);
    
    // Check that task is not completed yet (thread should be blocked)
    const bool task_completed_t1 = g_test_0001_task_completed;
    
    // Signal the semaphore to unblock the thread
    semaphoreSignal(&g_test_0001_semaphore);
    
    // T2: Sleep briefly to allow thread to complete its work
    const int64_t t2 = t1 + 10; // t1 + 10ms
    threadSleepMs(t2 - t1);
    
    // Check that task is now completed
    const bool task_completed_t2 = g_test_0001_task_completed;
    
    // Wait on the semaphore that the thread should have signaled
    semaphoreWait(&g_test_0001_semaphore);
    
    //* Then
    // - T1
    // Assert that the task was not completed before signaling
    if (task_completed_t1) {
        rc = TEST_ASSERTION_FAILED;
        goto test_cleanup;
    }
    
    // - T2
    // Assert that the task was completed after signaling
    if (!task_completed_t2) {
        rc = TEST_ASSERTION_FAILED;
        goto test_cleanup;
    }

    //* Clean-up
test_cleanup:
    threadWaitForExit(&thread);
    threadClose(&thread);

    return rc;
}

//</editor-fold>

//<editor-fold desc="Test 0002: Semaphore multiple threads initial count">

#define TEST_0002_NUM_THREADS 4
#define TEST_0002_SEMAPHORE_INITIAL_COUNT 2
#define TEST_0002_WORK_DELAY_MS 100

static Semaphore g_test_0002_semaphore;
static Mutex g_test_0002_mutex;
static uint32_t g_test_0002_active_threads = 0;
static uint32_t g_test_0002_completed_threads = 0;

/**
 * Thread function for Test #0002
 */
static void test_0002_thread_func(void *arg) {
    // Wait on the semaphore
    semaphoreWait(&g_test_0002_semaphore);
    
    // Increment active threads count using mutex for thread safety
    mutexLock(&g_test_0002_mutex);
    g_test_0002_active_threads++;
    mutexUnlock(&g_test_0002_mutex);
    
    // Do some work
    threadSleepMs(TEST_0002_WORK_DELAY_MS);
    
    // Update counters using mutex for thread safety
    mutexLock(&g_test_0002_mutex);
    g_test_0002_active_threads--;
    g_test_0002_completed_threads++;
    mutexUnlock(&g_test_0002_mutex);
    
    // Signal the semaphore to allow another thread to proceed
    semaphoreSignal(&g_test_0002_semaphore);
}

/**
 * This test creates multiple threads that wait on a semaphore with an initial count.
 * Each thread decrements the semaphore count and performs its work.
 */
test_rc_t test_0002_semaphore_multiple_threads_initial_count(void) {
    Result rc = 0;

    //* Given
    // Initialize the test global semaphore with initial count and mutex
    semaphoreInit(&g_test_0002_semaphore, TEST_0002_SEMAPHORE_INITIAL_COUNT);
    mutexInit(&g_test_0002_mutex);

    // Create threads
    Thread threads[TEST_0002_NUM_THREADS];
    for (int i = 0; i < TEST_0002_NUM_THREADS; i++) {
        rc = threadCreate(&threads[i], test_0002_thread_func, NULL, NULL, 0x10000, 0x2C, -2);
        if (R_FAILED(rc)) {
            goto test_cleanup;
        }
    }

    //* When
    // Start all threads
    for (int i = 0; i < TEST_0002_NUM_THREADS; i++) {
        rc = threadStart(&threads[i]);
        if (R_FAILED(rc)) {
            goto test_cleanup;
        }
    }
    
    const int64_t t0 = 0;

    // T1: Sleep briefly to allow threads to start
    const int64_t t1 = t0 + 10; // t0 + 10ms
    threadSleepMs(t1 - t0);

    // Check initial active threads (should match initial semaphore count)
    mutexLock(&g_test_0002_mutex);
    const uint32_t active_threads_t1 = g_test_0002_active_threads;
    const uint32_t completed_threads_t1 = g_test_0002_completed_threads;
    mutexUnlock(&g_test_0002_mutex);
    
    // T2: Wait for half the threads to complete
    const int64_t t2 = t1 + TEST_0002_WORK_DELAY_MS + 10; // t1 + 100ms + 10ms
    threadSleepMs(t2 - t1);

    mutexLock(&g_test_0002_mutex);
    const uint32_t active_threads_t2 = g_test_0002_active_threads;
    const uint32_t completed_threads_t2 = g_test_0002_completed_threads;
    mutexUnlock(&g_test_0002_mutex);

    // T3: Wait for the remaining threads to complete
    const int64_t t3 = t1 + 2 * TEST_0002_WORK_DELAY_MS + 10; // t1 + 200ms + 10ms
    threadSleepMs(t3 - t2);
    
    mutexLock(&g_test_0002_mutex);
    const uint32_t active_threads_t3 = g_test_0002_active_threads;
    const uint32_t completed_threads_t3 = g_test_0002_completed_threads;
    mutexUnlock(&g_test_0002_mutex);

    //* Then
    // - T1
    // Assert that initial active threads matches the semaphore count
    if (active_threads_t1 != TEST_0002_SEMAPHORE_INITIAL_COUNT || completed_threads_t1 != 0) {
        rc = TEST_ASSERTION_FAILED;
        goto test_cleanup;
    }

    // - T2
    // Assert that at the midpoint, we still have the same number of active threads
    // and some threads have completed
    if (active_threads_t2 != TEST_0002_SEMAPHORE_INITIAL_COUNT || completed_threads_t2 == 0) {
        rc = TEST_ASSERTION_FAILED;
        goto test_cleanup;
    }

    // - T3
    // Assert that all threads eventually completed
    if (active_threads_t3 != 0 || completed_threads_t3 != TEST_0002_NUM_THREADS) {
        rc = TEST_ASSERTION_FAILED;
        goto test_cleanup;
    }

    //* Clean-up
test_cleanup:
    for (int i = 0; i < TEST_0002_NUM_THREADS; i++) {
        threadWaitForExit(&threads[i]);
        threadClose(&threads[i]);
    }

    return rc;
}

//</editor-fold>

//<editor-fold desc="Test 0003: Semaphore producer consumer">

#define TEST_0003_NUM_PRODUCERS 2
#define TEST_0003_NUM_CONSUMERS 3
#define TEST_0003_BUFFER_SIZE 5
#define TEST_0003_ITEMS_PER_PRODUCER 4
#define TEST_0003_PRODUCER_DELAY_MS 30
#define TEST_0003_CONSUMER_DELAY_MS 50
#define TEST_0003_EXPECTED_PRODUCED (TEST_0003_NUM_PRODUCERS * TEST_0003_ITEMS_PER_PRODUCER)

static Semaphore g_test_0003_empty_semaphore;  // Counts empty slots in buffer
static Semaphore g_test_0003_full_semaphore;   // Counts filled slots in buffer
static Mutex g_test_0003_buffer_mutex;         // Protects access to the buffer
static uint32_t g_test_0003_buffer_count = 0;  // Number of items in buffer
static uint32_t g_test_0003_produced_count = 0; // Total items produced
static uint32_t g_test_0003_consumed_count = 0; // Total items consumed
static bool g_test_0003_producers_done = false; // Flag to signal consumers to exit

/**
 * Producer thread function for Test #0003
 */
static void test_0003_producer_thread_func(void *arg) {
    for (int i = 0; i < TEST_0003_ITEMS_PER_PRODUCER; i++) {
        // Simulate production time
        threadSleepMs(TEST_0003_PRODUCER_DELAY_MS);
        
        // Wait for an empty slot
        semaphoreWait(&g_test_0003_empty_semaphore);
        
        // Lock the buffer, add an item to the buffer, and unlock the buffer
        mutexLock(&g_test_0003_buffer_mutex);
        g_test_0003_buffer_count++;
        g_test_0003_produced_count++;
        mutexUnlock(&g_test_0003_buffer_mutex);
        
        // Signal that a slot is filled
        semaphoreSignal(&g_test_0003_full_semaphore);
    }
}

/**
 * Consumer thread function for Test #0003
 */
static void test_0003_consumer_thread_func(void *arg) {
    while (true) {
        // Check if producers are done and buffer is empty
        mutexLock(&g_test_0003_buffer_mutex);
        const bool should_exit = g_test_0003_producers_done && (g_test_0003_buffer_count == 0);
        mutexUnlock(&g_test_0003_buffer_mutex);
        
        if (should_exit) {
            return;
        }
        
        // Try to get an item without blocking
        if (!semaphoreTryWait(&g_test_0003_full_semaphore)) {
            // No items available, sleep briefly and try again
            threadSleepMs(10);
            continue;
        }
        
        // Lock the buffer
        mutexLock(&g_test_0003_buffer_mutex);
        
        // Remove item from buffer
        if (g_test_0003_buffer_count > 0) {
            g_test_0003_buffer_count--;
            g_test_0003_consumed_count++;
        }
        
        // Unlock the buffer
        mutexUnlock(&g_test_0003_buffer_mutex);
        
        // Signal that a slot is empty
        semaphoreSignal(&g_test_0003_empty_semaphore);
        
        // Simulate consumption time
        threadSleepMs(TEST_0003_CONSUMER_DELAY_MS);
    }
}

/**
 * This test creates multiple producer and consumer threads.
 * Producer threads signal the semaphore, and consumer threads wait on it.
 */
test_rc_t test_0003_semaphore_producer_consumer(void) {
    Result rc = 0;

    //* Given
    // Initialize the semaphores and mutex
    semaphoreInit(&g_test_0003_empty_semaphore, TEST_0003_BUFFER_SIZE);
    semaphoreInit(&g_test_0003_full_semaphore, 0);
    mutexInit(&g_test_0003_buffer_mutex);

    // Create producer threads
    Thread producers[TEST_0003_NUM_PRODUCERS];
    for (int i = 0; i < TEST_0003_NUM_PRODUCERS; i++) {
        rc = threadCreate(&producers[i], test_0003_producer_thread_func, NULL, NULL, 0x10000, 0x2C, -2);
        if (R_FAILED(rc)) {
            goto test_cleanup;
        }
    }
    
    // Create consumer threads
    Thread consumers[TEST_0003_NUM_CONSUMERS];
    for (int i = 0; i < TEST_0003_NUM_CONSUMERS; i++) {
        rc = threadCreate(&consumers[i], test_0003_consumer_thread_func, NULL, NULL, 0x10000, 0x2C, -2);
        if (R_FAILED(rc)) {
            goto test_cleanup;
        }
    }

    //* When
    // Start all producer threads
    for (int i = 0; i < TEST_0003_NUM_PRODUCERS; i++) {
        rc = threadStart(&producers[i]);
        if (R_FAILED(rc)) {
            goto test_cleanup;
        }
    }
    
    // Start all consumer threads
    for (int i = 0; i < TEST_0003_NUM_CONSUMERS; i++) {
        rc = threadStart(&consumers[i]);
        if (R_FAILED(rc)) {
            goto test_cleanup;
        }
    }
    
    // T1: Wait for all producer threads to complete
    for (int i = 0; i < TEST_0003_NUM_PRODUCERS; i++) {
        threadWaitForExit(&producers[i]);
    }
    
    // Signal consumers that producers are done
    mutexLock(&g_test_0003_buffer_mutex);
    g_test_0003_producers_done = true;
    mutexUnlock(&g_test_0003_buffer_mutex);
    
    // T2: Wait for all consumer threads to complete
    for (int i = 0; i < TEST_0003_NUM_CONSUMERS; i++) {
        threadWaitForExit(&consumers[i]);
    }
    
    mutexLock(&g_test_0003_buffer_mutex);
    const uint32_t total_produced = g_test_0003_produced_count;
    const uint32_t total_consumed = g_test_0003_consumed_count;
    const uint32_t items_in_buffer = g_test_0003_buffer_count;
    mutexUnlock(&g_test_0003_buffer_mutex);

    //* Then
    // Assert that the expected number of items were produced
    if (total_produced != TEST_0003_EXPECTED_PRODUCED) {
        rc = TEST_ASSERTION_FAILED;
        goto test_cleanup;
    }
    
    // Assert that all produced items were consumed
    if (total_consumed != TEST_0003_EXPECTED_PRODUCED) {
        rc = TEST_ASSERTION_FAILED;
        goto test_cleanup;
    }
    
    // Assert that the buffer is empty
    if (items_in_buffer != 0) {
        rc = TEST_ASSERTION_FAILED;
        goto test_cleanup;
    }

    //* Clean-up
test_cleanup:
    for (int i = 0; i < TEST_0003_NUM_PRODUCERS; i++) {
        threadWaitForExit(&producers[i]);
        threadClose(&producers[i]);
    }
    for (int i = 0; i < TEST_0003_NUM_CONSUMERS; i++) {
        threadWaitForExit(&consumers[i]);
        threadClose(&consumers[i]);
    }

    return rc;
}

//</editor-fold>
