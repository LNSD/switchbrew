/* Static linker script for hbmenu launch function symbols redirection */
/* Redirects homebrew launch functions to __nx_hbmenu_launch_* implementations */

/* Launch functions */
/* EXTERN(__nx_hbmenu_launch_init);
EXTERN(__nx_hbmenu_launch_exit);
EXTERN(__nx_hbmenu_launch_menu_entry);

launchInit = __nx_hbmenu_launch_init;
launchExit = __nx_hbmenu_launch_exit;
launchMenuEntry = __nx_hbmenu_launch_menu_entry; */