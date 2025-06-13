/* Static linker script for random function symbols redirection */
/* Redirects libnx random functions to nx-rand __nx_rand_* implementations */

EXTERN(__nx_rand_get);
EXTERN(__nx_rand_get64);

randomGet = __nx_rand_get;
randomGet64 = __nx_rand_get64; 
