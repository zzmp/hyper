//! Utility functions for Header implementations.

use std::str;
use std::fmt;
use time;

/// Reads a single raw string when parsing a header
pub fn from_one_raw_str<T: str::FromStr>(raw: &[Vec<u8>]) -> Option<T> {
    if raw.len() != 1 {
        return None;
    }
    // we JUST checked that raw.len() == 1, so raw[0] WILL exist.
    match str::from_utf8(&raw[0][..]) {
        Ok(s) => str::FromStr::from_str(s).ok(),
        Err(_) => None
    }
}

/// Reads a comma-delimited raw header into a Vec.
#[inline]
pub fn from_comma_delimited<T: str::FromStr>(raw: &[Vec<u8>]) -> Option<Vec<T>> {
    if raw.len() != 1 {
        return None;
    }
    // we JUST checked that raw.len() == 1, so raw[0] WILL exist.
    from_one_comma_delimited(&raw[0][..])
}

/// Reads a comma-delimited raw string into a Vec.
pub fn from_one_comma_delimited<T: str::FromStr>(raw: &[u8]) -> Option<Vec<T>> {
    match str::from_utf8(raw) {
        Ok(s) => {
            Some(s.as_slice()
                 .split(',')
                 .map(|x| x.trim())
                 .filter_map(|x| x.parse().ok())
                 .collect())
        }
        Err(_) => None
    }
}

/// Format an array into a comma-delimited string.
pub fn fmt_comma_delimited<T: fmt::Display>(fmt: &mut fmt::Formatter, parts: &[T]) -> fmt::Result {
    let last = parts.len() - 1;
    for (i, part) in parts.iter().enumerate() {
        try!(write!(fmt, "{}", part));
        if i < last {
            try!(write!(fmt, ", "));
        }
    }
    Ok(())
}

/// Get a Tm from HTTP date formats.
//    Prior to 1995, there were three different formats commonly used by
//   servers to communicate timestamps.  For compatibility with old
//   implementations, all three are defined here.  The preferred format is
//   a fixed-length and single-zone subset of the date and time
//   specification used by the Internet Message Format [RFC5322].
//
//     HTTP-date    = IMF-fixdate / obs-date
//
//   An example of the preferred format is
//
//     Sun, 06 Nov 1994 08:49:37 GMT    ; IMF-fixdate
//
//   Examples of the two obsolete formats are
//
//     Sunday, 06-Nov-94 08:49:37 GMT   ; obsolete RFC 850 format
//     Sun Nov  6 08:49:37 1994         ; ANSI C's asctime() format
//
//   A recipient that parses a timestamp value in an HTTP header field
//   MUST accept all three HTTP-date formats.  When a sender generates a
//   header field that contains one or more timestamps defined as
//   HTTP-date, the sender MUST generate those timestamps in the
//   IMF-fixdate format.
pub fn tm_from_str(s: &str) -> Option<time::Tm> {
    time::strptime(s, "%a, %d %b %Y %T %Z").or_else(|_| {
        time::strptime(s, "%A, %d-%b-%y %T %Z")
    }).or_else(|_| {
        time::strptime(s, "%c")
    }).ok()
}

#[cfg(test)]
mod tests {
    use time::Tm;
    use super::tm_from_str;

    const NOV_07: Tm = Tm {
        tm_nsec: 0,
        tm_sec: 37,
        tm_min: 48,
        tm_hour: 8,
        tm_mday: 7,
        tm_mon: 10,
        tm_year: 94,
        tm_wday: 0,
        tm_isdst: 0,
        tm_yday: 0,
        tm_utcoff: 0,
    };

    #[test]
    fn test_imf_fixdate() {
        assert_eq!(tm_from_str("Sun, 07 Nov 1994 08:48:37 GMT"),
                   Some(NOV_07));
    }

    #[test]
    fn test_rfc_850() {
        assert_eq!(tm_from_str("Sunday, 07-Nov-94 08:48:37 GMT"),
                   Some(NOV_07));
    }

    #[test]
    fn test_asctime() {
        assert_eq!(tm_from_str("Sun Nov  7 08:48:37 1994"),
                   Some(NOV_07));
    }


}
