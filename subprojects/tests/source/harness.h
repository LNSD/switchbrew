#pragma once

#include <stdint.h>
#include <stdio.h>

/**
 * @brief The result code for a test case.
 */
typedef int32_t test_rc_t;

/**
 * @brief The result code for a test case.
 */
#define TEST_SUCCESS ((test_rc_t)0)

/**
 * @brief The failure code for a test case for an assertion failure.
 */
#define TEST_ASSERTION_FAILED ((test_rc_t)-101)


#define TEST_SUITE(suite_name) \
    printf(CONSOLE_CYAN "TEST SUITE:" CONSOLE_RESET " " suite_name "\n\n")

#define TEST_CASE(test_title, test_func) \
    { \
        printf(test_title ": "); \
        test_rc_t test_res = test_func(); \
        if (test_res == TEST_SUCCESS) { \
            printf(CONSOLE_GREEN "OK" CONSOLE_RESET "\n"); \
        } else { \
            printf(CONSOLE_RED "FAILED" CONSOLE_RESET " (%d)\n", test_res); \
        } \
    }
