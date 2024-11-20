/*
 * Please do not edit this file.
 * It was generated using rpcgen.
 */

#ifndef _PMAP_PROT_H_RPCGEN
#define	_PMAP_PROT_H_RPCGEN

#include <rpc/rpc.h>
#ifndef _KERNEL
#include <synch.h>
#include <thread.h>
#endif /* !_KERNEL */
/*
 * Copyright (c) 1984,1989 by Sun Microsystems, Inc.
 */
/* from pmap_prot.x */

#pragma ident	"%Z%%M%	%I%	%E% SMI"

#ifndef _KERNEL

/*
 * Protocol for the local binder service, or pmap.
 *
 * Copyright (C) 1984, Sun Microsystems, Inc.
 *
 * The following procedures are supported by the protocol:
 *
 * PMAPPROC_NULL() returns ()
 * 	takes nothing, returns nothing
 *
 * PMAPPROC_SET(struct pmap) returns (bool_t)
 * 	TRUE is success, FALSE is failure.  Registers the tuple
 *	[prog, vers, prot, port].
 *
 * PMAPPROC_UNSET(struct pmap) returns (bool_t)
 *	TRUE is success, FALSE is failure.  Un-registers pair
 *	[prog, vers].  prot and port are ignored.
 *
 * PMAPPROC_GETPORT(struct pmap) returns (rpcport_t).
 *	0 is failure.  Otherwise returns the port number where the pair
 *	[prog, vers] is registered.  It may lie!
 *
 * PMAPPROC_DUMP() RETURNS (struct pmaplist_ptr)
 *
 * PMAPPROC_CALLIT(unsigned, unsigned, unsigned, string<>)
 * 	RETURNS (port, string<>);
 * usage: encapsulatedresults = PMAPPROC_CALLIT(prog, vers, proc,
 *						encapsulatedargs);
 * 	Calls the procedure on the local machine.  If it is not registered,
 *	this procedure is quite; ie it does not return error information!!!
 *	This procedure only is supported on rpc/udp and calls via
 *	rpc/udp.  This routine only passes null authentication parameters.
 *	This file has no interface to xdr routines for PMAPPROC_CALLIT.
 *
 * The service supports remote procedure calls on udp/ip or tcp/ip socket 111.
 */

#define	PMAPPORT 111


/*
 * A mapping of (program, version, protocol) to port number
 */

struct pmap {
	rpcprog_t pm_prog;
	rpcvers_t pm_vers;
	rpcprot_t pm_prot;
	rpcport_t pm_port;
};
typedef struct pmap pmap;

typedef pmap PMAP;


/*
 * Supported values for the "prot" field
 */

#define	PMAP_IPPROTO_TCP 6
#define	PMAP_IPPROTO_UDP 17


/*
 * A list of mappings
 *
 * Below are two definitions for the pmaplist structure.  This is done because
 * xdr_pmaplist() is specified to take a struct pmaplist **, rather than a
 * struct pmaplist * that rpcgen would produce.  One version of the pmaplist
 * structure (actually called pm__list) is used with rpcgen, and the other is
 * defined only in the header file for compatibility with the specified
 * interface.
 */

struct pm__list {
	pmap pml_map;
	struct pm__list *pml_next;
};
typedef struct pm__list pm__list;

typedef pm__list *pmaplist_ptr;

struct pmaplist {
	PMAP pml_map;
	struct pmaplist *pml_next;
};

typedef struct pmaplist pmaplist;
typedef struct pmaplist PMAPLIST;

#ifdef __cplusplus
extern "C" {
#endif
#ifdef __STDC__
extern  bool_t xdr_pmaplist(XDR *, pmaplist**);
#else /* K&R C */
bool_t xdr_pmaplist();
#endif
#ifdef	__cplusplus
}
#endif


/*
 * Arguments to callit
 */

struct rmtcallargs {
	rpcprog_t prog;
	rpcvers_t vers;
	rpcproc_t proc;
	struct {
		u_int args_len;
		char *args_val;
	} args;
};
typedef struct rmtcallargs rmtcallargs;

/*
 * Client-side only representation of rmtcallargs structure.
 *
 * The routine that XDRs the rmtcallargs structure must deal with the
 * opaque arguments in the "args" structure.  xdr_rmtcall_args() needs to be
 * passed the XDR routine that knows the args' structure.  This routine
 * doesn't need to go over-the-wire (and it wouldn't make sense anyway) since
 * the application being called knows the args structure already.  So we use a
 * different "XDR" structure on the client side, p_rmtcallargs, which includes
 * the args' XDR routine.
 */
struct p_rmtcallargs {
	rpcprog_t prog;
	rpcvers_t vers;
	rpcproc_t proc;
	struct {
		u_int args_len;
		char *args_val;
	} args;
	xdrproc_t	xdr_args;	/* encodes args */
};



/*
 * Results of callit
 */

struct rmtcallres {
	rpcport_t port;
	struct {
		u_int res_len;
		char *res_val;
	} res;
};
typedef struct rmtcallres rmtcallres;

/*
 * Client-side only representation of rmtcallres structure.
 */
struct p_rmtcallres {
	rpcport_t port;
	struct {
		u_int res_len;
		char *res_val;
	} res;
	xdrproc_t	xdr_res;	/* decodes res */
};


#define	PMAPVERS_PROTO		((rpcvers_t)2)
#define	PMAPVERS_ORIG		((rpcvers_t)1)

#else		/* ndef _KERNEL */

#include <rpc/pmap_rmt.h>

#ifdef __cplusplus
extern "C" {
#endif

#define	PMAPPORT 111

struct pmap {
	rpcprog_t pm_prog;
	rpcvers_t pm_vers;
	rpcprot_t pm_prot;
	rpcport_t pm_port;
};
typedef struct pmap PMAP;
#ifdef __STDC__
extern bool_t xdr_pmap (XDR *, struct pmap *);
#else
extern bool_t xdr_pmap ();
#endif

struct pmaplist {
	struct pmap pml_map;
	struct pmaplist *pml_next;
};
typedef struct pmaplist PMAPLIST;
typedef struct pmaplist *pmaplist_ptr;


#ifdef __cplusplus
}
#endif

#endif		/* ndef _KERNEL */

#define	PMAPPROG	100000
#define	PMAPVERS	2
#define	PMAPPROC_NULL	0
extern  enum clnt_stat pmapproc_null_2();
extern  bool_t pmapproc_null_2_svc();
#define	PMAPPROC_SET	1
extern  enum clnt_stat pmapproc_set_2();
extern  bool_t pmapproc_set_2_svc();
#define	PMAPPROC_UNSET	2
extern  enum clnt_stat pmapproc_unset_2();
extern  bool_t pmapproc_unset_2_svc();
#define	PMAPPROC_GETPORT	3
extern  enum clnt_stat pmapproc_getport_2();
extern  bool_t pmapproc_getport_2_svc();
#define	PMAPPROC_DUMP	4
extern  enum clnt_stat pmapproc_dump_2();
extern  bool_t pmapproc_dump_2_svc();
#define	PMAPPROC_CALLIT	5
extern  enum clnt_stat pmapproc_callit_2();
extern  bool_t pmapproc_callit_2_svc();
extern int pmapprog_2_freeresult();

/* the xdr functions */
extern bool_t xdr_pmap();
extern bool_t xdr_pm__list();
extern bool_t xdr_pmaplist_ptr();
extern bool_t xdr_rmtcallargs();
extern bool_t xdr_rmtcallres();

#endif /* !_PMAP_PROT_H_RPCGEN */
