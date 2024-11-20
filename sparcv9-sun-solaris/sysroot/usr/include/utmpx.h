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
/*
 * Copyright 2014 Garrett D'Amore <garrett@damore.org>
 *
 * Copyright 1997 Sun Microsystems, Inc.  All rights reserved.
 * Use is subject to license terms.
 */

/*	Copyright (c) 1983,1984,1985,1986,1987,1988,1989 AT&T	*/
/*	  All Rights Reserved	*/

/*
 * Portions of this source code were derived from Berkeley 4.3 BSD
 * under license from the Regents of the University of California.
 */

#ifndef _UTMPX_H
#define	_UTMPX_H

#include <sys/feature_tests.h>
#include <sys/types.h>
#include <sys/time.h>
#include <utmp.h>

#ifdef	__cplusplus
extern "C" {
#endif

#define	_UTMPX_FILE	"/var/log/utmpx"
#define	_WTMPX_FILE	"/var/log/wtmpx"
#if !defined(_XPG4_2) || defined(__EXTENSIONS__)
#define	UTMPX_FILE	_UTMPX_FILE
#define	WTMPX_FILE	_WTMPX_FILE
#endif

#define	__UT_LINESIZE	32
#define	__UT_NAMESIZE	32
#define	__UT_HOSTSIZE	256

/*
 * This data structure describes the utmpx entries returned by
 * the getutxent(3c) family of APIs.  It does not (necessarily)
 * correspond to the contents of the utmpx or wtmpx files.
 *
 * Applications should only interact with this subsystem via
 * the getutxent(3c) family of APIs.
 */
struct utmpx {
	char ut_user[__UT_NAMESIZE];	/* Username.  */
	char ut_id[4];			/* Inittab ID.  */
	char ut_line[__UT_LINESIZE];	/* Devicename.  */
	pid_t ut_pid;			/* Process ID of login process.  */
	short int ut_type;		/* Type of login.  */
#if !defined(_XPG4_2) || defined(__EXTENSIONS__)
	struct exit_status ut_exit;	/* Exit status of a process marked */
					/* as DEAD_PROCESS.  */
#else
	struct ut_exit_status ut_exit;	/* Exit status of a process marked */
					/* as DEAD_PROCESS.  */
#endif
	struct timeval ut_tv;		/* Time entry was made.  */
	int ut_session;			/* Session ID, used for windowing.  */
	char ut_host[__UT_HOSTSIZE];	/* Hostname for remote login.  */
	int32_t ut_addr_v6[4];		/* Internet address of remote host.  */
	char __libc_reserved[20];	/* Reserved for future use.  */
};

#if !defined(_XPG4_2) || defined(__EXTENSIONS__)

#include <sys/types32.h>
#include <inttypes.h>

/*
 * This data structure describes the utmp *file* contents using
 * fixed-width data types.  It should only be used by the implementation.
 *
 * Applications should use the getutxent(3c) family of routines to interact
 * with this database.
 */

struct futmpx {
	char ut_user[__UT_NAMESIZE];	/* Username.  */
	char ut_id[4];			/* Inittab ID.  */
	char ut_line[__UT_LINESIZE];	/* Devicename.  */
	pid32_t ut_pid;			/* Process ID of login process.  */
	int16_t ut_type;		/* Type of login.  */
	struct {
		int16_t	e_termination;	/* process termination status */
		int16_t	e_exit;		/* process exit status */
	} ut_exit;			/* exit status of a process */
	struct timeval32 ut_tv;		/* Time entry was made.  */
	int32_t ut_session;		/* Session ID, used for windowing.  */
	char ut_host[__UT_HOSTSIZE];	/* Hostname for remote login.  */
	int32_t ut_addr_v6[4];		/* Internet address of remote host.  */
	char __libc_reserved[20];	/* Reserved for future use.  */
};

#define	MOD_WIN		10

/*	Define and macro for determing if a normal user wrote the entry */
/*	and marking the utmpx entry as a normal user */
#define	NONROOT_USRX	2
#define	nonuserx(utx)	((utx).ut_exit.e_exit == NONROOT_USRX ? 1 : 0)
#define	setuserx(utx)	((utx).ut_exit.e_exit = NONROOT_USRX)

#endif /* !defined(_XPG4_2) || defined(__EXTENSIONS__) */

extern void endutxent(void);
extern struct utmpx *getutxent(void);
extern struct utmpx *getutxid(const struct utmpx *);
extern struct utmpx *getutxline(const struct utmpx *);
extern struct utmpx *pututxline(const struct utmpx *);
extern void setutxent(void);

#if !defined(_XPG4_2) || defined(__EXTENSIONS__)
extern int utmpxname(const char *);
extern struct utmpx *makeutx(const struct utmpx *);
extern struct utmpx *modutx(const struct utmpx *);
extern void getutmp(const struct utmpx *, struct utmp *);
extern void getutmpx(const struct utmp *, struct utmpx *);
extern void updwtmp(const char *, struct utmp *);
extern void updwtmpx(const char *, struct utmpx *);
#endif /* !defined(_XPG4_2) || defined(__EXTENSIONS__) */

#ifdef	__cplusplus
}
#endif

#endif	/* _UTMPX_H */
