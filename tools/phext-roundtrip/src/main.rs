//! Read a phext as raw bytes, explode it to scrolls with libphext-rs, implode
//! those scrolls, and compare the result to the input byte for byte.
//!
//! libphext's explode and implode speak `&str` / `String`. The bytes are
//! decoded as UTF-8 (a phext is text) and the imploded string's bytes are
//! what gets compared. Nothing else rewrites the buffer.
//!
//! The corpus is not vendored. Pass a path:
//!
//! ```text
//! phext-roundtrip samples/Incipit.phext
//! ```
//!
//! Exit 0 when the bytes match. Exit 1 on a mismatch. Exit 2 when the file
//! cannot be read or is not UTF-8.

use std::env;
use std::fs;
use std::io::{self, Write};
use std::process::ExitCode;

use libphext::phext;

fn main() -> ExitCode {
    let mut args = env::args_os();
    let _argv0 = args.next();
    let Some(path) = args.next() else {
        eprintln!("usage: phext-roundtrip <file.phext>");
        return ExitCode::from(2);
    };
    if args.next().is_some() {
        eprintln!("usage: phext-roundtrip <file.phext>");
        return ExitCode::from(2);
    }

    let input = match fs::read(&path) {
        Ok(bytes) => bytes,
        Err(err) => {
            eprintln!("read {}: {err}", path.to_string_lossy());
            return ExitCode::from(2);
        }
    };

    let text = match std::str::from_utf8(&input) {
        Ok(text) => text,
        Err(err) => {
            eprintln!("input is not UTF-8 at byte {}: {err}", err.valid_up_to());
            let _ = print_report(&input, None, None, None);
            return ExitCode::from(2);
        }
    };

    let scrolls = phext::explode(text);
    let scroll_count = scrolls.len();
    let output = phext::implode(scrolls).into_bytes();
    let mismatch = first_differing_offset(&input, &output);

    if print_report(&input, Some(&output), Some(scroll_count), mismatch).is_err() {
        return ExitCode::from(2);
    }
    if mismatch.is_some() {
        ExitCode::from(1)
    } else {
        ExitCode::SUCCESS
    }
}

/// `None` for the differing offset means the buffers match.
fn print_report(
    input: &[u8],
    output: Option<&[u8]>,
    scroll_count: Option<usize>,
    mismatch: Option<usize>,
) -> io::Result<()> {
    let mut stdout = io::stdout().lock();
    writeln!(stdout, "input_sha256: {}", hex(&sha256(input)))?;
    match output {
        Some(output) => writeln!(stdout, "output_sha256: {}", hex(&sha256(output)))?,
        None => writeln!(stdout, "output_sha256:")?,
    }
    writeln!(stdout, "byte_length: {}", input.len())?;
    match scroll_count {
        Some(count) => writeln!(stdout, "scroll_count: {count}")?,
        None => writeln!(stdout, "scroll_count:")?,
    }
    if let Some(offset) = mismatch {
        if let Some(output) = output {
            writeln!(stdout, "output_byte_length: {}", output.len())?;
        }
        writeln!(stdout, "first_differing_offset: {offset}")?;
        writeln!(stdout, "result: FAIL")?;
    } else if output.is_some() {
        writeln!(stdout, "result: PASS")?;
    }
    stdout.flush()
}

/// First index where the slices differ. When one is a proper prefix of the
/// other, that index is the shorter length. `None` when the slices are equal.
fn first_differing_offset(left: &[u8], right: &[u8]) -> Option<usize> {
    let shared = left.len().min(right.len());
    for index in 0..shared {
        if left[index] != right[index] {
            return Some(index);
        }
    }
    if left.len() != right.len() {
        Some(shared)
    } else {
        None
    }
}

fn hex(bytes: &[u8]) -> String {
    const DIGITS: &[u8; 16] = b"0123456789abcdef";
    let mut out = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        out.push(DIGITS[(byte >> 4) as usize] as char);
        out.push(DIGITS[(byte & 0x0f) as usize] as char);
    }
    out
}

