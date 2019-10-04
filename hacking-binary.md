# Hacking the reMarkable xochitl binary

The main binary on the reMarkable is `xochitl` and is closed-source. It is a Qt
application using QML among other technologies. We are primarily interested in
how it accessed online services.

The update process does not appear to be governed by this binary, since there is
no sign of `get-updates` among its strings. For now, since updates are handled
without authentication etc, we are unlikely to encounter issues with the update
process itself; though ensuring that our changes persist across updates will be
interesting as we'll need to hook the update process later.

## Authentication

Currently we have no indication of the authentication URL location within the
file. The following strings containing `my.rem` are present in the file:

```
testing.my.remarkable.com
development-test.my.remarkable.com
qa.my.remarkable.com
```

Note, there is no `production.my.remarkable.com` nor a bare `my.remarkable.com`
which is the target used by other authentication clients with measured success.

Replacing the `r` of `remarkable` in those hostnames with `t`, `d`, and `q`
respectively did not cause any useful error logging. It is possible that the
error logging in question is not verbose enough to know it was a DNS lookup
failure which caused the SSL connection to fail.

There is no hint of `my.remarkable.com` anywhere on the filesystem other than
in `xochitl` so it's not likely to be being acquired as a hostname from elsewhere.

After further investigation we discovered that actually the login service is
another appspot service of `webapp-production-dot-remarkable-production.appspot.com`
and changing the obvious `webapp.` string to `logins.` reflects in the login flow.

The `-production-dot-remarkable-production.appspot.com` suffix appears to be
the one mentioned below, as a full string.

## Synchronisation

For now, our primary interest is in the synchronisation protocol and that means
both authentication (currently via my.remarkable.com) and service discovery and
beyond.

Hidden in the binary is a set of strings:

```
-production-dot-remarkable-production.appspot.com
-staging-dot-remarkable-development.appspot.com
-development-dot-remarkable-development.appspot.com
-qa-dot-remarkable-qa.appspot.com
service-manager-
-dot-remarkable-
.appspot.com
```

Interestingly the first four do not seem to be used in the initial negotiation
used in the production firmware. Instead the service manager hostname appears
to have been constructed from the `service-manager-` string, a string consisting
of `production` (likely from later in the file), the `-dot-remarkable-` string,
another copy of the `production` string, and then `.appspot.com`.

Indeed later in the file we see a set of strings:

```
hwdev
production
staging
development
qa
```

This means that, assuming we leave `service-manager-` and `production` alone,
we have `-dot-remarkable-` available as some kind of potential vhost spot,
and only `.appspot.com` to play with in terms of top level domain. It's possible
that if we can make `production` be a prefix then the `.` in the `.appspot.com`
string may not have to be the separator. In this instance, the `-dot-remarkable-`
would likely have to contain a real `.` in order to make it easier to host.

There is no evidence (yet) that the URLs returned from the discovery service
will in any way need to be the same. As such, it might be plausible to design
a vhosting service which uses either the authentication token or the hostname in
some form, to redirect to third party servers for actual storage.

For now, some top level domain names to consider are:

- `.remsync.net`
- `.r.flarn.net`
- `.remarkz.net`

The ideal, for remsync, would be if a 3 letter TLD were to have `remsync` free.

As such, for further discussions we will assume `.remsync.net` is available and
we'll use that in all our examples

## SSL certificates

The server responding to any request made by the reMarkable tablet, such as for
the `service-manager-production-dot-remarkable-production.remsync.net` endpoint,
will need an SSL certificate which the reMarkable will accept.

If at all possible, it will make sense to acquire a `*.remsync.net` or similar
certificate which can be deployed to any and all services under that domain.

TODO: We know the set of trusted root certificates, does LetsEncrypt fall into that?

## Domain names etc.

The string `-production-dot-remarkable-dot-production.appspot.com` and the
`service-manager-` `production` `-dot-remarkable-` `production` `.appspot.com`
strings are all nominally configurable by means of hex-editing the binary.

The trivial first pass is to replace `appspot.com` with `remsync.net` wherever
it shows up. This has the advantage that string lengths are preserved and
that only a single SSL cert for `*.remsync.net` is needed to cover all
eventualities.

Since we see that the login flow doesn't request something of the service-manager,
we know that it's unlikely to revert to `my.remarkable.com` in the near future.

# Configuration file

There's no obvious way to pick an entirely different configuration file name
as far as looking at the bytes in the binary goes. On the other hand some of
the field names are moderately easy to spot.

```
devicetoken -- Corresponds to the JWT for the device
usertoken   -- Corresponds to the session JWT (renewed every 24hrs or so)
wifinetworks -- The group name for wifi networks
```

We should probably rename those entries in order to reduce the chance of information
leakage should a tablet be updated without the user knowing / being able to prevent it.

Reversing the character order so `nekotecived` and `nekotresu` for the two tokens,
and the obvious but amusing `wifinotworks` for the group should be sufficient to
ensure that an update doesn't leak secrets.
