# Upgrade Notes

## generic-array 1.x Migration

- Current stack (`digest 0.10`, `sha2 0.10`, `sha1 0.10`, `hmac 0.12`, `hkdf 0.12`, `rsa 0.9`, etc.) hard-depends on `generic-array 0.14.x` APIs. No published versions in this stack use `generic-array 1.x` yet.
- A move to 1.x requires new major releases of the RustCrypto crates (e.g., `digest 0.11`, `sha2 0.11`, `hmac 0.13`, `hkdf 0.13`, `rsa 0.10+`) that adopt `generic-array 1.x`. Until those exist, forcing 1.x breaks transitive deps.
- Alternative hashing stacks (e.g., `ring`, `openssl`/`aws-lc-rs`, `blake3`) are not drop-in replacements for SHA1/SHA256/HMAC/MD5 paths and would require feature-gated rewrites of auth flows.
- Work once 1.x-compatible releases exist:
  - Bump `digest`/`sha*`/`hmac`/`hkdf`/`rsa` to the 1.x-compatible releases.
  - Update code where old `GenericArray` APIs were used (construction/output handling).
  - Validate MySQL auth (SHA-1/SHA-256 scramble), Postgres SASL (HMAC/SHA-256), HKDF/RSA call sites.

## Postgres ipnetwork Array Support

- The `ipnetwork` feature already provides Type/Encode/Decode for single `IpNetwork`. To enable `Option<Vec<IpNetwork>>`, add `impl PgHasArrayType for ipnetwork::IpNetwork` under `cfg(feature = "ipnetwork")`.
- Point `array_type_info()` to the array OID consistent with the single-value mapping (typically `inet[]` = OID 1041; `cidr[]` = OID 651 if used).
- Keep the feature flag name `ipnetwork` stable and transitively enabled.
- Touch points: `sqlx-postgres/src/types/ipnetwork.rs` (or module where Type is defined) and `sqlx-postgres/src/types/array.rs` for the `PgHasArrayType` impl; ensure exported under the feature.
