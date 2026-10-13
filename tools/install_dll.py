"""Reversibly install the TGCX DLL. The game must be closed; originals are retained."""

import argparse
import ctypes
from ctypes import wintypes
import hashlib
import json
import os
from pathlib import Path
import shutil

import check_exe
from tgcx_pe import PE

ROOT = Path(__file__).resolve().parent.parent
SOURCE = ROOT / "dll/tgcx/Release/lua51.dll"
ORIGINAL_SIZE = 11264
ORIGINAL_SHA256 = "b2ba19343691f22d1af1417f9a7cbaeae74ba820a2a412f5a6cdea271cc1a7b0"
STATE = "tgcx_dll_install.json"


def digest(path):
    return hashlib.sha256(Path(path).read_bytes()).hexdigest()


def game_running():
    if os.name != "nt":
        return False
    class Entry(ctypes.Structure):
        _fields_ = [("dwSize", wintypes.DWORD), ("cntUsage", wintypes.DWORD),
                    ("th32ProcessID", wintypes.DWORD), ("th32DefaultHeapID", ctypes.c_size_t),
                    ("th32ModuleID", wintypes.DWORD), ("cntThreads", wintypes.DWORD),
                    ("th32ParentProcessID", wintypes.DWORD), ("pcPriClassBase", wintypes.LONG),
                    ("dwFlags", wintypes.DWORD), ("szExeFile", wintypes.WCHAR * 260)]
    kernel = ctypes.WinDLL("kernel32", use_last_error=True)
    kernel.CreateToolhelp32Snapshot.argtypes = [wintypes.DWORD, wintypes.DWORD]
    kernel.CreateToolhelp32Snapshot.restype = wintypes.HANDLE
    kernel.Process32FirstW.argtypes = [wintypes.HANDLE, ctypes.POINTER(Entry)]
    kernel.Process32NextW.argtypes = [wintypes.HANDLE, ctypes.POINTER(Entry)]
    kernel.CloseHandle.argtypes = [wintypes.HANDLE]
    snapshot = kernel.CreateToolhelp32Snapshot(2, 0)
    if snapshot == ctypes.c_void_p(-1).value:
        raise OSError("cannot check whether the game is running")
    try:
        entry = Entry()
        entry.dwSize = ctypes.sizeof(entry)
        more = kernel.Process32FirstW(snapshot, ctypes.byref(entry))
        while more:
            if entry.szExeFile.lower() == "v2game.exe":
                return True
            more = kernel.Process32NextW(snapshot, ctypes.byref(entry))
        if ctypes.get_last_error() != 18:  # ERROR_NO_MORE_FILES
            raise OSError("process enumeration failed")
        return False
    finally:
        kernel.CloseHandle(snapshot)


def original(path):
    if not path.is_file() or path.stat().st_size != ORIGINAL_SIZE or digest(path) != ORIGINAL_SHA256:
        raise ValueError(f"{path}: not the verified original 11,264-byte Lua DLL; leaving it alone")


def proxy(path, reference):
    image = PE(path)
    if image.machine != 0x14C or not any(marker in image.data for marker in
                (b"TGCX DLL-0 0.1.0", b"TGCX DLL-1+HEMI-1 0.2.0", b"TGCX DLL-1+HEMI-1 0.2.1", b"TGCX DLL-1+HEMI-1 0.2.2", b"TGCX DLL-1+HEMI-1 0.2.3", b"TGCX DLL-1+HEMI-1 0.2.4", b"TGCX DLL-1+HEMI-2 0.2.5")):
        raise ValueError(f"{path}: not a recognized TGCX Win32 build")
    expected = {name: (ordinal, f"lua51_real.{name}")
                for name, (ordinal, _) in PE(reference).exports().items()}
    if len(expected) != 128 or image.exports() != expected:
        raise ValueError("Lua exports do not match the original's 128 names and ordinals")


def unique_off(folder, name):
    candidate = folder / name
    index = 1
    while candidate.exists():
        candidate = folder / f"{name}.{index}"
        index += 1
    return candidate


def write_state(path, data):
    temporary = path.with_name(path.name + ".pending")
    if temporary.exists():
        raise ValueError(f"{temporary} exists; refusing to overwrite it")
    with temporary.open("x", encoding="utf-8", newline="\n") as stream:
        json.dump(data, stream, indent=2)
        stream.write("\n")
    os.replace(temporary, path)


