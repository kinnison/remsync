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

TODO: DNS hijack to find out what's going on with this.

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
