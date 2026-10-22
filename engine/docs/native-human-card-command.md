# Native human card command

Source findings and an executed external card-input probe for the original ARM64 ELF `analysis/libg-cipher/libg.decrypted.so` (SHA-256 `79fe6612e20922d4c4086b1e6c939464202cefa7cb3b20344236529f941423d6`). Addresses below are raw RVAs, without Ghidra's `+0x100000` image base. Complete native AI-match evidence is recorded separately in [native-battle-loop.md](native-battle-loop.md).

## Command object and human callsite

The native play-card command is allocated as `0x58` bytes. Its constructor is `d774ac(X0=command, X1=actor/session-context)`. It calls the base command constructor `d72718(command, context)`, installs vtable pointer RVA `0x19103d0`, initializes the target coordinates at `+0x28/+0x2c` to zero, initializes `+0x30` and `+0x50` to `-1`, and fills the card-reference area from the native default-reference factory. `d77c8c(X0=command, X1=selection-reference)` copies **20 bytes** from the reserved 24-byte descriptor slot into `+0x38..+0x4b`: the assembly stores one 16-byte vector and one 32-bit word. The last four bytes of the reserved slot are not copied or established. `d77a14(X0=command, W1=x, W2=y)` writes the two target coordinates at `+0x28/+0x2c`.

The human play path is in FDE `be1634` (assembly `analysis/native-focused/human-command-abi/rva_be1634.asm`). It obtains a selected-card reference by calling the selected object's virtual `+0x28`, then constructs the command and passes that reference to `d77c8c`. It passes the placement pair computed in its native validation path to `d77a14`; no card identifier or target position is a constructor argument. It then submits a pointer-to-command through `cc4658(controller, &command_pointer, 1)`. The call sequence is visible at Ghidra addresses `0x00ce19a4..0x00ce19d0`, corresponding to raw RVAs `be19a4..be19d0` (`be1634+0x370..+0x39c`).

The selected reference is a native 24-byte descriptor slot, not an integer card ID. In the harness bot path, `e80bbc(X0=selected native card wrapper, X1=side card/deck object, W2=1, X8=&descriptor)` asks the wrapper's virtual `+0x110` to produce it; with a null wrapper it copies the native default descriptor. `e80680(X0=&descriptor)` tail-dispatches the descriptor's first pointer through virtual `+0x1f0` and returns a native selection proxy used by `d42214` to validate candidate placement. The command setter receives the descriptor address itself: bot assembly passes `X1=&descriptor` to `d77c8c`; the human path passes the descriptor returned by the selected object's virtual `+0x28`. `e80628(descriptor)` decodes the hand/deck position as `((u32 at descriptor+0x10 >> 22) & 0x3f) - 1`; `e80654` returns the pointer stored at descriptor offset `+0`. The original bot descriptor construction is at `11d65a4..11d65bc` and target validation at `11d6604..11d6618` in `rva_11d6398.asm`. The precise per-card row identity remains native and data-dependent.

## Callable harness route

For side index `i`, the bot harness uses `battle = *(world+0xa8)`, obtains the native side card/deck object with `d35d5c(battle, i)`, and obtains its actor/side reference with `d3c980(battle, i)`. It chooses an actual card wrapper from that native side object at a valid slot `s` (the bot uses the card-list lookup call at Ghidra `0x0109dd98`, raw RVA `0xf9dd98`, with `X0=side-card-list`, `W1=s`). Then:

```text
e80bbc(card_wrapper, side_card_list, 1, X8=&descriptor[24])
selection_proxy = e80680(&descriptor)
d42214(battle, x, y, selection_proxy, 0)  // native placement validity check
command = operator_new(0x58)
d774ac(command, harness[0])
d77c8c(command, &descriptor)
d77a14(command, x, y)
11d680c(harness, &command, actor_ref)
```

The first `e80bbc` input must be the real native card wrapper selected from the loaded battle side; a fabricated row/reference is not supported by this evidence. The harness obtains this source through its live battle card list and index, not through `LogicTestingData` alone. For the real card source and active hand position, use the same side index and card slot that the harness selected.

