#include <stdio.h>
#include <inttypes.h>
#include <switch.h>

#include "sync/mutex.h"

int main()
{
    consoleInit(NULL);

    // Configure our supported input layout: a single player with standard controller styles
    padConfigureInput(1, HidNpadStyleSet_NpadStandard);

    // Initialize the default gamepad (which reads handheld mode inputs as well as the first connected controller)
    PadState pad;
    padInitializeDefault(&pad);

    // Print the test header
    printf("NX-TESTS\n");
    printf("Press + to exit.\n\n");

    // Run the test suites
    // - sync/mutex
    sync_mutex_suite();

    // Main loop:
    // - Display the test results
    // - Wait for the user to press + to exit
    while(appletMainLoop())
    {
        padUpdate(&pad);

        const uint32_t key_down = padGetButtonsDown(&pad);
        if (key_down & HidNpadButton_Plus) {
            break;
        }

        consoleUpdate(NULL);
    }

    consoleExit(NULL);
    return 0;
}
