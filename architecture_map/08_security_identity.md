# 08 · Security & Identity — who are you, what can you do

> skin's lock. every layer's trust boundary.
> prev ← [07](07_frontend.md) · next → [09 full stack](09_full_stack.md)

## map

```
security & identity
├── AuthN (who are you?)
│   └── password | OTP / MFA | passkeys (WebAuthn/FIDO2) | magic link | SSO | biometrics | client cert (mTLS) | API key
├── AuthZ (what can you do?)
│   ├── models ── ACL | RBAC (roles) | ABAC (attributes) | ReBAC (relationships, Google Zanzibar) | PBAC (policies)
│   ├── token-level ── scopes | claims | roles in JWT
│   └── engines ── OPA / Rego | Cedar | OpenFGA | Casbin
├── remembering you (http is stateless)
│   ├── server session ── random session id in cookie, state on server (memory / redis)
│   └── token
│       ├── JWT ── self-contained, signed (header.payload.signature), verify w/o db
│       ├── opaque token ── random string, server looks it up / introspects
│       └── access token (short, mins) + refresh token (long, rotated)
├── where browser keeps it ── HttpOnly cookie (safest) | in-memory | localStorage (XSS can steal)
├── protocols (delegation & federation)
│   ├── OAuth 2.0 ── authorization: app gets access token to act for user, no password shared
│   │   └── grants ── auth code + PKCE (web/mobile) | client credentials (machine↔machine) | device code (tv) | refresh
│   │                 deprecated: implicit, password (ROPC)
│   ├── OIDC ── authentication on top of OAuth2: adds id_token (JWT about user) + /userinfo
│   ├── SAML 2.0 ── xml assertions, enterprise SSO (older)
│   ├── Kerberos / LDAP ── on-prem directory world (Active Directory)
│   └── IdPs ── Entra ID (Azure AD) | Okta | Auth0 | Keycloak | Cognito | Google | Ping
├── browser security
│   ├── same-origin policy (SOP) ── js can only read responses from same scheme+host+port
│   ├── CORS ── server header lists which OTHER origins may call it (relaxes SOP) · preflight = OPTIONS
│   ├── CSRF ── evil site makes your browser send request WITH your cookies → SameSite, CSRF token
│   ├── XSS ── attacker js runs inside your page → escape output, CSP, HttpOnly cookies
│   ├── clickjacking ── your site in invisible iframe → frame-ancestors / X-Frame-Options
│   └── cookie flags ── HttpOnly | Secure | SameSite (Strict / Lax / None) | Domain | Path | Max-Age
├── transport ── TLS / HTTPS | HSTS | mTLS (service↔service, zero trust) | cert rotation
├── secrets ── vault (HashiCorp, Azure Key Vault, AWS Secrets Manager) | KMS | never in git
├── data protection
│   ├── in transit (TLS) | at rest (disk/db encryption) | field-level / tokenization
│   ├── passwords ── hash + salt with bcrypt / scrypt / argon2 (never encrypt, never md5/sha1)
│   └── PII ── masking, minimization, retention · DPDP (India) | GDPR | RBI data localisation | PCI-DSS (cards)
└── appsec ── OWASP Top 10 | input validation | SQL injection | SSRF | least privilege | zero trust
```

## authN vs authZ

| | authN | authZ |
|---|---|---|
| question | who are you? | are you allowed? |
| fails with | 401 Unauthorized | 403 Forbidden |
| artifact | id_token, session | access token scopes, roles |
| protocol | OIDC, SAML | OAuth2 (scopes), RBAC/ABAC |

## session vs JWT (siblings)

| | server session | JWT |
|---|---|---|
| state lives | server (redis) | inside token |
| revoke | delete session, instant | hard — wait expiry / denylist |
| scale | shared session store needed | any instance verifies with key |
| size | tiny cookie | bigger (claims) |
| good for | classic web apps | APIs, microservices, mobile |

## OAuth2 vs OIDC vs SAML

| | OAuth 2.0 | OIDC | SAML |
|---|---|---|---|
| purpose | authorization (access) | authentication (login) | authentication (SSO) |
| token | access token (any format) | id_token (JWT) + access token | xml assertion |
| typical | "let app read my google drive" | "login with google" | corporate SSO into saas |
| era | 2012 | 2014 | 2005 |

## attacks (siblings)

| attack | one-liner | defense |
|---|---|---|
| XSS | your page runs attacker's js | escape, CSP, HttpOnly |
| CSRF | attacker triggers request with your cookies | SameSite, CSRF token |
| SQL injection | input becomes query | parameterized queries |
| SSRF | server tricked into calling internal urls | allowlist egress |
| clickjacking | invisible iframe over buttons | frame-ancestors |
| MITM | intercept traffic | TLS, HSTS, cert pinning |
| credential stuffing | leaked passwords tried elsewhere | MFA, rate limit |

## login with OIDC (auth code + PKCE) — one line each

1. app → redirect to IdP `/authorize` with code_challenge
2. user logs in at IdP
3. IdP → redirect back with `code`
4. app backend → IdP `/token` with code + code_verifier
5. gets id_token (who) + access_token (what) + refresh_token
6. call API with `Authorization: Bearer <access_token>`
