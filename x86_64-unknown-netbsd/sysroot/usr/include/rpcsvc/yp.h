/*
 * Please do not edit this file.
 * It was generated using rpcgen.
 */

#ifndef _YP_H_RPCGEN
#define _YP_H_RPCGEN

#define RPCGEN_VERSION	199506

#include <rpc/rpc.h>

#define YPMAXRECORD 1024
#define YPMAXDOMAIN 64
#define YPMAXMAP 64
#define YPMAXPEER 64

enum ypstat {
	YP_TRUE = 1,
	YP_NOMORE = 2,
	YP_FALSE = 0,
	YP_NOMAP = -1,
	YP_NODOM = -2,
	YP_NOKEY = -3,
	YP_BADOP = -4,
	YP_BADDB = -5,
	YP_YPERR = -6,
	YP_BADARGS = -7,
	YP_VERS = -8
};
typedef enum ypstat ypstat;

enum ypxfrstat {
	YPXFR_SUCC = 1,
	YPXFR_AGE = 2,
	YPXFR_NOMAP = -1,
	YPXFR_NODOM = -2,
	YPXFR_RSRC = -3,
	YPXFR_RPC = -4,
	YPXFR_MADDR = -5,
	YPXFR_YPERR = -6,
	YPXFR_BADARGS = -7,
	YPXFR_DBM = -8,
	YPXFR_FILE = -9,
	YPXFR_SKEW = -10,
	YPXFR_CLEAR = -11,
	YPXFR_FORCE = -12,
	YPXFR_XFRERR = -13,
	YPXFR_REFUSED = -14
};
typedef enum ypxfrstat ypxfrstat;

typedef char *domainname;

typedef char *mapname;

typedef char *peername;

typedef struct {
	unsigned int keydat_len;
	char *keydat_val;
} keydat;

typedef struct {
	unsigned int valdat_len;
	char *valdat_val;
} valdat;

struct ypmap_parms {
	domainname domain;
	mapname map;
	u_int ordernum;
	peername peer;
};
typedef struct ypmap_parms ypmap_parms;

struct ypreq_key {
	domainname domain;
	mapname map;
	keydat key;
};
typedef struct ypreq_key ypreq_key;

struct ypreq_nokey {
	domainname domain;
	mapname map;
};
typedef struct ypreq_nokey ypreq_nokey;

struct ypreq_xfr {
	ypmap_parms map_parms;
	u_int transid;
	u_int prog;
	u_int port;
};
typedef struct ypreq_xfr ypreq_xfr;

struct ypresp_val {
	ypstat stat;
	valdat val;
};
typedef struct ypresp_val ypresp_val;

struct ypresp_key_val {
	ypstat stat;
	keydat key;
	valdat val;
};
typedef struct ypresp_key_val ypresp_key_val;

struct ypresp_master {
	ypstat stat;
	peername peer;
};
typedef struct ypresp_master ypresp_master;

struct ypresp_order {
	ypstat stat;
	u_int ordernum;
};
typedef struct ypresp_order ypresp_order;

struct ypresp_all {
	bool_t more;
	union {
		ypresp_key_val val;
	} ypresp_all_u;
};
typedef struct ypresp_all ypresp_all;

struct ypresp_xfr {
	u_int transid;
	ypxfrstat xfrstat;
};
typedef struct ypresp_xfr ypresp_xfr;

struct ypmaplist {
	mapname map;
	struct ypmaplist *next;
};
typedef struct ypmaplist ypmaplist;

struct ypresp_maplist {
	ypstat stat;
	ypmaplist *maps;
};
typedef struct ypresp_maplist ypresp_maplist;

enum yppush_status {
	YPPUSH_SUCC = 1,
	YPPUSH_AGE = 2,
	YPPUSH_NOMAP = -1,
	YPPUSH_NODOM = -2,
	YPPUSH_RSRC = -3,
	YPPUSH_RPC = -4,
	YPPUSH_MADDR = -5,
	YPPUSH_YPERR = -6,
	YPPUSH_BADARGS = -7,
	YPPUSH_DBM = -8,
	YPPUSH_FILE = -9,
	YPPUSH_SKEW = -10,
	YPPUSH_CLEAR = -11,
	YPPUSH_FORCE = -12,
	YPPUSH_XFRERR = -13,
	YPPUSH_REFUSED = -14
};
typedef enum yppush_status yppush_status;

struct yppushresp_xfr {
	u_int transid;
	yppush_status status;
};
typedef struct yppushresp_xfr yppushresp_xfr;

enum ypbind_resptype {
	YPBIND_SUCC_VAL = 1,
	YPBIND_FAIL_VAL = 2
};
typedef enum ypbind_resptype ypbind_resptype;

struct ypbind_binding {
	char ypbind_binding_addr[4];
	char ypbind_binding_port[2];
};
typedef struct ypbind_binding ypbind_binding;

struct ypbind_resp {
	ypbind_resptype ypbind_status;
	union {
		u_int ypbind_error;
		ypbind_binding ypbind_bindinfo;
	} ypbind_resp_u;
};
typedef struct ypbind_resp ypbind_resp;
#define YPBIND_ERR_ERR 1
#define YPBIND_ERR_NOSERV 2
#define YPBIND_ERR_RESC 3

struct ypbind_setdom {
	domainname ypsetdom_domain;
	ypbind_binding ypsetdom_binding;
	u_int ypsetdom_vers;
};
typedef struct ypbind_setdom ypbind_setdom;

