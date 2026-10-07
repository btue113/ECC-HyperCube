//! Reads jobs from stdin and prints the result of each one.
//!
//! Input: a job count, then one job per line: `r m activity cache data`
//! (activity `e` = encode, `d` = decode). The cache flag is accepted for
//! compatibility with the original C++ tool and ignored: cubes are
//! enumerated on the fly, so nothing needs caching.

use ecc_hypercube::{format_bits, parse_bits, ReedMuller};
use std::error::Error;
use std::io::{self, BufWriter, Read, Write};
use std::process::ExitCode;
use std::time::Instant;

const SEPARATOR: &str = "--------------------------------";

fn main() -> ExitCode {
    let mut input = String::new();
    if let Err(e) = io::stdin().read_to_string(&mut input) {
        eprintln!("error: reading stdin: {e}");
        return ExitCode::FAILURE;
    }
    let mut tokens = input.split_whitespace();
    let jobs: usize = match tokens.next().map(str::parse) {
        Some(Ok(n)) => n,
        _ => {
            eprintln!("error: first token must be the number of jobs");
            return ExitCode::FAILURE;
        }
    };

    let mut out = BufWriter::new(io::stdout().lock());
    let mut status = ExitCode::SUCCESS;
    for job in 1..=jobs {
        let fields: Vec<&str> = tokens.by_ref().take(5).collect();
        if fields.len() < 5 {
            eprintln!("error: expected {jobs} jobs, found {}", job - 1);
            return ExitCode::FAILURE;
        }
        if let Err(e) = run_job(&mut out, &fields) {
            eprintln!("error: job {job}: {e}");
            // a bad job is skipped; failing to write output ends the run
            if e.is::<io::Error>() {
                return ExitCode::FAILURE;
            }
            status = ExitCode::FAILURE;
        }
    }
    if let Err(e) = out.flush() {
        eprintln!("error: {e}");
        return ExitCode::FAILURE;
    }
    status
}

fn run_job(out: &mut impl Write, fields: &[&str]) -> Result<(), Box<dyn Error>> {
    let r: u32 = fields[0].parse()?;
    let m: u32 = fields[1].parse()?;
    let activity = fields[2];
    let data = parse_bits(fields[4])?;

    let start = Instant::now();
    let rm = ReedMuller::new(r, m)?;
    writeln!(out, "Initialization in:{} sec.", fmt_g(start.elapsed().as_secs_f64()))?;
    writeln!(out, "Message length(k):{}", rm.message_len())?;
    writeln!(out, "Block length(n):{}", rm.block_len())?;
    writeln!(out, "Rate(k/n):{}", fmt_g(rm.rate()))?;
    writeln!(out, "Bit error tolerance:{} bit(s)", rm.error_tolerance())?;

    let start = Instant::now();
    match activity {
        "e" => {
            let encoded = rm.encode(&data)?;
            writeln!(out, "message:{}", fields[4])?;
            writeln!(out, "encoded:{}", format_bits(&encoded))?;
            writeln!(out, "encoded in:{} sec.", fmt_g(start.elapsed().as_secs_f64()))?;
        }
        "d" => {
            let decoded = rm.decode(&data)?;
            writeln!(out, "received_code:{}", fields[4])?;
            writeln!(out, "______decoded:{}", format_bits(&decoded))?;
            writeln!(out, "decoded in:{} sec.", fmt_g(start.elapsed().as_secs_f64()))?;
        }
        other => return Err(format!("unknown activity {other:?}, expected e or d").into()),
    }
    writeln!(out, "{SEPARATOR}")?;
    Ok(())
}

/// Formats like C++ iostream's default (`%g`, 6 significant digits).
fn fmt_g(x: f64) -> String {
    if x == 0.0 || !x.is_finite() {
        return format!("{x}");
    }
    let exp = x.abs().log10().floor() as i32;
    if !(-4..6).contains(&exp) {
        // C prints at least two exponent digits: 6.10352e-05
        let s = format!("{x:.5e}");
        let (mantissa, e) = s.split_once('e').unwrap();
        let mantissa = mantissa.trim_end_matches('0').trim_end_matches('.');
        let (sign, digits) = e.strip_prefix('-').map_or(("+", e), |d| ("-", d));
        return format!("{mantissa}e{sign}{digits:0>2}");
    }
    let s = format!("{:.*}", (5 - exp).max(0) as usize, x);
    if s.contains('.') {
        s.trim_end_matches('0').trim_end_matches('.').to_string()
    } else {
        s
    }
}

#[cfg(test)]
mod tests {
    use super::fmt_g;

    #[test]
    fn g_format() {
        assert_eq!(fmt_g(0.3125), "0.3125");
        assert_eq!(fmt_g(1.0 / 3.0), "0.333333");
        assert_eq!(fmt_g(0.5), "0.5");
        assert_eq!(fmt_g(0.0), "0");
        assert_eq!(fmt_g(0.000123456789), "0.000123457");
        assert_eq!(fmt_g(1.0 / 16384.0), "6.10352e-05");
        assert_eq!(fmt_g(1e-5), "1e-05");
        assert_eq!(fmt_g(1234567.0), "1.23457e+06");
    }
}
