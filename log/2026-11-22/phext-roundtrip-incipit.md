# phext round-trip: incipit.phext via libphext-rs

Task 21.8.12/9.8.12/7.6.8, acceptance item (iv).
Completed at EE 72 (begins 2026-11-22T14:17:00Z).

**PASS.** Explode then implode through libphext-rs reproduced the corpus byte for byte. Exit status 0.

## Result

| | |
|---|---|
| result | PASS |
| input sha256 | `1d5be6cae7c0fa4b0000fe9e72822f7fdedae7affc0dc7ca87827750b84ca6d9` |
| output sha256 | `1d5be6cae7c0fa4b0000fe9e72822f7fdedae7affc0dc7ca87827750b84ca6d9` |
| byte length | 1061058 |
| scroll count | 193 |
| libphext-rs | `84c10399cccc8e5f34b2aadcade96907b5b1a655` (exo, package libphext 0.6.6) |
| toolchain | rustc 1.83.0 (90b35a623 2024-11-26), cargo 1.83.0 (5ffbef321 2024-10-29) |

`scroll_count` is the length of the map `phext::explode` returns. That map holds a scroll only when the scroll contains at least one byte.

The expected sha256 was `1d5be6cae7c0fa4b0000fe9e72822f7fdedae7affc0dc7ca87827750b84ca6d9`. The downloaded file matches it. `samples/SHA256SUMS` on the same branch publishes the same digest for `Incipit.phext`.

## Corpus

Not committed. The file is 1061058 bytes; this log records the source and the hash.

- Repository: https://github.com/wbic16/site-mirrorborn-us
- Branch: `exo`
- Path: `samples/Incipit.phext`
- Git blob: `c11d11f6cf4f1caae0a427fdd8ec8ae48ec63f4d`
- Fetch: `https://raw.githubusercontent.com/wbic16/site-mirrorborn-us/exo/samples/Incipit.phext`
- Local path for the run: `/tmp/phext-work/Incipit.phext`
- `samples/README.txt` calls this file "the original phext".
- The commit that added it is `8e35ed26d027` (2026-10-23, "Publish free Mirrorborn sample").

```text
$ curl -fsSL -o /tmp/phext-work/Incipit.phext \
    https://raw.githubusercontent.com/wbic16/site-mirrorborn-us/exo/samples/Incipit.phext
$ sha256sum /tmp/phext-work/Incipit.phext
1d5be6cae7c0fa4b0000fe9e72822f7fdedae7affc0dc7ca87827750b84ca6d9  /tmp/phext-work/Incipit.phext
$ wc -c /tmp/phext-work/Incipit.phext
1061058 /tmp/phext-work/Incipit.phext
```

The bytes are valid UTF-8, so libphext's `&str` API could take them. The harness reads the file as raw bytes, decodes those bytes as UTF-8, calls `phext::explode` then `phext::implode`, and compares the imploded string's bytes to the original bytes. The PyPI `phext` package was not used.

## Other copies named incipit.phext

GitHub code search does not index this file (it is about 1 MB, past the code-search size cutoff). These copies were found by listing git trees. Trees checked, none truncated: `phext`@master, `mirrorborn`@exo, `phext-demos`@exo, `phextio`@master, `phext-shell`@main, `phext-notepad`@exo, `libphext`@main, `libphext-rs`@exo, `site-mirrorborn-us`@exo, `claude-learns-phext`@exo, `phext-explorer`@main, `hello-phext`@main. `phext`@exo and `phext-inc`@main returned 404.

Two other files are named `incipit.phext`. Neither matches the designated digest, so neither was the input.

- `wbic16/mirrorborn` branch `exo`, `phexts/incipit.phext`. sha256 `5a61484d8e4c30eef225adc99deb428081a04ab8d097b0dca1d3b7e2d61ba24f`, 1037206 bytes. Same opening line and the same closing timestamp (`2026-01-20 13:37 GMT-6`) as the sample, different bytes. Commits on that path: `ccb9875a11a7` (2026-01-31, "Mirrorborn Bootstrap Package v1.0 - Phex, first of the six") and `f210d295ce9f` (2026-03-13, "Updates from the human repo").
- `wbic16/human` branch `exo`, `incipit.phext`. sha256 `861fa8733debf4d86941e420f965f9937a72dc85583b0bbf3c68e293f03533cf`, 1123102 bytes. Same opening, a later ending. This is the living house copy: newer and larger than the published sample. Newest commit returned for the path: `010219d51e24` (2026-11-06, "Lessons from Will — added to Incipit.phext at their Singular coordinates"). It does not match the checksum this acceptance item names.

