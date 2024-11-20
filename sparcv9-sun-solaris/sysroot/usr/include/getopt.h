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
 * Copyright 2004 Sun Microsystems, Inc.  All rights reserved.
 * Use is subject to license terms.
 */

/*
 * GNU-like getopt_long(), getopt_long_only().
 * Solaris-specific getopt_clip().
 */

#ifndef	_GETOPT_H
#define	_GETOPT_H

/*
 * Values for has_arg field.
 *
 * optional_argument is not supported by getopt_clip()
 */
#define	no_argument		0
#define	required_argument	1
#define	optional_argument	2

struct option {
	const char *name; /* name of long option */
	int has_arg;	/* whether option takes an argument */
	int *flag;	/* if not NULL, set *flag to val when option found */
	int val;	/* if flag is not NULL, value to set *flag to. */
			/* if flag is NULL, return value */
};

#ifdef	__cplusplus
extern "C" {
#endif
/*
 * The use of getopt_long_only in new development is strongly discouraged.
 */
extern int	getopt_long(int, char * const *, const char *,
		    const struct option *, int *);
extern int	getopt_long_only(int, char * const *, const char *,
		    const struct option *, int *);
extern int	getopt_clip(int, char * const *, const char *,
		    const struct option *, int *);
#ifndef _GETOPT_DECLARED
#define	_GETOPT_DECLARED
int	 getopt(int, char * const [], const char *);
/*
 * External variables used by these routines.
 */
extern char *optarg;
extern int  opterr, optind, optopt;
#endif /* _GETOPT_DECLARED */
#ifndef _OPTRESET_DECLARED
#define	_OPTRESET_DECLARED
extern int optreset;			/* getopt(3) external variable */
#endif
#ifdef __cplusplus
}
#endif

#endif	/* _GETOPT_H */
