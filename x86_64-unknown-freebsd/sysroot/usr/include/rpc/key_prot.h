/*
 * Please do not edit this file.
 * It was generated using rpcgen.
 */

#ifndef _KEY_PROT_H_RPCGEN
#define	_KEY_PROT_H_RPCGEN

#include <rpc/rpc.h>

#ifdef __cplusplus
extern "C" {
#endif

/*-
 * Copyright (c) 2010, Oracle America, Inc.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions are
 * met:
 *
 *     * Redistributions of source code must retain the above copyright
 *       notice, this list of conditions and the following disclaimer.
 *     * Redistributions in binary form must reproduce the above
 *       copyright notice, this list of conditions and the following
 *       disclaimer in the documentation and/or other materials
 *       provided with the distribution.
 *     * Neither the name of the "Oracle America, Inc." nor the names of its
 *       contributors may be used to endorse or promote products derived
 *       from this software without specific prior written permission.
 *
 *   THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
 *   "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
 *   LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS
 *   FOR A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE
 *   COPYRIGHT HOLDER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT,
 *   INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
 *   DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE
 *   GOODS OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS
 *   INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY,
 *   WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING
 *   NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 *   OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */
/* From: #pragma ident	"@(#)key_prot.x	1.7	94/04/29 SMI" */
/* Copyright (c)  1990, 1991 Sun Microsystems, Inc. */
#include <sys/cdefs.h>
__FBSDID("$FreeBSD: releng/12.3/include/rpcsvc/key_prot.x 259117 2013-12-09 04:26:50Z hrs $");

/* 
 * Compiled from key_prot.x using rpcgen.
 * DO NOT EDIT THIS FILE!
 * This is NOT source code!
 */
#define	PROOT 3
#define	HEXMODULUS "d4a0ba0250b6fd2ec626e7efd637df76c716e22d0944b88b"
#define	HEXKEYBYTES 48
#define	KEYSIZE 192
#define	KEYBYTES 24
#define	KEYCHECKSUMSIZE 16

enum keystatus {
	KEY_SUCCESS = 0,
	KEY_NOSECRET = 1,
	KEY_UNKNOWN = 2,
	KEY_SYSTEMERR = 3
};
typedef enum keystatus keystatus;

typedef char keybuf[HEXKEYBYTES];

typedef char *netnamestr;

struct cryptkeyarg {
	netnamestr remotename;
	des_block deskey;
};
typedef struct cryptkeyarg cryptkeyarg;

struct cryptkeyarg2 {
	netnamestr remotename;
	netobj remotekey;
	des_block deskey;
};
typedef struct cryptkeyarg2 cryptkeyarg2;

struct cryptkeyres {
	keystatus status;
	union {
		des_block deskey;
	} cryptkeyres_u;
};
typedef struct cryptkeyres cryptkeyres;
#define	MAXGIDS 16

struct unixcred {
	u_int uid;
	u_int gid;
	struct {
		u_int gids_len;
		u_int *gids_val;
	} gids;
};
typedef struct unixcred unixcred;

struct getcredres {
	keystatus status;
	union {
		unixcred cred;
	} getcredres_u;
};
typedef struct getcredres getcredres;

struct key_netstarg {
	keybuf st_priv_key;
	keybuf st_pub_key;
	netnamestr st_netname;
};
typedef struct key_netstarg key_netstarg;

struct key_netstres {
	keystatus status;
	union {
		key_netstarg knet;
	} key_netstres_u;
};
typedef struct key_netstres key_netstres;

#ifndef opaque
#define opaque char
#endif


#define	KEY_PROG ((unsigned long)(100029))
#define	KEY_VERS ((unsigned long)(1))

extern  void key_prog_1(struct svc_req *rqstp, SVCXPRT *transp);
#define	KEY_SET ((unsigned long)(1))
extern  keystatus * key_set_1(char *, CLIENT *);
extern  keystatus * key_set_1_svc(char *, struct svc_req *);
#define	KEY_ENCRYPT ((unsigned long)(2))
extern  cryptkeyres * key_encrypt_1(cryptkeyarg *, CLIENT *);
extern  cryptkeyres * key_encrypt_1_svc(cryptkeyarg *, struct svc_req *);
#define	KEY_DECRYPT ((unsigned long)(3))
extern  cryptkeyres * key_decrypt_1(cryptkeyarg *, CLIENT *);
extern  cryptkeyres * key_decrypt_1_svc(cryptkeyarg *, struct svc_req *);
#define	KEY_GEN ((unsigned long)(4))
extern  des_block * key_gen_1(void *, CLIENT *);
extern  des_block * key_gen_1_svc(void *, struct svc_req *);
#define	KEY_GETCRED ((unsigned long)(5))
extern  getcredres * key_getcred_1(netnamestr *, CLIENT *);
extern  getcredres * key_getcred_1_svc(netnamestr *, struct svc_req *);
extern int key_prog_1_freeresult(SVCXPRT *, xdrproc_t, caddr_t);
#define	KEY_VERS2 ((unsigned long)(2))

extern  void key_prog_2(struct svc_req *rqstp, SVCXPRT *transp);
extern  keystatus * key_set_2(char *, CLIENT *);
extern  keystatus * key_set_2_svc(char *, struct svc_req *);
extern  cryptkeyres * key_encrypt_2(cryptkeyarg *, CLIENT *);
extern  cryptkeyres * key_encrypt_2_svc(cryptkeyarg *, struct svc_req *);
extern  cryptkeyres * key_decrypt_2(cryptkeyarg *, CLIENT *);
extern  cryptkeyres * key_decrypt_2_svc(cryptkeyarg *, struct svc_req *);
extern  des_block * key_gen_2(void *, CLIENT *);
extern  des_block * key_gen_2_svc(void *, struct svc_req *);
extern  getcredres * key_getcred_2(netnamestr *, CLIENT *);
extern  getcredres * key_getcred_2_svc(netnamestr *, struct svc_req *);
#define	KEY_ENCRYPT_PK ((unsigned long)(6))
extern  cryptkeyres * key_encrypt_pk_2(cryptkeyarg2 *, CLIENT *);
extern  cryptkeyres * key_encrypt_pk_2_svc(cryptkeyarg2 *, struct svc_req *);
#define	KEY_DECRYPT_PK ((unsigned long)(7))
extern  cryptkeyres * key_decrypt_pk_2(cryptkeyarg2 *, CLIENT *);
extern  cryptkeyres * key_decrypt_pk_2_svc(cryptkeyarg2 *, struct svc_req *);
#define	KEY_NET_PUT ((unsigned long)(8))
extern  keystatus * key_net_put_2(key_netstarg *, CLIENT *);
extern  keystatus * key_net_put_2_svc(key_netstarg *, struct svc_req *);
#define	KEY_NET_GET ((unsigned long)(9))
extern  key_netstres * key_net_get_2(void *, CLIENT *);
extern  key_netstres * key_net_get_2_svc(void *, struct svc_req *);
#define	KEY_GET_CONV ((unsigned long)(10))
extern  cryptkeyres * key_get_conv_2(char *, CLIENT *);
extern  cryptkeyres * key_get_conv_2_svc(char *, struct svc_req *);
extern int key_prog_2_freeresult(SVCXPRT *, xdrproc_t, caddr_t);

/* the xdr functions */
extern  bool_t xdr_keystatus(XDR *, keystatus*);
extern  bool_t xdr_keybuf(XDR *, keybuf);
extern  bool_t xdr_netnamestr(XDR *, netnamestr*);
extern  bool_t xdr_cryptkeyarg(XDR *, cryptkeyarg*);
extern  bool_t xdr_cryptkeyarg2(XDR *, cryptkeyarg2*);
extern  bool_t xdr_cryptkeyres(XDR *, cryptkeyres*);
extern  bool_t xdr_unixcred(XDR *, unixcred*);
extern  bool_t xdr_getcredres(XDR *, getcredres*);
extern  bool_t xdr_key_netstarg(XDR *, key_netstarg*);
extern  bool_t xdr_key_netstres(XDR *, key_netstres*);

#ifdef __cplusplus
}
#endif

#endif /* !_KEY_PROT_H_RPCGEN */
