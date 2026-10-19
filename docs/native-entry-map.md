# Original Android entry and protection map

This investigation concerns the supplied APK version `160402020`. The native
startup code is separate from the approximate Rust simulator. The artifacts
below recover original Java control flow and actual ARM64 protection code.
They do not by themselves establish a recovered battle engine or a working
desktop port of the original interface.

## Java to native boundary

Original APK SHA-256:
`0d0dfed74fdb755b71f5424e70af9234ed25471c029d09863b657fed9057b13e`.
REA method evidence and focused JADX source exports are retained in
`analysis/native-entry/`. Exporting source does not execute APK code.

1. The manifest names `com.supercell.titan.TitanApplication` as the Application
   and `com.supercell.titan.GameApp` as the launcher Activity.
2. `TitanApplication` extends the protection wrapper `lskumcyej.Z`.
   `Z` and the Application static initializers invoke `C0073aj.c()`.
   That class's `b()` explicitly loads `supercell_clashroyale`.
3. Protection class initialization registers native methods through
   `C0067ad.a(Class,int)`. Decoded Java string access follows
   `aw.a(int) -> av.a(int) -> C0091m.b(int)`, whose final method is native.
   These `C00...` class names are JADX display aliases; the decompiled
   comments retain their original short names (`aj`, `ad`, `m`).
4. `TitanApplication.onCreate()` loads library names returned by string IDs
   210/211, then attempts `LibsSideload.loadLibG()` and fallback ID 214.
   The supplied `LibsSideload.loadLibG(Context)` body returns `false`.
   Successful native loading sets `nativeLibrariesLoaded`, invokes
   `nOnNativeLibrariesLoaded()`, then `GameApp.nOnApplicationCreate()`.
5. `GameApp.onCreate()` supplies the original `AssetManager`, file paths,
   screen dimensions and Activity to native `createGameMain(...)`.
   `GL2JNISurfaceView` is an Android `SurfaceView`: it forwards surface
   lifecycle and touch coordinates to native methods. The game's ordinary
   arena UI and simulation are not implemented in this Java view.

Key exported `libg.so` RVAs recovered directly from `PT_DYNAMIC` symbols:

| Symbol | RVA | Exported byte size |
| --- | --- | --- |
| `JNI_OnLoad` | `0x146096c` | 144 |
| `GameApp_createGameMain` | `0x1460bd4` | 1892 |
| `GameApp_nOnCreate` | `0x1461ed0` | 92 |
| `GameApp_nOnSurfaceCreated` | `0x146220c` | 120 |
| `GameApp_nOnTouchEvent` | `0x14616d4` | 112 |
| `TitanApplication_nOnNativeLibrariesLoaded` | `0x1461e84` | 76 |

The original exported entry bytes are protected, so symbol recovery alone
does not recover those functions. Complete symbol inventories are
`libg.so-exports.json` and `libsupercell_clashroyale.so-exports.json`.

## Actual companion bootstrap recovered

Original `libsupercell_clashroyale.so` SHA-256:
`20132160e841a00740ef818e0ae2796508cbdd4f5173834f53e7c6d5af045f2f`.
It exports `JNI_OnLoad` at RVA `0x68179c`; its constructor is RVA
`0x65ccf0`, obtained from the `DT_INIT_ARRAY` relative relocation.

The exported OnLoad and helper `0x681780` dispatch through globals
`0x6c6468` and `0x6c6470`. The independently acquired parent-process
memory manifest records runtime base `0x7fda04465000` and SHA-256
`747a1754c91ace13e3f2216638b187c2f66d2084897ee5b239fcd00e83cd4424`.
In that captured state, those globals identify original OnLoad implementation
`0x64449c` and the library API getter `0x19fee0`.

`0x64449c` calls bootstrap `0x65440c` before using JavaVM's `GetEnv`
table entry at offset `0x30`. The bootstrap performs integrity checks,
native section decryption and additional initialization. Naming this large
function merely a JNI-version getter would conceal its substantive work.

## Checksum predicate and actual text decryption

REA/Ghidra recovered a masked function-pointer call at RVA `0x641fd4`:
the target is the XOR of the context's first two 64-bit words. The bootstrap
initializes context `0x6cc128` to `{0x2b4, base+0x65b0a4}`; its XOR resolves
to `base+0x65b210`, a thunk to checksum implementation `0x65b0a8`.
The checksum uses two accumulators over 32-bit words, reduces modulo
`0xffffffff`, and XORs the resulting region checksums.

Recovered probe results from the unchanged captured memory:

