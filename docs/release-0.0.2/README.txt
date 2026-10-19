ATCU 0.0.2 - Army Template and Conform Universal
Victoria II 3.04 (Windows), single player, beta

Save army compositions, recruit them in one click, and press Conform on
an army in the field to make it match a template: extras are detached
into a new stack, missing regiments are built and march to join it.
Works on vanilla and on base mods: the unit list is read from the game.

New in 0.0.2: culture and continent filters for recruitment, and Conform
orders that survive loading a save.

Tested (short checks): 0.0.2 on TGC 2.5.2, with V2DLL 5.82 and with the
included loader. 0.0.1 also passed vanilla, HPM 0.4.5 and Divergences of
Darkness Fan Fork; the 0.0.2 filters were not rechecked there. Other
mods and multiplayer are untested. Only the 3.04 v2game.exe with SHA-256
62d48c204364dd706584777c2e2b3c7ab3c5f1dd0170872554943575d53d6648
is supported; on any other build ATCU turns itself off and says so in
Logs/tgcx.log.


INSTALL

Exit the game first. Back up any file you replace.

1. Copy this ZIP's "mod" folder into your game folder (the one with
   v2game.exe). You should end up with:
     <game>/mod/tgcx_army_universal.mod
     <game>/mod/tgcx_army_universal/tgcx.dll

2. Pick ONE loader:

   a) No V2DLL: rename the game's original lua51.dll to lua51_real.dll,
      then copy this ZIP's lua51.dll next to v2game.exe. (Turn on file
      extensions in Explorer first.) If lua51_real.dll already exists,
      never overwrite it: you likely have our loader already, so skip
      this step.

   b) V2DLL already installed: do NOT copy lua51.dll. In the game
      folder's v2dll_settings.ini set LOAD_DLL_FROM_MOD_FOLDER=1
      (tested with LOCAL_MOD_CONFIG=0).
      V2DLL only loads the plugin of the first ticked mod, so ATCU must
      sort first in the launcher (it does before DoD, HPM and TGC).

3. In the launcher tick "ATCU - Army Template and Conform Universal"
   plus your base mod (or ATCU alone for vanilla). Untick any older
   "Army Templates and Conform" entry.

Updating from 0.0.1: replace mod/tgcx_army_universal and its .mod file.
Keep your loader. Your templates are imported (see below).

Removing: untick ATCU and delete mod/tgcx_army_universal and its .mod.
Our lua51.dll can stay. To remove it too, delete it and rename
lua51_real.dll back to lua51.dll. Never touch V2DLL's own files.


HOW TO USE

Templates: Military > Build Army > the small Templates button near the
unit folders (on DoD it sits in the title area). New Template, set the
counts, Recruit. Click = +/-1, Alt-click = 2, Ctrl-click = 5.

Conform: select an army, click its Conform button, choose a template,
check Changes, click Conform. Unpause so the orders can run.

Culture: each template keeps one choice, "Primary + accepted" or
"Primary only". Click the ticked box again to clear it and allow any
culture. Build Army and Conform both use it for new regiments; existing
regiments are not removed for their culture.

Continent: the arrows in Build Army choose Any or a continent where you
own provinces. This is for the current recruitment only and is not
saved. Conform ignores it and keeps its own land-route rule.

Files: in Documents/Paradox Interactive/Victoria II (in the mod's
settings folder if it has one). The exact path is in Logs/tgcx.log.
- tgcx_templates_v2.txt: your templates. On first run it is created
  from 0.0.1's tgcx_templates.txt, which is left unchanged.
- tgcx_conform_orders.txt: regiments being built for a Conform (country,
  date, stack and construction ids). Loading a save of the same country
  dated within an order resumes it: the regiment still marches to and
  joins its army, even from a rally point. Deleting this file only stops
  that.
Neither file is stored in your save.


KNOWN LIMITS

- A Conform order is resumed only if its army still exists with the
  same id after the load. An army merged or split away before the save
  is skipped, and the regiment finishes as its own stack.
- Conform only recruits where there is a land route to the army. It
  does not ship troops overseas; shortages are reported.
- With a filter on, fewer provinces qualify, so more regiments may be
  left out; the shortage is reported.
- Templates use the mod's unit names: a library from one mod may not
  load in a mod with different units.
- The log says "TGCX TPL-7 0.11.2": that is the internal build of this
  release and is expected.


PROBLEMS

Exit the game and save Logs/tgcx.log (and Logs/v2dll.log if you use
V2DLL) before starting it again. Report at
https://github.com/azha2005/victoria2tgcx/issues with: the ATCU version,
your base mod and every ticked mod, which loader, what you clicked,
what happened, a screenshot, and the log. Logs can contain your Windows
user name in paths.


CREDITS

azha2005 / JC: the mod. artyom-kuznetsov and V2DLL contributors: Lua
forwarding, hook patterns and the plugin API
(https://github.com/artyom-kuznetsov/V2DLL_Victoria-2-Extension).
maxioten and the Victoria2-Reverse-Engineering contributors: notes and
symbols. Paradox Interactive: Victoria II and the interface art some
ATCU textures are drawn from. TGC, HPM and DoD Fan Fork teams: the base
mods it was tested with. No endorsement is implied. No license has been
chosen for ATCU yet.
