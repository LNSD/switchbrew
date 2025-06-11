/* Static linker script for thread function symbols redirection */
/* Redirects libnx thread functions to nx-thread __nx_thread_* implementations */

/* Thread Local Storage (TLS) */
EXTERN(__nx_thread_get_tls);

armGetTls = __nx_thread_get_tls;

/* Thread Variables */
EXTERN(__nx_thread_get_thread_vars);
EXTERN(__nx_thread_get_current_thread_handle);

getThreadVars = __nx_thread_get_thread_vars;
threadGetCurHandle = __nx_thread_get_current_thread_handle;
