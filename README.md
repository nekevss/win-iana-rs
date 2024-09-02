Mostly a proof of concept using CLDR's windowZones.json file to get an IANA time zone ID on windows.

There's still a few things to figure out and a lot to be optimized.

## TODOs

- `windowsZones.json` uses a territory code to [differentiate zones](https://github.com/nekevss/win-iana-rs/blob/main/cldr-data/cldr-core/supplemental/windowsZones.json#L668) with "001" as the default.
    - More research is needed. Codes appear to be ISO3166, but that needs to be confirmed that the codes are ISO 3166
    - Need to test the proper way to fetch codes from Windows. `uregion_*` methods in Win32::Globalization are the current [prime suspects](https://docs.rs/windows-sys/0.59.0/windows_sys/Win32/Globalization/fn.uregion_getRegionCode.html)
- Massively improve deserialization and general representation of deserialized `windowsZones`.
    - As this is a proof of concept, it was mostly just get it working, but this would need to be MASSIVELY improved upon if it were to be ever released in production. 
- Upstream `tinystr` update or use a different struct to represent Windows strings.

## Note on breaking CI

Also worth noting, this depends on a local update to `tinystr`, so the CI is always going to fail unless that is updated (oops).

```rust
    pub const fn try_from_raw_u16(raw: [u16; N]) -> Result<Self, ParseError> {
        Self::try_from_utf16_inner(&raw, 0, raw.len(), true)
    }
```
