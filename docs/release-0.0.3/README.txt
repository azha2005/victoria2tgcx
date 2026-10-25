ATCU 0.0.3 - Army Template and Conform Universal
Victoria II 3.04 (Windows), single player, beta

Save army compositions, recruit them in one click, and press Conform on
an army in the field to make it match a template: extras are detached
into a new stack, missing regiments are built and march to join it.
Works on vanilla and on base mods: the unit list is read from the game.

New in 0.0.3:
- Replace depleted: one click builds a fresh regiment for each worn-out
  one; the old regiment leaves the army only after the new one joins.
- Build Army gathering: one Recruit click makes one army, gathered at
  the best supply province, the capital or a rally point.
- Non-core only culture choice, and Mobo stacks for mobilization.
See CHANGELOG.txt for the full list.

Tested (short checks): 0.0.3 on TGC 2.5.2 with the included loader.
0.0.2 also passed with V2DLL 5.82, and 0.0.1 passed vanilla, HPM 0.4.5
and Divergences of Darkness Fan Fork; the 0.0.3 features were not
rechecked there. Other mods and multiplayer are untested. Only the 3.04
v2game.exe with SHA-256
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

Updating from 0.0.1 or 0.0.2: replace mod/tgcx_army_universal and its
.mod file. Keep your loader (the included lua51.dll is unchanged since
0.0.1). Your templates are imported (see below).

Removing: untick ATCU and delete mod/tgcx_army_universal and its .mod.
Our lua51.dll can stay. To remove it too, delete it and rename
lua51_real.dll back to lua51.dll. Never touch V2DLL's own files.


HOW TO USE

Templates: Military > Build Army > the small Templates button near the
unit folders (on DoD it sits in the title area). New Template, set the
counts, Recruit. Click = +/-1, Alt-click = 2, Ctrl-click = 5.

Gather at: one Recruit click makes one army. Each regiment is built in
its own province, then marches by a safe land route and merges. The
arrows choose Best supply (your province with the highest supply limit
that the regiments reach by land), Capital, or each of your land rally
points. If an army of yours already stands there, the new army gathers
in the free province beside it with the highest supply. Pops with no
land route to the target are left out and reported. You need at least
one army already, to plan the routes. The choice is not saved.

Conform: select an army and click the green button in its header,
choose a template, check Changes, click Conform. Unpause so the orders
can run.

Replace depleted: the maroon button beside Conform. Tick Low strength
(set the level with the slider, 10% to 90%) and/or Pop too small (the
pop the unit panel shows red or yellow), then Replace. One regiment of
the same type is built for each depleted one, within land reach, and
marches to the army. When it has joined, the old regiment is detached
into its own stack beside the army. Nothing is disbanded: use the
game's own Disband button for that.

Culture: each template, and the Replace window, keeps one choice:
"Primary + accepted", "Primary only" or "Non-core only" (a soldier pop
whose culture is neither primary nor accepted). Click the ticked box
again to clear it and allow any culture. Existing regiments are not
removed for their culture.

Continent: the arrows in Build Army choose Any or a continent where you
own provinces. This is for the current recruitment only and is not
saved. Conform and Replace ignore it and keep their land-route rule.

Mobo stack: the box in the Conform window header marks the selected
army to be completed with infantry when you mobilize. Each new
all-infantry stack then marches to the nearest marked army still short
of infantry and joins it, if all its regiments fit.

Files: in Documents/Paradox Interactive/Victoria II (in the mod's
settings folder if it has one). The exact path is in Logs/tgcx.log.
- tgcx_templates_v3.txt: your templates. On first run it is created
  from 0.0.2's tgcx_templates_v2.txt (or 0.0.1's tgcx_templates.txt),
  which is left unchanged.
- tgcx_conform_orders.txt, tgcx_replace_orders.txt,
  tgcx_gather_orders.txt and tgcx_mobo_marks.txt: regiments on their
  way and marked armies. Loading a save of the same country resumes
  them, from the in-game menu or the main menu. Deleting a file only
  stops that.
- tgcx_replace_settings.txt: the Replace window's choices.
None of these files is stored in your save.


KNOWN LIMITS

- An order is resumed only if its army still exists with the same id
  after the load. An army merged or split away before the save is
  skipped, and the regiment finishes as its own stack.
- A reload to the same date, or less than about 45 days ahead while
  playing, is not seen as a load. Loading through the menus as usual
  works.
- Conform, Replace and Build Army only recruit where there is a land
  route. They do not ship troops overseas; shortages are reported.
- With a filter on, fewer provinces qualify, so more regiments may be
  left out; the shortage is reported.
- Templates use the mod's unit names: a library from one mod may not
  load in a mod with different units.
- Going back to 0.0.2 keeps your files, but template edits made in
  0.0.3 are in tgcx_templates_v3.txt and do not appear there.
- The log says "TGCX TPL-12 0.11.16": that is the internal build of
  this release and is expected.


PROBLEMS

Exit the game and save Logs/tgcx.log (and Logs/v2dll.log if you use
V2DLL) before starting it again. Report at
https://github.com/azha2005/victoria2tgcx/issues with: the ATCU version,
your base mod and every ticked mod, which loader, what you clicked,
what happened, a screenshot, and the log. Logs can contain your Windows
user name in paths.


CREDITS

azha2005 / JC: the mod. fedacking: the Rust port of the DLL and loader,
and Mobo stacks. artyom-kuznetsov and V2DLL contributors: Lua
forwarding, hook patterns and the plugin API
(https://github.com/artyom-kuznetsov/V2DLL_Victoria-2-Extension).
maxioten and the Victoria2-Reverse-Engineering contributors: notes and
symbols. Paradox Interactive: Victoria II and the interface art some
ATCU textures are drawn from. TGC, HPM and DoD Fan Fork teams: the base
mods it was tested with. No endorsement is implied. No license has been
chosen for ATCU yet.
