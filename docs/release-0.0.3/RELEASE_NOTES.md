Third beta of **ATCU - Army Template and Conform Universal** for Victoria II 3.04 (single player).

New in 0.0.3:

- **Replace depleted:** a button beside Conform builds a fresh regiment for each worn-out one (low strength, or a pop too small). The new regiment marches to the army and joins it, then the old one is detached. Nothing is disbanded.
- **Build Army gathering:** one Recruit click makes one army, gathered at the best supply province, the capital or a land rally point.
- **Non-core only** culture choice, and **Mobo stacks** that fill a marked army with infantry on mobilization.
- Orders now resume after loading a save from the main menu, and the Disband depleted button is fully clickable.

The DLL is now written in Rust (thanks to fedacking). The loader is unchanged since 0.0.1.

Short checks passed with 0.0.3 on TGC 2.5.2 with the included loader. 0.0.2 passed with V2DLL 5.82, and 0.0.1 passed vanilla, HPM 0.4.5 and Divergences of Darkness Fan Fork; the new features were not rechecked there. Other mods and multiplayer are untested. Only the 3.04 `v2game.exe` is supported, and ATCU turns itself off on any other build.

**Install or update:** download `ATCU-0.0.3.zip`, extract it and follow `README.txt`. You copy files only: no installer and no scripts. Updating replaces the mod folder; your templates are imported. `CHANGELOG.txt` lists every change.

Known limits: troops are never shipped overseas, and an order is resumed after a load only if its army still exists with the same id.
