use zcash_address::ZcashAddress;

/// Validate the full encoding and checksum before classifying the address.
pub fn address_type(input: &str) -> Option<&'static str> {
    if input.trim() != input {
        return None;
    }
    let addr = ZcashAddress::try_from_encoded(input).ok()?;
    let encoded = addr.encode();
    if encoded.starts_with("zs1")
        || encoded.starts_with("ztestsapling1")
        || encoded.starts_with("zregtestsapling1")
    {
        Some("sapling")
    } else if encoded.starts_with("u1")
        || encoded.starts_with("utest1")
        || encoded.starts_with("uregtest1")
    {
        Some("unified")
    } else if addr.is_transparent_only()
        && (encoded.starts_with("t1")
            || encoded.starts_with("t3")
            || encoded.starts_with("tm")
            || encoded.starts_with("t2"))
    {
        Some("transparent")
    } else {
        None
    }
}

/// Amounts are handled as integer zatoshis, never floating point.
pub fn amount_zatoshis(s: &str) -> Result<u64, &'static str> {
    if s.starts_with('-') {
        return Err("ERR_AMOUNT_NON_POSITIVE");
    }
    let mut parts = s.split('.');
    let whole = parts.next().unwrap_or("");
    let frac = parts.next();
    if parts.next().is_some() || whole.is_empty() || !whole.bytes().all(|b| b.is_ascii_digit()) {
        return Err("ERR_AMOUNT_FORMAT");
    }
    let fractional = frac.unwrap_or("");
    if (frac.is_some() && fractional.is_empty()) || !fractional.bytes().all(|b| b.is_ascii_digit())
    {
        return Err("ERR_AMOUNT_FORMAT");
    }
    if fractional.len() > 8 {
        return Err("ERR_AMOUNT_PRECISION");
    }
    let w = whole.parse::<u64>().map_err(|_| "ERR_AMOUNT_RANGE")?;
    let f = if fractional.is_empty() {
        0
    } else {
        fractional.parse::<u64>().map_err(|_| "ERR_AMOUNT_FORMAT")?
            * 10u64.pow(8 - fractional.len() as u32)
    };
    let amount = w
        .checked_mul(100_000_000)
        .and_then(|x| x.checked_add(f))
        .ok_or("ERR_AMOUNT_RANGE")?;
    if amount == 0 {
        return Err("ERR_AMOUNT_NON_POSITIVE");
    }
    if amount > 21_000_000 * 100_000_000 {
        return Err("ERR_AMOUNT_RANGE");
    }
    Ok(amount)
}

pub fn payment_request(
    address: &str,
    amount: &str,
    shielded: bool,
) -> Result<String, &'static str> {
    let kind = address_type(address).ok_or("ERR_INVALID_ADDRESS")?;
    if shielded && kind == "transparent" {
        return Err("ERR_SHIELDED_REQUIRED");
    }
    let zats = amount_zatoshis(amount)?;
    let mut value = format!("{}.{:08}", zats / 100_000_000, zats % 100_000_000);
    while value.ends_with('0') {
        value.pop();
    }
    if value.ends_with('.') {
        value.pop();
    }
    let canonical = ZcashAddress::try_from_encoded(address)
        .map_err(|_| "ERR_INVALID_ADDRESS")?
        .encode();
    Ok(format!("zcash:{canonical}?amount={value}"))
}

#[cfg(test)]
mod tests {
    use super::*;
    const T: &str = "t1Hsc1LR8yKnbbe3twRp88p6vFfC5t7DLbs";
    const S: &str =
        "zs1z7rejlpsa98s2rrrfkwmaxu53e4ue0ulcrw0h4x5g8jl04tak0d3mm47vdtahatqrlkngh9slya";
    #[test]
    fn transparent_and_sapling() {
        assert_eq!(address_type(T), Some("transparent"));
        assert_eq!(address_type(S), Some("sapling"));
    }
    #[test]
    fn malformed_and_empty() {
        for s in [
            "",
            "garbage",
            "u1fake",
            "zs1fake",
            " t1Hsc1LR8yKnbbe3twRp88p6vFfC5t7DLbs",
        ] {
            assert_eq!(address_type(s), None);
        }
    }
    #[test]
    fn bad_checksum() {
        assert_eq!(address_type("t1Hsc1LR8yKnbbe3twRp88p6vFfC5t7DLbt"), None);
    }
    #[test]
    fn precision_and_integer_arithmetic() {
        assert_eq!(amount_zatoshis("0.00000001"), Ok(1));
        assert_eq!(amount_zatoshis("1.23000000"), Ok(123000000));
        assert_eq!(amount_zatoshis("21000000"), Ok(2100000000000000));
    }
    #[test]
    fn non_positive() {
        for s in ["0", "0.00000000", "-1"] {
            assert_eq!(amount_zatoshis(s), Err("ERR_AMOUNT_NON_POSITIVE"));
        }
    }
    #[test]
    fn invalid_formats() {
        for s in ["NaN", "inf", "1e2", "", ".1", "1.", "1.2.3", "+1", " 1"] {
            assert_eq!(amount_zatoshis(s), Err("ERR_AMOUNT_FORMAT"));
        }
    }
    #[test]
    fn precision_and_range_errors() {
        assert_eq!(amount_zatoshis("0.000000001"), Err("ERR_AMOUNT_PRECISION"));
        for s in ["21000000.00000001", "999999999999999999999999999"] {
            assert_eq!(amount_zatoshis(s), Err("ERR_AMOUNT_RANGE"));
        }
    }
    #[test]
    fn payment_errors() {
        assert_eq!(
            payment_request("bad", "1", false),
            Err("ERR_INVALID_ADDRESS")
        );
        assert_eq!(payment_request(T, "1", true), Err("ERR_SHIELDED_REQUIRED"));
        assert_eq!(payment_request(T, "NaN", false), Err("ERR_AMOUNT_FORMAT"));
    }
    #[test]
    fn uri_happy_path() {
        assert_eq!(
            payment_request(S, "1.23000000", true),
            Ok(format!("zcash:{S}?amount=1.23"))
        );
        assert_eq!(
            payment_request(T, "1", false),
            Ok(format!("zcash:{T}?amount=1"))
        );
    }
}
