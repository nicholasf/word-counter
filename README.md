# Introduction

## Install

Install the go migration binary.

## Postgres.

For local installation you will probably need to tweak a couple of things:
*
1. Locate your pg_hba.conf file. On Fedora mine is `/var/lib/pgsql/data/pg_hba.conf`.
2. Configure *local* connections to be trusted from "peer"

```
# "local" is for Unix domain socket connections only
local   all             all                                     trust
```

3. Configure IPv4 connections on the host to use "md5" not ident or peer.

```
# IPv4 local connections:
host    all             all             127.0.0.1/32            md5

```