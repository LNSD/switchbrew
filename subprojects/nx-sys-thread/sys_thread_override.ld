/* Static linker script for thread function symbols redirection */
/* Redirects libnx thread functions to __nx_sys_thread_* implementations */

/* Thread Local Storage (TLS) */
EXTERN(__nx_sys_thread_get_ptr);
EXTERN(__nx_sys_thread_get_thread_vars);
EXTERN(__nx_sys_thread_get_tls_start_offset);

armGetTls         = __nx_sys_thread_get_ptr;
getThreadVars     = __nx_sys_thread_get_thread_vars;
getTlsStartOffset = __nx_sys_thread_get_tls_start_offset;

/* Thread */
EXTERN(__nx_sys_thread_start)
EXTERN(__nx_sys_thread_pause)
EXTERN(__nx_sys_thread_resume)
EXTERN(__nx_sys_thread_dump_context)
EXTERN(__nx_sys_thread_get_cur_handle);
EXTERN(__nx_sys_thread_wait_for_exit)
EXTERN(__nx_sys_thread_get_self);
EXTERN(__nx_sys_thread_tls_get);
EXTERN(__nx_sys_thread_tls_set);

threadStart        = __nx_sys_thread_start;
threadPause        = __nx_sys_thread_pause;
threadResume       = __nx_sys_thread_resume;
threadDumpContext  = __nx_sys_thread_dump_context;
threadGetCurHandle = __nx_sys_thread_get_cur_handle;
threadWaitForExit  = __nx_sys_thread_wait_for_exit;
threadGetSelf      = __nx_sys_thread_get_self;
threadTlsGet       = __nx_sys_thread_tls_get;
threadTlsSet       = __nx_sys_thread_tls_set;

/* libc (newlib) */
EXTERN(__nx_sys_thread_newlib_sleep)
EXTERN(__nx_sys_thread_newlib_usleep)
EXTERN(__nx_sys_thread_newlib_nanosleep)
EXTERN(__nx_sys_thread_newlib_pthread_setspecific)
EXTERN(__nx_sys_thread_newlib_pthread_getspecific)
EXTERN(__nx_sys_thread_newlib_sched_yield)
EXTERN(__nx_sys_thread_newlib_sched_getcpu)

sleep               = __nx_sys_thread_newlib_sleep;
usleep              = __nx_sys_thread_newlib_usleep;
nanosleep           = __nx_sys_thread_newlib_nanosleep;
pthread_setspecific = __nx_sys_thread_newlib_pthread_setspecific;
pthread_getspecific = __nx_sys_thread_newlib_pthread_getspecific;
sched_yield         = __nx_sys_thread_newlib_sched_yield;
sched_getcpu        = __nx_sys_thread_newlib_sched_getcpu;

/* Pthread functions - NOT YET IMPLEMENTED (todo!()) */
/* Uncomment as functions are implemented */

/* Pthread Mutex Functions */
/* EXTERN(__nx_sys_thread_newlib_pthread_mutex_init) */
/* EXTERN(__nx_sys_thread_newlib_pthread_mutex_destroy) */
/* EXTERN(__nx_sys_thread_newlib_pthread_mutex_lock) */
/* EXTERN(__nx_sys_thread_newlib_pthread_mutex_unlock) */
/* EXTERN(__nx_sys_thread_newlib_pthread_mutex_trylock) */
/* pthread_mutex_init    = __nx_sys_thread_newlib_pthread_mutex_init; */
/* pthread_mutex_destroy = __nx_sys_thread_newlib_pthread_mutex_destroy; */
/* pthread_mutex_lock    = __nx_sys_thread_newlib_pthread_mutex_lock; */
/* pthread_mutex_unlock  = __nx_sys_thread_newlib_pthread_mutex_unlock; */
/* pthread_mutex_trylock = __nx_sys_thread_newlib_pthread_mutex_trylock; */

/* Pthread Condition Variable Functions */
/* EXTERN(__nx_sys_thread_newlib_pthread_cond_init) */
/* EXTERN(__nx_sys_thread_newlib_pthread_cond_destroy) */
/* EXTERN(__nx_sys_thread_newlib_pthread_cond_wait) */
/* EXTERN(__nx_sys_thread_newlib_pthread_cond_timedwait) */
/* EXTERN(__nx_sys_thread_newlib_pthread_cond_signal) */
/* EXTERN(__nx_sys_thread_newlib_pthread_cond_broadcast) */
/* pthread_cond_init      = __nx_sys_thread_newlib_pthread_cond_init; */
/* pthread_cond_destroy   = __nx_sys_thread_newlib_pthread_cond_destroy; */
/* pthread_cond_wait      = __nx_sys_thread_newlib_pthread_cond_wait; */
/* pthread_cond_timedwait = __nx_sys_thread_newlib_pthread_cond_timedwait; */
/* pthread_cond_signal    = __nx_sys_thread_newlib_pthread_cond_signal; */
/* pthread_cond_broadcast = __nx_sys_thread_newlib_pthread_cond_broadcast; */

