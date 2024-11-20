/*
 * This file and its contents are supplied under the terms of the
 * Common Development and Distribution License ("CDDL"), version 1.0.
 * You may only use this file in accordance with the terms of version
 * 1.0 of the CDDL.
 *
 * A full copy of the text of the CDDL should have accompanied this
 * source.  A copy of the CDDL is also available via the Internet at
 * http://www.illumos.org/license/CDDL.
 */

/*
 * Copyright 2022 DilOS
 */

#ifndef _SYS_KMEM_CACHE_H
#define	_SYS_KMEM_CACHE_H

#define	spl_kmem_cache_inuse(CACHE) \
	kmem_cache_stat((CACHE), "buf_inuse")
#define	spl_kmem_cache_entry_size(CACHE) \
	kmem_cache_stat((CACHE), "buf_size")

#endif	/* _SYS_KMEM_CACHE_H */
