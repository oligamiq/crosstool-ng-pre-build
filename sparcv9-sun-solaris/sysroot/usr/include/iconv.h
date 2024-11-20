/*
 * SPDX-License-Identifier: BSD-2-Clause
 *
 * Copyright (c) 2003 Citrus Project,
 * Copyright (c) 2009, 2010 Gabor Kovesdan <gabor@FreeBSD.org>
 * All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 * 1. Redistributions of source code must retain the above copyright
 *    notice, this list of conditions and the following disclaimer.
 * 2. Redistributions in binary form must reproduce the above copyright
 *    notice, this list of conditions and the following disclaimer in the
 *    documentation and/or other materials provided with the distribution.
 *
 * THIS SOFTWARE IS PROVIDED BY THE AUTHOR AND CONTRIBUTORS ``AS IS'' AND
 * ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
 * IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE
 * ARE DISCLAIMED.  IN NO EVENT SHALL THE AUTHOR OR CONTRIBUTORS BE LIABLE
 * FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
 * DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS
 * OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION)
 * HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT
 * LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY
 * OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF
 * SUCH DAMAGE.
 *
 */

#ifndef	_ICONV_H
#define	_ICONV_H

#include <sys/types.h>

#ifdef __cplusplus
extern "C" {
#endif

typedef	void	*iconv_t;

iconv_t	iconv_open(const char *, const char *);
size_t	iconv(iconv_t, const char ** __restrict,
		size_t * __restrict, char ** __restrict,
		size_t * __restrict);
int	iconv_close(iconv_t);

/*
 * non-portable interfaces for iconv
 */
size_t  __iconv(iconv_t, char **, size_t *, char **,
		size_t *, uint32_t, size_t *);

/*
 * GNU interfaces for iconv
 */
typedef struct {
	void	*spaceholder[64];
} iconv_allocation_t;

int	 iconv_open_into(const char *, const char *, iconv_allocation_t *);
void	 iconv_set_relocation_prefix(const char *, const char *);

/*
 * iconvctl() request macros
 */
#define	ICONV_TRIVIALP		0
#define	ICONV_GET_TRANSLITERATE	1
#define	ICONV_SET_TRANSLITERATE	2
#define	ICONV_GET_DISCARD_ILSEQ	3
#define	ICONV_SET_DISCARD_ILSEQ	4
#define	ICONV_SET_HOOKS		5
#define	ICONV_SET_FALLBACKS	6
#define	ICONV_GET_ILSEQ_INVALID	128
#define	ICONV_SET_ILSEQ_INVALID	129

void		iconvlist(int (*do_one) (unsigned int, const char * const *,
		    void *), void *);
const char	*iconv_canonicalize(const char *);
int		iconvctl(iconv_t, int, void *);

#ifdef __cplusplus
}
#endif

#endif /* _ICONV_H */