/* Pthread Thread Management Functions */
/* EXTERN(__nx_sys_thread_newlib_pthread_create) */
/* EXTERN(__nx_sys_thread_newlib_pthread_join) */
/* EXTERN(__nx_sys_thread_newlib_pthread_detach) */
/* EXTERN(__nx_sys_thread_newlib_pthread_self) */
/* EXTERN(__nx_sys_thread_newlib_pthread_exit) */
/* EXTERN(__nx_sys_thread_newlib_pthread_equal) */
/* pthread_create = __nx_sys_thread_newlib_pthread_create; */
/* pthread_join   = __nx_sys_thread_newlib_pthread_join; */
/* pthread_detach = __nx_sys_thread_newlib_pthread_detach; */
/* pthread_self   = __nx_sys_thread_newlib_pthread_self; */
/* pthread_exit   = __nx_sys_thread_newlib_pthread_exit; */
/* pthread_equal  = __nx_sys_thread_newlib_pthread_equal; */

/* Pthread Attribute Functions */
/* EXTERN(__nx_sys_thread_newlib_pthread_attr_init) */
/* EXTERN(__nx_sys_thread_newlib_pthread_attr_destroy) */
/* EXTERN(__nx_sys_thread_newlib_pthread_attr_setstacksize) */
/* EXTERN(__nx_sys_thread_newlib_pthread_attr_getstacksize) */
/* EXTERN(__nx_sys_thread_newlib_pthread_attr_setdetachstate) */
/* EXTERN(__nx_sys_thread_newlib_pthread_attr_getdetachstate) */
/* pthread_attr_init            = __nx_sys_thread_newlib_pthread_attr_init; */
/* pthread_attr_destroy         = __nx_sys_thread_newlib_pthread_attr_destroy; */
/* pthread_attr_setstacksize    = __nx_sys_thread_newlib_pthread_attr_setstacksize; */
/* pthread_attr_getstacksize    = __nx_sys_thread_newlib_pthread_attr_getstacksize; */
/* pthread_attr_setdetachstate  = __nx_sys_thread_newlib_pthread_attr_setdetachstate; */
/* pthread_attr_getdetachstate  = __nx_sys_thread_newlib_pthread_attr_getdetachstate; */

/* Pthread Read-Write Lock Functions */
/* EXTERN(__nx_sys_thread_newlib_pthread_rwlock_init) */
/* EXTERN(__nx_sys_thread_newlib_pthread_rwlock_destroy) */
/* EXTERN(__nx_sys_thread_newlib_pthread_rwlock_rdlock) */
/* EXTERN(__nx_sys_thread_newlib_pthread_rwlock_wrlock) */
/* EXTERN(__nx_sys_thread_newlib_pthread_rwlock_unlock) */
/* EXTERN(__nx_sys_thread_newlib_pthread_rwlock_tryrdlock) */
/* EXTERN(__nx_sys_thread_newlib_pthread_rwlock_trywrlock) */
/* pthread_rwlock_init      = __nx_sys_thread_newlib_pthread_rwlock_init; */
/* pthread_rwlock_destroy   = __nx_sys_thread_newlib_pthread_rwlock_destroy; */
/* pthread_rwlock_rdlock    = __nx_sys_thread_newlib_pthread_rwlock_rdlock; */
/* pthread_rwlock_wrlock    = __nx_sys_thread_newlib_pthread_rwlock_wrlock; */
/* pthread_rwlock_unlock    = __nx_sys_thread_newlib_pthread_rwlock_unlock; */
/* pthread_rwlock_tryrdlock = __nx_sys_thread_newlib_pthread_rwlock_tryrdlock; */
/* pthread_rwlock_trywrlock = __nx_sys_thread_newlib_pthread_rwlock_trywrlock; */

/* Semaphore Functions */
/* EXTERN(__nx_sys_thread_newlib_sem_init) */
/* EXTERN(__nx_sys_thread_newlib_sem_destroy) */
/* EXTERN(__nx_sys_thread_newlib_sem_wait) */
/* EXTERN(__nx_sys_thread_newlib_sem_post) */
/* EXTERN(__nx_sys_thread_newlib_sem_trywait) */
/* sem_init    = __nx_sys_thread_newlib_sem_init; */
/* sem_destroy = __nx_sys_thread_newlib_sem_destroy; */
/* sem_wait    = __nx_sys_thread_newlib_sem_wait; */
/* sem_post    = __nx_sys_thread_newlib_sem_post; */
/* sem_trywait = __nx_sys_thread_newlib_sem_trywait; */

/* Pthread Barrier Functions */
/* EXTERN(__nx_sys_thread_newlib_pthread_barrier_init) */
/* EXTERN(__nx_sys_thread_newlib_pthread_barrier_destroy) */
/* EXTERN(__nx_sys_thread_newlib_pthread_barrier_wait) */
/* pthread_barrier_init    = __nx_sys_thread_newlib_pthread_barrier_init; */
/* pthread_barrier_destroy = __nx_sys_thread_newlib_pthread_barrier_destroy; */
/* pthread_barrier_wait    = __nx_sys_thread_newlib_pthread_barrier_wait; */

/* Pthread Once Functions */
/* EXTERN(__nx_sys_thread_newlib_pthread_once) */
/* pthread_once = __nx_sys_thread_newlib_pthread_once; */

/* Pthread Key Functions */
/* EXTERN(__nx_sys_thread_newlib_pthread_key_create) */
/* EXTERN(__nx_sys_thread_newlib_pthread_key_delete) */
/* pthread_key_create = __nx_sys_thread_newlib_pthread_key_create; */
/* pthread_key_delete = __nx_sys_thread_newlib_pthread_key_delete; */
