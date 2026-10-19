Second beta of **ATCU - Army Template and Conform Universal** for Victoria II 3.04 (single player).

New in 0.0.2:

- **Culture filter:** each template recruits from Primary + accepted cultures, Primary only, or any culture. Build Army and Conform both use it.
- **Continent filter:** Build Army can recruit from one continent where you own provinces, or from any. It applies to the current recruitment only.
- **Conform survives a load:** regiments being built for a Conform are listed in a local file. After you load a save of the same country, they still march to and join their army instead of stopping at the rally point.
- Native checkboxes and a compact continent selector in the Templates and Conform windows.

Short checks passed with 0.0.2 on TGC 2.5.2, both with V2DLL 5.82 and with the included loader. 0.0.1 passed vanilla, HPM 0.4.5 and Divergences of Darkness Fan Fork; the new filters were not rechecked there. Other mods and multiplayer are untested. Only the 3.04 `v2game.exe` is supported, and ATCU turns itself off on any other build.

**Install or update:** download `ATCU-0.0.2.zip`, extract it and follow `README.txt`. You copy files only: no installer and no scripts. Updating from 0.0.1 replaces the mod folder; your templates are imported.

Known limits: Conform never ships troops overseas, and a Conform order is resumed after a load only if its army still exists with the same id.