| Probe | Input RVA | Length | Table XOR | Literal XOR | Result |
| --- | --- | --- | --- | --- | --- |
| `0x65ae4c` | `0x6522ec` | 20751 | `0xe45faecd` | `0x41e6d70d` | `0x55` |
| `0x65aeb4` | `0x6541fc` | 20750 | `0x5bc6dfbf` | `0x99df9df4` | `0x4659c` |

`tools/native_entry_checksum.py` executes only these pure original ARM64
checksum bodies in bounded Unicorn. `bootstrap-checksum-probes.json`
retains hashes, counts and a separately labeled comparison against encrypted
on-disk bytes. That comparison is not a pristine decrypted-code baseline.

`tools/native_entry_bootstrap_predicate.py` additionally executes the original
bootstrap up to `cmp w8,w0` at RVA `0x6572fc`. After 478,797 instructions,
both operands equal `0x4659c`. The first integrity predicate therefore passes
for this captured image in offline execution. This does not identify the
cause of a later Android startup failure. The fatal wrapper at `0x642bf8`
is exactly ARM64 `exit_group` syscall 94.

The next substantive recovery is original decrypt stage `0x65bb90`:

1. It derives key material through `0x65b230` using loader region
   RVA `0x65ccf0`, length `0x25478`.
2. Section metadata at RVA `0x6c4d20` describes one encrypted code region:
   RVA `0x18d0a0`, length `0x4a2a28`, plus 32 bytes of section key material.
3. `0x65c834` combines key material. `0x682160` performs the original
   in-place native cipher operation.
4. Native `mprotect` requests make the aligned region RWX and then RX.

`tools/native_entry_decrypt_companion.py` reproduced this entire stage using
the captured native ARM64 code. It returned `1`, changed 4,841,564 of the
4,860,456 encrypted bytes, and used only a virtual errno cell and modeled
`mprotect` memory permissions. No Android or host native import executed.

The resulting contiguous memory image is
`analysis/native-entry/companion-text-decrypt/companion-decrypted-memory.bin`,
SHA-256
`714b5e3979b263917a1b85c029b0a84d07c515404f1085b1c27f41bed55d4b56`.
Its report and derived manifest preserve the original capture provenance.
`libsupercell.text-decrypted.so` is an analysis ELF reconstructed from that
image with original relocation slots restored; its SHA-256 is
`91f8f1dae4e5c4148eecf1ff47cbc0ca5c994aab04eac336ed14b9f39d45c9e2`.
This analysis copy is not claimed to be a portable executable.

## Library API now readable

After the recovered decrypt stage, RVA `0x19fee0` is eight ordinary ARM64
instructions. It returns `0` and stores two function pointers into the
caller's structure:

- `base+0x19fcec`: gets a singleton through `0x5c2b6c`, requires its state
  word at offset `0xdd0` to equal `2`, then passes library metadata to
  `0x19fac4` with an ID XORed with the current PID.
- `base+0x19fd60`: resolves an obfuscated state pointer. A bounded pure
  execution on the recovered captured state returns exactly the companion's
  runtime base `0x7fda04465000`.

These are concrete original loader interfaces needed by the `libg.so`
initializer. They are not battle simulation functions. Recovering the
original `libg.so` implementation and its native UI remains a further step.

## Evidence and reproduction boundary

Focused REA records in `analysis/native-entry/rea-seeded/` include:

- Masked call: `ev_171ba4385e8547baa8e22bac938b6905ad41e77272700a369699e7ef5e7f6e64`.
- Probe A: `ev_3a1f6368e05a250071a1e8409952ddc40d6d943101279798caec4c2244aaa673`.
- Probe B: `ev_98d57164a987d00476dfdb0eb85872e67c1bcd9bd074c6e416dc1705925f06e9`.
- Checksum body: `ev_07960df126f867dd184c69584c599bd89737f0085b09db1a1636fcb25d7b377d`.
- Decrypt-stage wrapper: `ev_659c758ce7a51ccfe2f6b3cd44d8fd25d81fbf2f1481ea0fdc7b3f329c36224d`.
- Section decrypt loop: `ev_4bf2757f0c263f97902843b03620f102788cb17e2ac0dd72cdfddeef2204cde9`.

Seed names are investigation annotations, not original exported names.
Ghidra analysis addresses use image base `0x100000`; all addresses in this
document are ELF RVAs unless explicitly labeled runtime addresses.
Static decompilation, actual captured process memory, and offline execution
are distinct evidence types. None establishes a complete game launch or a
faithful Rust rewrite on its own.
