Mostly a proof of concept using CLDR's windowZones.json file to get an IANA time zone ID on windows.

There's still a few things to figure out and a lot to be optimized.

Also worth noting, this depends on a local update to `tinystr`, so the CI is always going to fail unless that is updated (oops).

```rust
    pub const fn try_from_raw_u16(raw: [u16; N]) -> Result<Self, ParseError> {
        Self::try_from_utf16_inner(&raw, 0, raw.len(), true)
    }
```
