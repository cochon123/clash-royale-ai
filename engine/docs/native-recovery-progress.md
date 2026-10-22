# Native recovery progress

The requested result is the original Clash Royale behavior and interface in a
Rust desktop application. Recovering package data or drawing an original HUD
does not establish a recovered battle engine. The earlier Rust simulation is
retained separately and is not used as evidence of native equivalence.

## Protection and engine loader

The ARM64 `libg.so` contains encrypted executable bytes. Its initializer locates
`libsupercell_clashroyale.so` through `/proc/self/maps` and calls the companion's
helper at RVA `0x681780`. Static analysis of the encrypted function bodies alone
does not produce meaningful battle algorithms.

An isolated Android startup capture supplied relocated companion memory. The
native routine at companion RVA `0x65bb90` was then executed in Unicorn using
only an explicit errno cell and virtual `mprotect` calls. It returned 1 and
changed 4,841,564 bytes in the 4,860,456-byte region beginning at RVA `0x18d0a0`.

- [Reproduction script](../tools/native_entry_decrypt_companion.py)
- [Execution report](../analysis/native-entry/companion-text-decrypt/report.json)
- Input memory SHA-256: `747a1754c91ace13e3f2216638b187c2f66d2084897ee5b239fcd00e83cd4424`
- Derived memory SHA-256: `714b5e3979b263917a1b85c029b0a84d07c515404f1085b1c27f41bed55d4b56`

The newly readable callback at RVA `0x19fee0` writes two real function pointers
into the loader's descriptor: `0x19fcec` and `0x19fd60`. Executing the second
against the captured state returns the actual companion base. The first consults
a singleton's startup state before looking up the game's protected metadata.

The [game initializer probe](../tools/native_unpack.py) now maps that decrypted
companion memory at its capture address and executes this handshake. Android
syscalls, memory operations, process IDs and uncontended mutexes are explicit
analysis models recorded in each report. An unresolved call or invalid access
stops execution. Derived ELF copies restore original relocation slots and must
not be mistaken for verified runnable Android libraries.

The captured process did not finish companion initialization: its state word is
zero and its metadata tree is empty. The loader requests entry `0x7e33eb5e`;
the real callback follows an exception path instead of supplying metadata.
That exact binding has since been recovered independently from the original
APK's `assets/anjeamelnchn.dat`:

1. The `lbts` envelope contains three tagged blocks. Block `0x30` is a signed
   protobuf carrying 416 protected bindings. Its signature is not verified by
   the wire extractor.
2. The requested record's ciphertext begins at original asset offset 883,541
   and contains 736 bytes. [Extraction](../tools/native_recover_metadata.py)
   checks the supplied APK and asset hashes and retains every source offset.
   See the [extraction manifest](../analysis/native-entry/metadata-recovery.json).
3. Executing original companion constructor RVA `0x2b95dc` and decoder RVA
   `0x2170b0` produces exactly 672 bytes, matching the game's request. The
   decoder uses its native generated table and cipher code, not a guessed key.
   The [execution report](../analysis/native-unpack/companion-binding-decoded/report.json)
   records input/output hashes and every modeled import category. The output
   SHA-256 is `29f73c4325b635d46233ca040c0048cad83570b7cad0fb456099785140f1bb83`.
4. Supplying those recovered bytes as the known lookup result lets `libg.so`'s
   own initializer begin decrypting its main image. This explicitly replaces
   the unavailable startup metadata lookup; it does not claim completed
   Android startup. The first bounded 55-second run changed 478,999 bytes.
   Complete CPU/memory checkpoints then enabled an independently verified
   instruction translation and recovery of the exact cipher.
5. The [Rust decoder](../tools/libg_cipher_decode.rs) implements the recovered
   Speck64/128-CBC operations and metadata layout. It decodes all five protected
   ranges (19,190,488 bytes). Every byte matches completed original loader
   execution, which returned successfully. The [comparison evidence](../analysis/libg-cipher/direct-all-ranges-proof.json)
   retains hashes for each range. See [cipher recovery](libg-cipher.md) for
   commands, calling boundaries and verification limits.

The source-preserving, fully decoded analysis copy is
`analysis/libg-cipher/libg.decrypted.so`, SHA-256
`79fe6612e20922d4c4086b1e6c939464202cefa7cb3b20344236529f941423d6`.
Its real JNI entry points now disassemble normally. REA/Ghidra is analyzing
this copy to recover game state, the battle loop and UI bindings.

