# Native battle termination and winner

Source ELF: `analysis/libg-cipher/libg.decrypted.so`, SHA-256
`79fe6612e20922d4c4086b1e6c939464202cefa7cb3b20344236529f941423d6`.
All addresses below are raw RVAs (Ghidra address minus `0x100000`). Focused
assembly and decompilation are under
`analysis/native-focused/rl-harness-completion-v1/` (completion getter and clock
route) and `analysis/native-focused/terminal-reward-review-raw/` (finalization
routine `d3e360`, winner stores and comparisons).

For a world pointer `world`, the native battle is `battle = *(u64 *)(world +
0xa8)`. `d4220c(battle)` is a one-byte getter: its complete original body is
`ldrb w0, [x0, #0x1e0]; ret`. Thus `battle+0x1e0` is the native completion
flag. The harness checks this flag after each normal world update before
constructing the ordinary terminal event; the event code is not a winner.

The exact native result is a signed 32-bit field at `battle+0x1b8`. The
completion/finalization routine `d3e360(battle, participant)` first returns if
`battle+0x1e0` is already set; otherwise it sets that byte to 1 and writes the
result field. Its no-participant branch compares the two native side records
at `battle+0xe0` and `battle+0xe8` (with a mode-specific score path where
configured). The original stores establish the mapping:

- `0`: side 0 wins.
- `1`: side 1 wins.
- `-1`: equal comparison, draw.

The non-resignation comparison stores 0 when side 0's score is greater, 1
when side 1's score is greater, and -1 on equality. The participant/resign
branch stores 0 or 1 after comparing the participant with the native side-0
participant. The same routine later reads `battle+0x1b8` to select native
players, confirming it is the finalized side index. Read this signed field
after `battle+0x1e0` becomes true; do not interpret unsigned `0xffffffff` as a
winner.

`battle+0x190` is a separate clock/time record used by update and finalization
helpers; it is not the winner field. The cached values at `battle+0x120` and
`battle+0x124` are updated by `d345a0` from native entity tests, but the saved
full-match terminal state has both equal to 3 while `battle+0x1b8` is 0.
Those counts therefore are not a valid winner reward basis by themselves.

In the saved native full-match run
`analysis/native-oracle/full-match-01/`, the terminal snapshot has
`battle+0x1e0 = 1`, signed `battle+0x1b8 = 0`, and counts `+0x120/+0x124 =
3/3`. This directly verifies that run's native outcome as side 0.
