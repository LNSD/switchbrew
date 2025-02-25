#include <stdint.h>

#include <switch.h>

#include "../harness.h"

/**
* @brief Sleeps the current thread for the given number of milliseconds.
* @param ms The number of milliseconds to sleep.
*/
static inline void threadSleepMs(int64_t ms) {
    svcSleepThread(ms * 1000000);
}

//<editor-fold desc="Test 0001: Barrier sync multiple threads">

#define TEST_0001_NUM_THREADS 4
#define TEST_0001_THREAD_DELAY_MS 50

static Barrier g_test_0001_barrier;
static uint64_t g_test_0001_bitflags = 0b0000;

/**
 * Thread function for Test #0001
 */
void test_0001_barrier_thread_func(void* arg)
{
    uint64_t num = (uint64_t) arg;

    for (uint64_t i=0; i<2; i++)
    {
        // Delay the thread execution
        threadSleepMs((num + 1) * TEST_0001_THREAD_DELAY_MS);

        // Flip the bitflag for this thread
        g_test_0001_bitflags ^= (1 << num);

        // Wait for all threads to reach the barrier
        barrierWait(&g_test_0001_barrier);
    }
}

test_rc_t test_0001_barrier_sync_multiple_threads(void)
{
    Result rc = 0;

    //* Given
    // Initialize the test global barrier
    barrierInit(&g_test_0001_barrier, TEST_0001_NUM_THREADS);

    //* When
    // Create the threads
    static Thread thread[TEST_0001_NUM_THREADS];

    for (uint64_t i=0; i<TEST_0001_NUM_THREADS; i++) {
        rc = threadCreate(&thread[i], test_0001_barrier_thread_func, (void*)i, NULL, 0x10000, 0x2C, -2);
        if (R_FAILED(rc)) {
            goto test_cleanup;
        }
    }

    // Start the threads
    for (uint64_t i=0; i<TEST_0001_NUM_THREADS; i++) {
        rc = threadStart(&thread[i]);
        if (R_FAILED(rc)) {
            goto test_cleanup;
        }
    }

    int64_t t0 = 0;

    // T1: Wait for all threads to reach the barrier for the first time, and continue
    const int64_t t1 = t0 + (TEST_0001_NUM_THREADS * TEST_0001_THREAD_DELAY_MS) + 10; // t0 + 400ms (+ 10ms)
    threadSleepMs(t1 - t0);

    const int64_t barrier_count_t1 = g_test_0001_barrier.count;
    const int64_t bitflags_t1 = g_test_0001_bitflags;

    // T2: Wait for %50 of the threads to reach the barrier
    const int64_t t2 = t1 + ((TEST_0001_NUM_THREADS / 2) * TEST_0001_THREAD_DELAY_MS) + 10; // t1 + 200ms (+ 10ms)
    threadSleepMs(t2 - t1);

    const int64_t barrier_count_t2 = g_test_0001_barrier.count;
    const int64_t bitflags_t2 = g_test_0001_bitflags;

    // T3: Wait for the rest of the threads to reach the barrier, and continue
    const int64_t t3 = t1 + (TEST_0001_NUM_THREADS * TEST_0001_THREAD_DELAY_MS) + 10; // t1 + 400ms (+ 10ms)
    threadSleepMs(t3 - t2);

    const int64_t barrier_count_t3 = g_test_0001_barrier.count;
    const int64_t bitflags_t3 = g_test_0001_bitflags;

    //* Then
    // - T1
    // Assert the barrier count has been reset after all threads have reached the barrier
    if (barrier_count_t1 != 0) {
        rc = TEST_ASSERTION_FAILED;
        goto test_cleanup;
    }

    // Assert all bitflags have been set
    if (bitflags_t1 != 0b1111) {
        rc = TEST_ASSERTION_FAILED;
        goto test_cleanup;
    }

    // - T2
    // Assert that 50% of the threads have reached the barrier
    if (barrier_count_t2 != TEST_0001_NUM_THREADS / 2) {
        rc = TEST_ASSERTION_FAILED;
        goto test_cleanup;
    }

    // Assert that the first half of the bitflags have been reset
    if (bitflags_t2 != 0b1100) {
        rc = TEST_ASSERTION_FAILED;
        goto test_cleanup;
    }

    // - T3
    // Assert the barrier count has been reset after all threads have reached the barrier
    if (barrier_count_t3 != 0) {
        rc = TEST_ASSERTION_FAILED;
        goto test_cleanup;
    }

    // Assert all bitflags have been reset
    if (bitflags_t3 != 0b0000) {
        rc = TEST_ASSERTION_FAILED;
        goto test_cleanup;
    }

    //* Cleanup
test_cleanup:
    for (uint64_t i=0; i<TEST_0001_NUM_THREADS; i++) {
        threadWaitForExit(&thread[i]);
        threadClose(&thread[i]);
    }

    return rc;
}

//</editor-fold>