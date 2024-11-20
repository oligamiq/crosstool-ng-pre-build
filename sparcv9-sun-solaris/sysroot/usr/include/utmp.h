/*
 * CDDL HEADER START
 *
 * The contents of this file are subject to the terms of the
 * Common Development and Distribution License, Version 1.0 only
 * (the "License").  You may not use this file except in compliance
 * with the License.
 *
 * You can obtain a copy of the license at usr/src/OPENSOLARIS.LICENSE
 * or http://www.opensolaris.org/os/licensing.
 * See the License for the specific language governing permissions
 * and limitations under the License.
 *
 * When distributing Covered Code, include this CDDL HEADER in each
 * file and include the License file at usr/src/OPENSOLARIS.LICENSE.
 * If applicable, add the following below this CDDL HEADER, with the
 * fields enclosed by brackets "[]" replaced with your own identifying
 * information: Portions Copyright [yyyy] [name of copyright owner]
 *
 * CDDL HEADER END
 */
/*	Copyright (c) 1988 AT&T	*/
/*	  All Rights Reserved	*/


/*
 * Copyright 2014 Garrett D'Amore <garrett@damore.org>
 *
 * Copyright 2004 Sun Microsystems, Inc.  All rights reserved.
 * Use is subject to license terms.
 */


#ifndef _UTMP_H
#define	_UTMP_H

/*
 * Note:  The getutent(3c) family of interfaces are obsolete.
 * The getutxent(3c) family provide a superset of this functionality
 * and should be used in place of getutent(3c).
 */

#include <sys/types.h>