Eight deck leaf operations now have Rust ports in `native-engine` and match
the original ARM64 instructions across 8,192 controlled cases. The report is
`analysis/native-engine-index/deck-leaf-rust-differential.json`. The covered
operations are pointer swap, identity search, both nonzero predicates, the
in-range indexed load, and three field loads. They do not establish hand
rotation, card identity, or a battle tick.

## Original battle execution and Rust host

The original native resource/bootstrap chain now completes, including full
typed postload, original SCDB prefab jobs and all 209 Location map loads. The
original `Normal` harness creates configuration, decks, tilemap and six initial
entities, then its original AI loop reaches the native completion flag at tick
1,354. A second run from the same prepared state has identical saved memory,
architectural registers, allocator state and import counts. The source-bound
chain and scope are recorded in [native-battle-loop.md](native-battle-loop.md).

The [Rust oracle host](../native-engine/README.md) now restores that prepared
state and executes the same original ARM64 functions through the Unicorn C API.
Independent Python/Rust comparisons pass for one world update, 100 world
updates and the complete AI match: all 12 saved regions, all 70 architectural
registers and full allocator metadata agree. The complete-match result is in
`analysis/native-oracle/full-match-comparison.json`. The warm Rust-hosted
emulation took 13.93 seconds for 1,352 additional ticks, excluding snapshot
loading. This is original binary execution and does not measure a Rust combat
implementation.

The recovered [card-play command](native-human-card-command.md) also has a
source-proven path from live native hand selection through the original
constructor, position/card setters, actor/tick attachment, validator and command
queue. An externally selected Mortar spends elixir, completes deployment,
creates a seventh entity and cycles the hand without bot decisions. The Rust
CLI supports that input and its complete state matches the independent Python
adapter in `analysis/native-oracle/manual-card-comparison.json`. Invalid target
and insufficient-elixir cases are rejected without queue insertion. Combat
component ports, interactive human/AI controller selection and the original
battle presentation remain work in progress.

The [minimal RL environment](../rl/README.md) now exposes persistent native
`reset`/`step` through Gymnasium. It supports external card actions for both
sides, partial observations and sparse rewards from the recovered signed native
winner field. The engine resets to the same original snapshot/decks while
Python opponent sampling is seeded separately. Gymnasium checks, reset and
truncation checks, and a complete Python-controlled native episode pass.
The first end-to-end benchmark completed 12 native games in 61.476 seconds
(11.71 per minute), including resets, observations and actions, with no errors
or truncated episodes. Unit positions and health remain absent from this first
observation interface; the result measures a basic RL loop with random policies,
not trained-agent performance.

## Original interface

The original assets provide movieclips, frames, meshes, matrices, colors, blend
modes and atlas images. The [Rust viewer](../native-ui-viewer/README.md) consumes
these recovered structures. All 56 sampled states match an independent Python
decoder pixel for pixel, including canvas origin: the player and spectator HUDs,
two components with masks, and all 52 frames of `ability_ingame_button`.
The [comparison report](../analysis/native-ui/rust-verification/comparison.json)
retains source identities, per-case results and the tested Rust binary hash.

These checks validate the sampled asset playback, not a running native game UI.
Native text layout, visibility/state bindings, card identities and game-driven
events still require recovery. See [UI evidence](native-ui-recovery.md).

## Android runtime

The x86_64 Android lab accepts the supplied ARM64 APK split through Android's
native bridge. The original application exits during protection startup before
mapping `libg.so`; no exact reason has been established. A separate API 29 ARM64
system image runs through QEMU software execution, with native bridge disabled,
to distinguish translation-related failures from other startup conditions.
It completed boot and launched four unchanged original APK splits. Its baseline
returns `JNI_ERR` from the companion's `JNI_OnLoad` before loading `libg.so`;
the [retained log](../analysis/android-arm-runtime/baseline-startup-logcat.txt)
records this result. The exact cause is unresolved. The large original asset
pack had not yet been installed in this separate lab.

The ARM lab uses a project-local compatibility metadata directory to select
ARM-compatible emulator devices; it does not change guest OS binaries or APK
bytes. It required restoring `/dev/input` after disabling incompatible emulator
PCI input devices. Runtime scripts and captured logs retain these interventions.

No account or live battle has been used. The lab is configured for local offline
analysis. Faithful Rust battle behavior and an offline original-client server
remain unimplemented.