__BEGIN_DECLS
bool_t xdr_ypstat(XDR *, ypstat *);
bool_t xdr_ypxfrstat(XDR *, ypxfrstat *);
bool_t xdr_domainname(XDR *, domainname *);
bool_t xdr_mapname(XDR *, mapname *);
bool_t xdr_peername(XDR *, peername *);
bool_t xdr_keydat(XDR *, keydat *);
bool_t xdr_valdat(XDR *, valdat *);
bool_t xdr_ypmap_parms(XDR *, ypmap_parms *);
bool_t xdr_ypreq_key(XDR *, ypreq_key *);
bool_t xdr_ypreq_nokey(XDR *, ypreq_nokey *);
bool_t xdr_ypreq_xfr(XDR *, ypreq_xfr *);
bool_t xdr_ypresp_val(XDR *, ypresp_val *);
bool_t xdr_ypresp_key_val(XDR *, ypresp_key_val *);
bool_t xdr_ypresp_master(XDR *, ypresp_master *);
bool_t xdr_ypresp_order(XDR *, ypresp_order *);
bool_t xdr_ypresp_all(XDR *, ypresp_all *);
bool_t xdr_ypresp_xfr(XDR *, ypresp_xfr *);
bool_t xdr_ypmaplist(XDR *, ypmaplist *);
bool_t xdr_ypresp_maplist(XDR *, ypresp_maplist *);
bool_t xdr_yppush_status(XDR *, yppush_status *);
bool_t xdr_yppushresp_xfr(XDR *, yppushresp_xfr *);
bool_t xdr_ypbind_resptype(XDR *, ypbind_resptype *);
bool_t xdr_ypbind_binding(XDR *, ypbind_binding *);
bool_t xdr_ypbind_resp(XDR *, ypbind_resp *);
bool_t xdr_ypbind_setdom(XDR *, ypbind_setdom *);
__END_DECLS

#define YPPROG 100004
#define YPVERS 2
#define YPPROC_NULL 0
#define YPPROC_DOMAIN 1
#define YPPROC_DOMAIN_NONACK 2
#define YPPROC_MATCH 3
#define YPPROC_FIRST 4
#define YPPROC_NEXT 5
#define YPPROC_XFR 6
#define YPPROC_CLEAR 7
#define YPPROC_ALL 8
#define YPPROC_MASTER 9
#define YPPROC_ORDER 10
#define YPPROC_MAPLIST 11

__BEGIN_DECLS
void *ypproc_null_2(void *, CLIENT *);
void *ypproc_null_2_svc(void *, struct svc_req *);
bool_t *ypproc_domain_2(domainname *, CLIENT *);
bool_t *ypproc_domain_2_svc(domainname *, struct svc_req *);
bool_t *ypproc_domain_nonack_2(domainname *, CLIENT *);
bool_t *ypproc_domain_nonack_2_svc(domainname *, struct svc_req *);
ypresp_val *ypproc_match_2(ypreq_key *, CLIENT *);
ypresp_val *ypproc_match_2_svc(ypreq_key *, struct svc_req *);
ypresp_key_val *ypproc_first_2(ypreq_key *, CLIENT *);
ypresp_key_val *ypproc_first_2_svc(ypreq_key *, struct svc_req *);
ypresp_key_val *ypproc_next_2(ypreq_key *, CLIENT *);
ypresp_key_val *ypproc_next_2_svc(ypreq_key *, struct svc_req *);
ypresp_xfr *ypproc_xfr_2(ypreq_xfr *, CLIENT *);
ypresp_xfr *ypproc_xfr_2_svc(ypreq_xfr *, struct svc_req *);
void *ypproc_clear_2(void *, CLIENT *);
void *ypproc_clear_2_svc(void *, struct svc_req *);
ypresp_all *ypproc_all_2(ypreq_nokey *, CLIENT *);
ypresp_all *ypproc_all_2_svc(ypreq_nokey *, struct svc_req *);
ypresp_master *ypproc_master_2(ypreq_nokey *, CLIENT *);
ypresp_master *ypproc_master_2_svc(ypreq_nokey *, struct svc_req *);
ypresp_order *ypproc_order_2(ypreq_nokey *, CLIENT *);
ypresp_order *ypproc_order_2_svc(ypreq_nokey *, struct svc_req *);
ypresp_maplist *ypproc_maplist_2(domainname *, CLIENT *);
ypresp_maplist *ypproc_maplist_2_svc(domainname *, struct svc_req *);
__END_DECLS

#define YPPUSH_XFRRESPPROG 0x40000000
#define YPPUSH_XFRRESPVERS 1
#define YPPUSHPROC_NULL 0
#define YPPUSHPROC_XFRRESP 1

__BEGIN_DECLS
void *yppushproc_null_1(void *, CLIENT *);
void *yppushproc_null_1_svc(void *, struct svc_req *);
yppushresp_xfr *yppushproc_xfrresp_1(void *, CLIENT *);
yppushresp_xfr *yppushproc_xfrresp_1_svc(void *, struct svc_req *);
__END_DECLS

#define YPBINDPROG 100007
#define YPBINDVERS 2
#define YPBINDPROC_NULL 0
#define YPBINDPROC_DOMAIN 1
#define YPBINDPROC_SETDOM 2

__BEGIN_DECLS
void *ypbindproc_null_2(void *, CLIENT *);
void *ypbindproc_null_2_svc(void *, struct svc_req *);
ypbind_resp *ypbindproc_domain_2(domainname *, CLIENT *);
ypbind_resp *ypbindproc_domain_2_svc(domainname *, struct svc_req *);
void *ypbindproc_setdom_2(ypbind_setdom *, CLIENT *);
void *ypbindproc_setdom_2_svc(ypbind_setdom *, struct svc_req *);
__END_DECLS

#endif /* !_YP_H_RPCGEN */