`11d680c` supplies the current world tick from `10ca998(world)` to the command's virtual `+0x40` setter, attaches `actor_ref` through `d72868(command, actor_ref)`, validates via virtual `+0x18(world, 1, 0)`, and enqueues through `d75a44(world+0x38, &command)`. This is the native world command manager route consumed by `d75ed0`. The human input path uses `cc4658`, which derives its tick from its controller's world, sets command virtual `+0x40`, and pushes into that world's `+0x38` queue through the same `d75a44` helper.

`d75a44` appends the same raw command pointer to the queue; it does not clone or visibly retain it. On the accepted path `d75ed0` later calls the command's destructor and release virtuals (`+0x0` and `+0x10`). Rejected commands are destroyed/released by their submit helper. The callable harness sequence should therefore keep the allocated command alive until submission succeeds, then treat queue insertion as ownership transfer; do not free it immediately after a successful enqueue.

No explicit side integer is set by the play-card constructor or its coordinate/reference setters. In the harness route, side is supplied by the native side card list and `actor_ref`; in the human route, it comes through the session/controller context. A serialized Rust command should not invent a separate side field until the native actor mapping is resolved. The exact encoding/unit conversion of human placement coordinates is also unresolved; only their final two 32-bit values and native validation path are established here.

## Executed external input

The [source-bound external-input evidence](../analysis/native-context/manual-card-evidence.json)
records a successful native card play without invoking the bot decision helper.
Starting from the prepared state, 100 original world updates bring the paired
counters to 102. Original live hand getters identify side 0, hand slot 0 as
`Mortar` (global ID 27000002), costing 4 elixir; available elixir is 7. The native
placement validator accepts coordinates `(4500,4500)` with result 0.

`manual-card-probe-01`, `manual-card-submit-01`, `manual-card-execute-01` and
`manual-card-followthrough-01` under `analysis/native-pipeline/` retain the
immutable plans, reports and states. The original constructor/setters and
`11d680c` accept the externally chosen command, increasing queue count from 0
to 1. The first world update reduces elixir from 7 to 3. Deployment is delayed:
after 10 further updates the command remains queued, and after 20 it has left
the queue and the entity count has risen from 6 to 7. At 100 further updates the
hand slot has cycled; the tick callbacks for all seven entities and 21 nonnull
components pass the original code provenance audit. No invalid memory access
occurred in these probes.

The call-plan adapter writes one native command pointer into an allocated
8-byte ABI input buffer. It does not fabricate a row, vtable or command layout;
original constructors and setters populate the command. Source row identity
and name are decoded using layouts established by exact getters `e0b624` (u32
at `+0x40`) and `e0b6a0` (native string at `+0x28`).

The [Rust oracle](../native-engine/README.md) now exposes this path with
`--play-card SIDE SLOT X Y` and `--after-play-steps N`. A separate exact-call
reference invokes the same getters, allocations, command setters, submission
and 100 follow-through updates through Python. The [comparison](../analysis/native-oracle/manual-card-comparison.json)
matches every saved memory region, all 70 architectural registers and complete
allocator metadata. Its latest Rust audit is
`analysis/native-oracle/manual-card-02/`. Insufficient-elixir and invalid-target
tests report rejection with no command allocation and an unchanged queue;
another submission probe accepts side 1. These tests establish the recovered
native input path, not a finished desktop UI or a Rust reimplementation of
card combat.

## Evidence files

Focused exports are under `analysis/native-focused/human-command-abi/`: `rva_be1634.{asm,c}`, `rva_cc4658.{asm,c}`, `rva_d774ac.{asm,c}`, `rva_d77504.{asm,c}`, `rva_d77a14.{asm,c}`, `rva_d77c8c.{asm,c}`, `rva_e80bbc.{asm,c}`, `rva_e80628.{asm,c}`, `rva_e80654.{asm,c}`, `rva_e80680.{asm,c}`, `rva_d75a44.{asm,c}`, and `rva_d75ed0.{asm,c}`. Harness callsites and side/deck inputs are in `analysis/native-focused/battle-tick/rva_11d6398.asm` and `.c`. `d77504` is the native play-command validation/execution method; its use of the card-reference index and world battle state confirms the descriptor is consumed as a native card selection.
