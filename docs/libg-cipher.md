# Recovery of libg protected code and data

The original Clash Royale 160402020 ARM64 `libg.so` can now be decoded offline.
This recovers the shipped engine's native machine code and protected strings.
It does not establish that Android startup works, or that the Rust game engine
has the same behavior as the original.

## Source identity

| Artifact | SHA256 |
| --- | --- |
| Original `analysis/native/lib/arm64-v8a/libg.so` | `347afc57c8d168bb67d0defa488a89f76297025cac1e8544b5182bb7e70ff433` |
| Native-decoded 672-byte binding metadata | `29f73c4325b635d46233ca040c0048cad83570b7cad0fb456099785140f1bb83` |
| Derived `analysis/libg-cipher/libg.decrypted.so` | `79fe6612e20922d4c4086b1e6c939464202cefa7cb3b20344236529f941423d6` |

The metadata is `analysis/native-unpack/companion-binding-decoded/binding-decoded.bin`.
The parent investigation recovered it by executing the original companion codec
at RVA `0x2170b0`. The offline decoder checks both input SHA256 values before
writing anything and refuses to overwrite an existing output.

## Reproduce the independent Rust decoder

```sh
rustc -O --edition=2021 tools/libg_cipher_decode.rs \
  -o analysis/libg-cipher/libg_cipher_decode
analysis/libg-cipher/libg_cipher_decode \
  analysis/native/lib/arm64-v8a/libg.so \
  analysis/native-unpack/companion-binding-decoded/binding-decoded.bin \
  analysis/libg-cipher/libg.new-decrypted.so
```

The decoder retains the source ELF layout and original relocation slots. It
changes only the five protected file ranges listed below. `direct-decode.json`
records the actual run: 19,190,488 processed bytes in 0.0813 seconds on this host
(including output hashing). This is a decoding measurement, not an engine speed
or reinforcement-learning throughput measurement.

| Virtual address | File offset | Bytes | Key offset in binding | CBC IV |
| --- | --- | ---: | --- | --- |
| `0x2b1680` | `0x2b1680` | 1,158,032 | `0x28` | `4bf53d6b8ede00a0` |
| `0x7706b0` | `0x7706b0` | 17,877,920 | `0xb8` | `99483f140cc95e58` |
| `0x187d250` | `0x187d250` | 6,864 | `0x148` | `4f20f8c729ea91b0` |
| `0x1ab8000` | `0x1ab0000` | 22,904 | `0x1f8` | `35b09421b60c3e3a` |
| `0x1ac0fb0` | `0x1ab8fb0` | 124,768 | `0x1f8` | `28ed80234f8864ba` |

## Exact recovered cipher operations

The loader implements Speck64/128 in CBC mode with 27 already-expanded 32-bit
round keys. Each eight-byte block stores the little-endian words `y`, then `x`.
For keys in reverse order, the original operation is:

```text
y = rotate_right_32(y XOR x, 3)
x = rotate_left_32((x XOR round_key) - y, 8)
```

Subtraction wraps modulo 2^32. After the inverse rounds, the eight output bytes
are XORed with the prior ciphertext block, or the metadata IV for the first
block. Native NEON code processes two eight-byte blocks at a time; the Rust
implementation processes each independently with the same CBC dependency.

This identification was inferred from the original loader's rotates,
subtractions and metadata key layout, then verified against recovered native
execution. It was not chosen by matching plausible plaintext alone.

## Metadata layout observed for this exact build

All scalar fields are little endian. A single outer u64 gives the segment count
(two). Each protected segment has four u64 fields:

```text
segment virtual address
segment size
requested protection (5, executable + readable)
number of cipher contexts
```

Each cipher context contains:

```text
27 x u32 expanded round keys
u32 number of protected ranges
u64 scratch/reserved value (zero in this binding)
range[number of ranges] {
    u64 absolute ELF virtual address
    u64 byte length
    8 raw CBC-IV bytes
}
```

The first segment has three contexts with one range each. The second segment
has one context with two ranges. The final record ends exactly at byte 672.
The scratch/reserved field's higher-level purpose is unresolved; the decoder
requires the observed zero value rather than guessing its semantics.

## Instruction-translation verification

`tools/libg_cipher_cfg.py` follows direct control flow from the original
initializer's loader function at RVA `0x1adfe4c`. Its bounded graph has 2,447
instructions and ten out-of-function calls. `reachable-disassembly.txt` and
`cfg.json` retain the source-bound inventory. The original source digest above
binds the instruction bytes. Loader file offsets equal virtual addresses minus
`0x8000` inside this bounded region.