def operate(folder, remove=False, dry_run=False, source=SOURCE):
    folder = Path(folder).resolve(strict=True)
    source = Path(source).resolve()
    if game_running():
        raise ValueError("close v2game.exe before installing or removing the DLL")
    if digest(folder / "v2game.exe") != check_exe.SHA256:
        raise ValueError("v2game.exe is not the reference build; leaving the folder alone")
    active, real, state = folder / "lua51.dll", folder / "lua51_real.dll", folder / STATE
    if not (folder / "lua5.1.dll").is_file():
        raise ValueError("lua5.1.dll is missing (the original shim requires it)")
    if any(p.is_symlink() for p in (active, real, state)):
        raise ValueError("refusing linked DLL or state paths")
    old_state = None
    if real.exists():
        original(real)
        if not state.is_file():
            raise ValueError("lua51_real.dll already exists without our installation record; refusing to overwrite")
        old_state = json.loads(state.read_text(encoding="utf-8"))
        if old_state.get("original_sha256") != ORIGINAL_SHA256:
            raise ValueError("unexpected original hash in installation record")
        if active.exists():
            proxy(active, real)
            if digest(active) != old_state.get("proxy_sha256"):
                raise ValueError("active DLL differs from our installation record; leaving it alone")
    else:
        original(active)
        if state.exists():
            raise ValueError("installation record exists without the original backup; inspect it first")
        if remove:
            print("install_dll: original already active; nothing to remove")
            return
    if remove:
        off = unique_off(folder, "lua51_tgcx.dll.off")
        state_off = unique_off(folder, "tgcx_dll_install.json.off")
        if dry_run:
            print(f"dry-run: retain proxy as {off.name}; restore lua51_real.dll -> lua51.dll")
            return
        if active.exists():
            active.rename(off)
        try:
            real.rename(active)
        except OSError:
            if off.exists():
                off.rename(active)
            raise
        state.rename(state_off)
        print(f"install_dll: original restored; proxy retained as {off.name}")
        return
    if not source.is_file():
        raise ValueError("build the DLL first: powershell -File tools/build_dll.ps1")
    proxy(source, real if real.exists() else active)
    source_hash = digest(source)
    if old_state and active.exists() and source_hash == digest(active):
        print("install_dll: identical proxy already installed; original retained")
        return
    if dry_run:
        print("dry-run: DLL, exports, original, executable and process gates OK; no files changed")
        return
    staged = folder / "tgcx_dll_pending.dll"
    if staged.exists():
        raise ValueError(f"{staged} exists; refusing to overwrite")
    # Exclusive staging, checked before the original is renamed.
    with source.open("rb") as src, staged.open("xb") as dst:
        shutil.copyfileobj(src, dst)
    if digest(staged) != source_hash:
        raise ValueError("staged DLL hash mismatch; original untouched")
    saved = real if not old_state else unique_off(folder, "lua51_tgcx.dll.off")
    moved = active.exists()
    if moved:
        active.rename(saved)
    try:
        staged.rename(active)
        write_state(state, {"original_sha256": ORIGINAL_SHA256, "proxy_sha256": source_hash,
                            "source": str(source), "card": "DLL-1+HEMI-2" if b"TGCX DLL-1+HEMI-2" in PE(source).data
                            else "DLL-1+HEMI-1" if b"TGCX DLL-1+HEMI-1" in PE(source).data else "DLL-0"})
    except (OSError, ValueError):
        if active.exists():
            active.rename(unique_off(folder, "lua51_tgcx.dll.off"))
        if moved:
            saved.rename(active)
        raise
    print("install_dll: proxy installed; verified original retained as lua51_real.dll")


def main():
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument("game_dir", type=Path)
    ap.add_argument("--remove", action="store_true")
    ap.add_argument("--dry-run", action="store_true")
    args = ap.parse_args()
    try:
        operate(args.game_dir, args.remove, args.dry_run)
    except (OSError, ValueError, KeyError) as error:
        raise SystemExit(f"install_dll: {error}") from error


if __name__ == "__main__":
    main()