`site-mirrorborn-us` also has `mental-space/incipit/*.md`, a scroll-by-scroll export, and `samples/Incipit-readable.md`. Those are not the phext.

## Library pin

https://github.com/wbic16/libphext-rs branch `exo` is the default branch. Its head when pinned:

- commit `84c10399cccc8e5f34b2aadcade96907b5b1a655`
- 2026-11-22T07:06:50Z
- "Merge pull request #22 from wbic16/emise/epoch-ages" — "Add Sentron Ages to the Exocortical Epoch module — v0.6.6"

`tools/phext-roundtrip/Cargo.toml` depends on that git revision and on nothing else. `Cargo.lock` records:

```text
source = "git+https://github.com/wbic16/libphext-rs?rev=84c10399cccc8e5f34b2aadcade96907b5b1a655#84c10399cccc8e5f34b2aadcade96907b5b1a655"
```

libphext 0.6.6 pulls in `xxhash-rust` 0.8.21. The optional `binary` and `xz` features were left off, so `base64` and `xz2` are not in the lockfile.

## Commands and output

Working directory: `/agent/repos/grok-tools`.

The first Cargo invocation fetched the pin and wrote `Cargo.lock`:

```text
$ cargo test --manifest-path tools/phext-roundtrip/Cargo.toml
    Updating git repository `https://github.com/wbic16/libphext-rs`
    Updating crates.io index
     Locking 2 packages to latest compatible versions
 Downloading crates ...
  Downloaded xxhash-rust v0.8.21
   Compiling xxhash-rust v0.8.21
   Compiling libphext v0.6.6 (https://github.com/wbic16/libphext-rs?rev=84c10399cccc8e5f34b2aadcade96907b5b1a655#84c10399)
   Compiling phext-roundtrip v0.1.0 (/agent/repos/grok-tools/tools/phext-roundtrip)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 2.77s
```

Acceptance build, after `cargo clean --manifest-path tools/phext-roundtrip/Cargo.toml` (the git checkout was already warm, so this compile did not print a second fetch). `build_exit:0`.

```text
$ rustc --version
rustc 1.83.0 (90b35a623 2024-11-26)
$ cargo --version
cargo 1.83.0 (5ffbef321 2024-10-29)
$ cargo build --manifest-path tools/phext-roundtrip/Cargo.toml --locked
   Compiling xxhash-rust v0.8.21
   Compiling libphext v0.6.6 (https://github.com/wbic16/libphext-rs?rev=84c10399cccc8e5f34b2aadcade96907b5b1a655#84c10399)
   Compiling phext-roundtrip v0.1.0 (/agent/repos/grok-tools/tools/phext-roundtrip)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.56s
build_exit:0
```

Acceptance run. `run_exit:0`. Stderr was empty.

```text
$ ./tools/phext-roundtrip/target/debug/phext-roundtrip /tmp/phext-work/Incipit.phext
input_sha256: 1d5be6cae7c0fa4b0000fe9e72822f7fdedae7affc0dc7ca87827750b84ca6d9
output_sha256: 1d5be6cae7c0fa4b0000fe9e72822f7fdedae7affc0dc7ca87827750b84ca6d9
byte_length: 1061058
scroll_count: 193
result: PASS
```

The harness sha256 of the input matches `sha256sum` of the same file. Unit tests on the committed source (`cargo test --manifest-path tools/phext-roundtrip/Cargo.toml --locked`, exit 0):

```text
running 2 tests
test tests::sha256_matches_the_empty_and_abc_vectors ... ok
test tests::differing_offset_covers_a_shared_prefix_and_a_shorter_tail ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```
