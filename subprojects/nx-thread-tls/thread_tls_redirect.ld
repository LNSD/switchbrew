/* Static linker script for thread function symbols redirection */
/* Redirects libnx thread functions to __nx_thread_tls_* implementations */

/* Thread Local Storage (TLS) */
EXTERN(__nx_thread_tls_get_ptr);

armGetTls = __nx_thread_tls_get_ptr;

/* Thread Variables */
EXTERN(__nx_thread_tls_get_thread_vars);
EXTERN(__nx_thread_tls_get_current_thread_handle);

getThreadVars = __nx_thread_tls_get_thread_vars;
threadGetCurHandle = __nx_thread_tls_get_current_thread_handle;
