"""Optional source-verified episode recovery for compiled coverage gaps."""
from __future__ import annotations
from pathlib import Path
from typing import Any
try:
    from .banana_train import JsonlEngine
except ImportError:
    from banana_train import JsonlEngine


def semantic(response: dict[str, Any]) -> dict[str, Any]:
    return {key: value for key, value in response.items() if key not in ('reset_seconds', 'step_seconds')}


class ReplayFallbackEngine:
    """Replay the exact episode in the reference only after a coverage error.

    The failed compiled process is discarded. Every successful prior response
    must agree with the reference replay before the failed request can proceed.
    The reference finishes that episode; the next reset tries compiled code
    again. No native state, action, tick or reward is synthesized.
    """
    def __init__(self, checkpoint: Path, compiled: Path, reference: Path):
        self.checkpoint, self.compiled, self.reference = checkpoint, compiled, reference
        self.client = JsonlEngine(checkpoint, compiled)
        self.history: list[tuple[dict[str, Any], dict[str, Any]]] = []
        self.in_reference = False
        self.fallback_count = 0
        self.fallback_events: list[dict[str, Any]] = []
        self.poisoned = False

    def request(self, request: dict[str, Any]) -> dict[str, Any]:
        if request.get('op') == 'reset':
            self.poisoned = False
            self.history.clear()
            if self.in_reference:
                self.client.close()
                self.client = JsonlEngine(self.checkpoint, self.compiled)
                self.in_reference = False
        if self.poisoned:
            raise RuntimeError('reference recovery failed; reset required')
        try:
            response = self.client.request(request)
        except RuntimeError as exc:
            reason = str(exc)
            coverage_error = any(word in reason for word in (
                'untranslated function', 'untranslated block', 'unsupported ',
                'unmodeled ', 'requires architectural handling', 'FP domain',
            ))
            if self.in_reference or not coverage_error:
                raise
            self.client.close()
            self.client = JsonlEngine(self.checkpoint, self.reference)
            self.in_reference = True
            try:
                for index, (prior, expected) in enumerate(self.history):
                    actual = self.client.request(prior)
                    if semantic(actual) != expected:
                        raise RuntimeError(f'reference recovery disagrees with compiled response {index}; refusing to continue') from exc
                response = self.client.request(request)
            except BaseException:
                self.poisoned = True
                self.client.close()
                raise
            self.fallback_count += 1
            self.fallback_events.append({'reason': reason, 'replayed_requests': len(self.history)})
        if request.get('op') not in ('close', 'audit'):
            self.history.append((dict(request), semantic(response)))
        return response

    def close(self) -> None:
        self.client.close()
