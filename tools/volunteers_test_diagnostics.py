"""Prepare a reversible diagnostics-only change for the isolated volunteer copy.

No game process is started/stopped. The caller owns the backup and installation.
Upstream DLLs, simulation options and TGC data are never modified by this helper.
"""
import re

KEYS = (b'ENABLE_STATE_PROBE', b'ENABLE_RNG_PROBE')


def light_settings(source: bytes) -> bytes:
    result = source
    for key in KEYS:
        pattern = rb'(?m)^(' + key + rb'[ \t]*=[ \t]*)([01])([ \t]*\r?$)'
        result, count = re.subn(pattern, lambda m: m[1] + b'0' + m[3], result)
        if count != 1:
            raise ValueError('expected exactly one ' + key.decode('ascii'))
    return result