`tools/libg_cipher_translate.py` translates that exact instruction graph to C++
scalar and NEON-lane operations. This makes the arithmetic fast enough to run
without guessing a high-level cipher. Direct branches remain direct branches.
Every original `BL` and `RET` stops before execution, allowing Unicorn to run
out-of-function instructions and modeled syscalls.

Build and verify:

```sh
tools/reverse-env/bin/python tools/libg_cipher_translate.py
g++ -std=c++17 -O3 -shared -fPIC \
  -o analysis/libg-cipher/loader-translated.so \
  analysis/libg-cipher/loader-translated.cpp
tools/reverse-env/bin/python tools/libg_cipher_verify.py --budgets 1 10 100 1000 10000
```

The retained tests start from the complete source checkpoint
`analysis/native-unpack/game-checkpoint/state/checkpoint.json`. They compare
all X0–X30 registers, Q0–Q31 vector registers, SP, PC, NZCV, and every byte in
all retained game, stack, TLS, and companion mappings. All comparisons match.
`analysis/libg-cipher/differential-verification.json` records the 10,000-branch
comparison. The test source checkpoint remains unchanged.

A second test starts close to the end of the first region and executes through
the loop exit to the next original call at PC `0x11adfb74`. After 7,043
conditional branches all registers, flags, vectors and mapped bytes still
match. Its report is `first-region-end-proof/differential-verification.json`.

The independent Rust decoder then matches all 1,158,032 bytes of the first
region and all 17,877,920 bytes of the native engine code region against the
translated native execution. Reports:

- `analysis/libg-cipher/direct-first-range-proof.json`: 144,754 CBC blocks.
- `analysis/libg-cipher/direct-code-range-proof.json`: 2,234,740 CBC blocks.

All five ranges now match the completed native initializer byte for byte, with
zero differences. `analysis/libg-cipher/direct-all-ranges-proof.json` records
all per-range hashes. The native initializer returned to its outer caller with
no unresolved call or invalid access; its final report is
`analysis/native-unpack/complete-loader/01-native/report.json`. This execution
uses the documented recovered metadata lookup and analysis syscall models.
It does not establish successful Android application startup.

## Checkpoint acceleration API

`tools/libg_cipher_checkpoint.py` accepts a verified source checkpoint and
writes a new checkpoint to a previously absent directory:

```sh
tools/reverse-env/bin/python tools/libg_cipher_checkpoint.py \
  analysis/native-unpack/game-checkpoint/state/checkpoint.json \
  --out analysis/libg-cipher/new-advanced-checkpoint
```

Every input region, the raw Unicorn CPU context, the original binary digest,
the shared-library digest, and the successful differential proof are checked.
Guest addresses use `MAP_FIXED_NOREPLACE`; a collision fails instead of
replacing host mappings. The input checkpoint is immutable. The output includes
normal `checkpoint.json`, raw CPU context, full memory files, new hashes and an
explicit acceleration record. `native_unpack.py --resume-from OUTPUT_DIRECTORY`
can execute the next original out-of-function call.

The C ABI is `int libg_loader_run(State *, uint64_t conditional_budget)` with:

```c
struct State {
    uint64_t x[31], sp;
    unsigned __int128 q[32];
    uint64_t pc, nzcv, status;
};
```

The struct is 800 bytes with 16-byte alignment. `pc` is an RVA; register and
memory pointers retain the original emulated absolute virtual addresses.
Statuses: zero before the budget-th conditional branch; one before `RET`; two
before `BL`; three for an unknown PC. FPCR, FPSR and TLS control registers are
unchanged by this loader translation and preserved in the output checkpoint.

## What this enables and what remains unresolved

The recovered engine exports now decode into ordinary coherent ARM64 code,
including `JNI_OnLoad` (`0x146096c`), `GameApp_createGameMain` (`0x1460bd4`),
`surfaceCreated` (`0x146220c`) and `GameApp_onCreate` (`0x1461ed0`). The derived
ELF can be opened in REA/Ghidra to inspect real game initialization, native UI
calls, data tables, battle simulation and object layouts.

This decoding does not bypass all startup protection, reconstruct the private
server protocol, recover original C++ source, or complete a Rust port. Remaining
work requires identifying the actual battle state transition functions and
verifying recovered behavior against the original native implementation.
