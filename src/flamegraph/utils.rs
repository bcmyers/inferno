use std::io;

use str_stack::StrStack;

/// Writes into the provided `StrStack` a formatted `&str` based on the provided `f64`
/// and provided `precision` (e.g. writes `"2.00"` if `f` is `2.0` and `precision` is `2`).
/// Returns the index into the `StrStack` of the written `&str`.
///
/// TODO: Does `dtoa` do scientific notation? If so, we need to handle that case.
/// If not, this implementation should be fine.
pub(crate) fn write_truncated(
    stack: &mut StrStack,
    mut buf: &mut Vec<u8>,
    f: f64,
    precision: usize,
) -> Result<usize, io::Error> {
    buf.clear();
    dtoa::write(&mut buf, f)?;
    let decimal_position = buf
        .iter()
        .position(|b| *b == '.' as u8)
        .expect("should always contain a '.'");
    let end = if precision == 0 {
        decimal_position
    } else {
        decimal_position + precision + 1
    };
    while buf.len() < end {
        buf.push('0' as u8);
    }
    let slice = &buf[..end];
    let s = unsafe { std::str::from_utf8_unchecked(slice) };
    stack.push(s);
    Ok(stack.len() - 1)
}

/// Writes into the provided `Vec<u8>` a formatted `&str` based on the provided `f64`
/// and provided `precision` (e.g. writes `"2.00"` if `f` is `2.0` and `precision` is `2`).
///
/// TODO: Does `dtoa` do scientific notation? If so, we need to handle that case.
/// If not, this implementation should be fine.
pub(crate) fn write_truncted2(
    mut buf: &mut Vec<u8>,
    f: f64,
    precision: usize,
) -> Result<(), io::Error> {
    buf.clear();
    dtoa::write(&mut buf, f)?;
    let decimal_position = buf
        .iter()
        .position(|b| *b == '.' as u8)
        .expect("should always contain a '.'");
    let end = if precision == 0 {
        decimal_position
    } else {
        decimal_position + precision + 1
    };
    while buf.len() < end {
        buf.push('0' as u8);
    }
    buf.truncate(end);
    Ok(())
}