fn sha256(message: &[u8]) -> [u8; 32] {
    const K: [u32; 64] = [
        0x428a2f98, 0x71374491, 0xb5c0fbcf, 0xe9b5dba5, 0x3956c25b, 0x59f111f1, 0x923f82a4,
        0xab1c5ed5, 0xd807aa98, 0x12835b01, 0x243185be, 0x550c7dc3, 0x72be5d74, 0x80deb1fe,
        0x9bdc06a7, 0xc19bf174, 0xe49b69c1, 0xefbe4786, 0x0fc19dc6, 0x240ca1cc, 0x2de92c6f,
        0x4a7484aa, 0x5cb0a9dc, 0x76f988da, 0x983e5152, 0xa831c66d, 0xb00327c8, 0xbf597fc7,
        0xc6e00bf3, 0xd5a79147, 0x06ca6351, 0x14292967, 0x27b70a85, 0x2e1b2138, 0x4d2c6dfc,
        0x53380d13, 0x650a7354, 0x766a0abb, 0x81c2c92e, 0x92722c85, 0xa2bfe8a1, 0xa81a664b,
        0xc24b8b70, 0xc76c51a3, 0xd192e819, 0xd6990624, 0xf40e3585, 0x106aa070, 0x19a4c116,
        0x1e376c08, 0x2748774c, 0x34b0bcb5, 0x391c0cb3, 0x4ed8aa4a, 0x5b9cca4f, 0x682e6ff3,
        0x748f82ee, 0x78a5636f, 0x84c87814, 0x8cc70208, 0x90befffa, 0xa4506ceb, 0xbef9a3f7,
        0xc67178f2,
    ];

    let mut state: [u32; 8] = [
        0x6a09e667, 0xbb67ae85, 0x3c6ef372, 0xa54ff53a, 0x510e527f, 0x9b05688c, 0x1f83d9ab,
        0x5be0cd19,
    ];

    let bit_len = (message.len() as u64).wrapping_mul(8);
    let mut padded = Vec::with_capacity(message.len() + 72);
    padded.extend_from_slice(message);
    padded.push(0x80);
    while padded.len() % 64 != 56 {
        padded.push(0);
    }
    padded.extend_from_slice(&bit_len.to_be_bytes());

    for chunk in padded.chunks_exact(64) {
        let mut words = [0u32; 64];
        for index in 0..16 {
            let start = index * 4;
            words[index] = u32::from_be_bytes(chunk[start..start + 4].try_into().unwrap());
        }
        for index in 16..64 {
            let s0 = words[index - 15].rotate_right(7)
                ^ words[index - 15].rotate_right(18)
                ^ (words[index - 15] >> 3);
            let s1 = words[index - 2].rotate_right(17)
                ^ words[index - 2].rotate_right(19)
                ^ (words[index - 2] >> 10);
            words[index] = words[index - 16]
                .wrapping_add(s0)
                .wrapping_add(words[index - 7])
                .wrapping_add(s1);
        }

        let mut a = state[0];
        let mut b = state[1];
        let mut c = state[2];
        let mut d = state[3];
        let mut e = state[4];
        let mut f = state[5];
        let mut g = state[6];
        let mut h = state[7];
        for index in 0..64 {
            let s1 = e.rotate_right(6) ^ e.rotate_right(11) ^ e.rotate_right(25);
            let choose = (e & f) ^ (!e & g);
            let temp1 = h
                .wrapping_add(s1)
                .wrapping_add(choose)
                .wrapping_add(K[index])
                .wrapping_add(words[index]);
            let s0 = a.rotate_right(2) ^ a.rotate_right(13) ^ a.rotate_right(22);
            let majority = (a & b) ^ (a & c) ^ (b & c);
            let temp2 = s0.wrapping_add(majority);
            h = g;
            g = f;
            f = e;
            e = d.wrapping_add(temp1);
            d = c;
            c = b;
            b = a;
            a = temp1.wrapping_add(temp2);
        }
        state[0] = state[0].wrapping_add(a);
        state[1] = state[1].wrapping_add(b);
        state[2] = state[2].wrapping_add(c);
        state[3] = state[3].wrapping_add(d);
        state[4] = state[4].wrapping_add(e);
        state[5] = state[5].wrapping_add(f);
        state[6] = state[6].wrapping_add(g);
        state[7] = state[7].wrapping_add(h);
    }

    let mut digest = [0u8; 32];
    for (index, word) in state.iter().enumerate() {
        digest[index * 4..index * 4 + 4].copy_from_slice(&word.to_be_bytes());
    }
    digest
}

#[cfg(test)]
mod tests {
    use super::{first_differing_offset, hex, sha256};

    #[test]
    fn sha256_matches_the_empty_and_abc_vectors() {
        assert_eq!(
            hex(&sha256(b"")),
            "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
        );
        assert_eq!(
            hex(&sha256(b"abc")),
            "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad"
        );
    }

    #[test]
    fn differing_offset_covers_a_shared_prefix_and_a_shorter_tail() {
        assert_eq!(first_differing_offset(b"abcd", b"abcd"), None);
        assert_eq!(first_differing_offset(b"abXd", b"abYd"), Some(2));
        assert_eq!(first_differing_offset(b"ab", b"abcd"), Some(2));
        assert_eq!(first_differing_offset(b"", b"a"), Some(0));
    }
}