#ifdef	__cplusplus
extern "C" {
#endif

#if !defined(_XPG4_2) || defined(__EXTENSIONS__)
#define	UTMP_FILE	"/var/log/utmp"
#define	WTMP_FILE	"/var/log/wtmp"
#endif

#ifdef	_LP64
#define	UT_LINESIZE	32
#define	UT_NAMESIZE	32
#define	UT_HOSTSIZE	256
#else
#define	UT_LINESIZE	8
#define	UT_NAMESIZE	8
#define	UT_HOSTSIZE	16
#endif

/*
 * The structure describing an entry in the database of
 * previous logins.
 */
struct lastlog
{
#ifdef	_LP64
    int32_t ll_time;
#else
    time_t ll_time;
#endif
    char ll_line[UT_LINESIZE];
    char ll_host[UT_HOSTSIZE];
};

#if !defined(_XPG4_2) || defined(__EXTENSIONS__)
struct exit_status {
	short e_termination;	/* Process termination status */
	short e_exit;		/* Process exit status */
};
#else
struct ut_exit_status {
	short ut_e_termination;	/* Process termination status */
	short ut_e_exit;	/* Process exit status */
};
#endif /* !defined(_XPG4_2) || defined(__EXTENSIONS__) */

#if !defined(_XPG4_2) || defined(__EXTENSIONS__)

/*
 * This data structure describes the utmp entries returned by
 * the getutent(3c) family of APIs.  It does not (necessarily)
 * correspond to the contents of the utmp or wtmp files.
 *
 * Applications should only interact with this subsystem via
 * the getutxent(3c) family of APIs, as the getutent(3c) family
 * are obsolete.
 */
struct utmp {
	char ut_user[UT_NAMESIZE];	/* Username.  */
	char ut_id[4];			/* Inittab ID.  */
	char ut_line[UT_LINESIZE];	/* Devicename.  */
	pid_t ut_pid;			/* Process ID of login process.  */
	short int ut_type;		/* Type of login.  */
	struct exit_status ut_exit;	/* Exit status of a process marked */
					/*  as DEAD_PROCESS.  */
	int ut_session;			/* Session ID, used for windowing.  */
	struct timeval ut_tv;		/* Time entry was made.  */
	char ut_host[UT_HOSTSIZE];	/* Hostname for remote login.  */
	int32_t ut_addr_v6[4];		/* Internet address of remote host.  */
	char __libc_reserved[20];	/* Reserved for future use.  */
};

/* Backwards compatibility hacks.  */
#define	ut_name	ut_user
#ifndef	_NO_UT_TIME
/*
 * We have a problem here: 'ut_time' is also used otherwise.  Define
 * _NO_UT_TIME if the compiler complains.
 */
#define	ut_time		ut_tv.tv_sec
#endif
#define	ut_xtime	ut_tv.tv_sec
#define	ut_addr		ut_addr_v6[0]

#include <sys/types32.h>
#include <inttypes.h>

/*
 * This data structure describes the utmp *file* contents using
 * fixed-width data types.  It should only be used by the implementation.
 *
 * Applications should use the getutxent(3c) family of routines to interact
 * with this database.
 */

struct futmp {
	char ut_user[UT_NAMESIZE];	/* Username.  */
	char ut_id[4];			/* Inittab ID.  */
	char ut_line[UT_LINESIZE];	/* Devicename.  */
	int16_t ut_pid;			/* Process ID of login process.  */
	int16_t ut_type;		/* Type of login.  */
	struct {
		int16_t	e_termination;	/* Process termination status */
		int16_t e_exit;		/* Process exit status */
	} ut_exit;			/* The exit status of a process */
	struct timeval32 ut_tv;		/* Time entry was made.  */
	int32_t ut_session;		/* Session ID, used for windowing.  */
	char ut_host[UT_HOSTSIZE];	/* Hostname for remote login.  */
	int32_t ut_addr_v6[4];		/* Internet address of remote host.  */
	char __libc_reserved[20];	/* Reserved for future use.  */
};

#endif /* !defined(_XPG4_2) || defined(__EXTENSIONS__) */

/*	Definitions for ut_type						*/

#define	EMPTY		0
#define	RUN_LVL		1
#define	BOOT_TIME	2
#define	OLD_TIME	3
#define	NEW_TIME	4
#define	INIT_PROCESS	5	/* Process spawned by "init" */
#define	LOGIN_PROCESS	6	/* A "getty" process waiting for login */
#define	USER_PROCESS	7	/* A user process */
#define	DEAD_PROCESS	8

#if !defined(_XPG4_2) || defined(__EXTENSIONS__)

#define	ACCOUNTING	9
#define	DOWN_TIME	10

#define	UTMAXTYPE	DOWN_TIME	/* Largest legal value of ut_type */

/*
 * Tell the user that we have a modern system with UT_HOST, UT_PID,
 * UT_TYPE, UT_ID and UT_TV fields.
 */
#define	_HAVE_UT_TYPE	1
#define	_HAVE_UT_PID	1
#define	_HAVE_UT_ID	1
#define	_HAVE_UT_TV	1
#define	_HAVE_UT_HOST	1

/*	Special strings or formats used in the "ut_line" field when	*/
/*	accounting for something other than a process.			*/
/*	No string for the ut_line field can be more than 11 chars +	*/
/*	a NULL in length.						*/

#define	RUNLVL_MSG	"run-level %c"
#define	BOOT_MSG	"system boot"
#define	OTIME_MSG	"old time"
#define	NTIME_MSG	"new time"
#define	PSRADM_MSG	"%03d  %s"	/* processor on or off */
#define	DOWN_MSG	"system down"

/*	Define and macro for determing if a normal user wrote the entry */
/*	 and marking the utmpx entry as a normal user */
#define	NONROOT_USR	2
#define	nonuser(ut)	((ut).ut_exit.e_exit == NONROOT_USR ? 1 : 0)
#define	setuser(ut)	((ut).ut_exit.e_exit = NONROOT_USR)


extern void endutent(void);
extern struct utmp *getutent(void);
extern struct utmp *getutid(const struct utmp *);
extern struct utmp *getutline(const struct utmp *);
extern struct utmp *pututline(const struct utmp *);
extern void setutent(void);
extern int utmpname(const char *);

#endif /* !defined(_XPG4_2) || defined(__EXTENSIONS__) */

#ifdef	__cplusplus
}
#endif

#endif	/* _UTMP_H */
