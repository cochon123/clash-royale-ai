"""Refuse to invent tower HP from the official client.

The native game lives in the AArch64 library ``libg.so``. Its only
battle-adjacent export is the Titan JNI bootstrap
``Java_com_supercell_titan_GameApp_createGameMain``. Ghidra 12.1.4
decompiles that symbol as ``halt_baddata``: the bytes at the export are
not a callable function. The JNI descriptor takes an Android
``AssetManager`` and ``Activity``, not two decks and an action list.
``libg.so`` also imports ``socket``, ``connect``, and ``getaddrinfo``.
This module does not load the library, does not emulate it, and does not
open a socket.
"""

from __future__ import annotations

from collections.abc import Mapping, Sequence

CREATE_GAME_MAIN = "Java_com_supercell_titan_GameApp_createGameMain"
CREATE_GAME_MAIN_DESCRIPTOR = (
    "(Landroid/content/res/AssetManager;Ljava/lang/String;Ljava/lang/String;"
    "Ljava/lang/String;JIIIFFILjava/lang/String;Landroid/app/Activity;)"
    "Ljava/lang/String;"
)

NATIVE_REPLAY_BLOCKER = (
    "No tower HP was read from libg.so. REA search-android-classes on "
    "base.apk found no battle, tower, crown, elixir, or logic class "
    "(15874 classes; the only Replay class is io.sentry.android.replay). "
    "REA execute-rizin-command iE on libg.so lists 86 dynamic symbols, all "
    "Titan JNI. Java_com_supercell_titan_GameApp_createGameMain is 1892 bytes "
    "at 0x1460bd4 and its descriptor takes AssetManager plus Activity, not "
    "decks and actions. libsupercell_clashroyale.so exports only JNI_OnLoad. "
    "A full-file string scan of both libraries has no plaintext replay, "
    "battle, tower, or crown. The install-time asset pack has csv_logic "
    "tables and no path containing replay. rea inspect-binary-layout rejects "
    "this AArch64 ELF (x86-64 only). Ghidra 12.1.4 via rea function "
    "--provider ghidra (exit 0, default auto-analysis finished) decompiled "
    "createGameMain at image address 0x1560bd4 as halt_baddata: a 1-byte "
    "body, no callees, and no strings. The rizin symbol is still 1892 bytes "
    "at 0x1460bd4; those bytes are not valid AArch64. The same provider "
    "result on libg JNI_OnLoad (0x156096c, 1 byte), nOnAppStart (0x1561b10, "
    "4 bytes), and nOnTouchEvent (0x15616d4, 4 bytes) is halt_baddata or a "
    "truncated bad instruction, with no callees and no strings. "
    "libsupercell_clashroyale.so JNI_OnLoad at 0x78179c decompiles to an "
    "obfuscated constant-driven state machine with no callees and no strings. "
    "None of those functions takes decks and actions or returns tower HP. "
    "DT_INIT at 0x1ae3c20 is valid startup code and an obfuscated dispatcher; "
    "it was not executed. libg.so imports socket, connect, and getaddrinfo, "
    "so loading it would boot the online client. That call is not made and "
    "those imports are not stubbed. GameApp.beforeLogicCallback updates "
    "PurchaseManager and NativeHTTPClientManager; it is not a combat step. "
    "Smallest honest next step: a process on http://127.0.0.1:8790 that "
    "speaks cr-local-match/1 and resolves combat itself. Do not patch "
    "libg.so and do not stub its sockets."
)


class ReplayEngineUnavailable(RuntimeError):
    """Raised when a caller asks libg.so for tower HP."""


def replay_towers(
    *,
    red_deck: Sequence[str],
    blue_deck: Sequence[str],
    actions: Sequence[Mapping],
) -> dict:
    """Return tower HP and crowns for one replay.

    The signature is the one a local engine would have: two decks and the
    local-match action records (card id, tile, time). The official client
    does not implement it. This function never returns hit points.
    """
    if len(red_deck) != 8 or len(blue_deck) != 8:
        raise ValueError("each deck must contain 8 cards")
    for action in actions:
        for key in ("card_id", "seconds"):
            if key not in action:
                raise ValueError(f"action is missing {key}")
    raise ReplayEngineUnavailable(NATIVE_REPLAY_BLOCKER)
