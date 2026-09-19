# arcade — Twilight Zone, the fix runbook (J101 · J103 · J21)

*The machine in the operator's room went RED: stuck in standby, reset
detected, with J101 and J103 + J21 named as the suspects. This runbook is
grounded in the machine's own manual (the arcademanual scan this vault
holds) and in the WPC power-rail failure family. Read this as the
discipline: voltage first, connectors second, boards third. The RED state
is the machine's own 1201/1202 — it refuses to continue rather than
corrupt; our job is to keep the 5V rail alive and witness the recovery.*

## the manual's own evidence

- the J21 header on the driver board carries the CPU rail: **J21-4 and
  J21-5 = +5VDC** (from J114-4 and J114-3), **J21-6 and J21-7 =
  +12VDC**, J21-1 = ground. These four power pins are the classic WPC
  brown-out path: charred pins, melted housings, sag under load.
- the manual's memory boundary: *"voltage drops below +4 V, memory reset
  occurs. Check the batteries and battery holder"* — the CPU's own
  threshold is 4V; the rail must hold 5.0–5.25V under load, and the
  battery holder is a second, independent suspect.
- connector notation: J101-3 = pin 3 of jack 1 on its board — the
  absolute addressing the diagnostics speak in.

## FIX — the protocol (in order, stop when the cause is found)
1. **Power off, unplug, inspect.** Pull J101, J103, J21. Look for the
   browned/melted pin housings, heat-stained pins, and the tell-tale
   IDC widow's peak. If any pin is charred, stop reseating — repin.
2. **Clean + repin.** Scrape/contact-clean the pins, replace charred
   pins with quality trifurcon replacements, replace melted housings.
   Heat-shrink the 5V/12V runs if the original wire insulation is
   stiffening.
3. **Reseat firmly, power on, measure.** +5V at **J21-4/5 vs J21-1**
   (ground): must sit 5.0–5.25V at idle. Then the load test:
   cycle both flippers and pull a coil bank — the rail must not dip
   below the low-4s; a dip under load is the rail (caps/regulator), not
   the connector.
4. **Battery/memory.** With the diagnostic power address in hand, check
   the CPU battery holder: below 4V the manual promises memory reset. If
   the holder shows corrosion, clean the trace (vinegar+rinse+dry) and
   fit fresh batteries; the Clock audits — the NOT-resettable record —
   depend on it and must survive this fix.
5. **The RED recheck.** Clear the reset warning via the operator menu,
   boot to attract, let it sit five minutes in standby without resetting.
   Then run the manual's own tests: **T.1 Clock test**, and the gumball
   test (T.15) for the Geneva optos.
6. **Still resetting?** Then it is not the connectors: check the CPU
   board reset circuitry and the power supply filter caps on the
   rectifier board — the classic order is: connector kit first, caps
   second, regulator third. Never skip steps 1-5 before opening board
   components.

## the RED "reset detected" message — what it is and the ladder

The banner is the CPU's own witness statement: it recorded an
UNEXPECTED reset — the processor lost +5V for a moment, or its reset
line wiggled, or a coil slam drowned the board. It is not a fault code
for one part; it is the input to a decision tree. Work the ladder only
as deep as the symptom demands.

**0. Clear it first.** Operator menu → clear the reset message. Then
classify the event: one event at power-up, game runs perfect → often a
soft switch-on dip; recurring during play → the rail or a coil.

**1. The +5V under load.** At the CPU (J21-4/5 vs J21-1): 5.0–5.25V
idle, and never into the low 4s while both flippers cycle and a coil
bank fires. Any dip → the debrowning ladder first (J101/J103/J21/J114),
then the filter caps, then the regulator. Never skip to the CPU.

**2. The resets and the ribbons.** Reseat the CPU↔driver ribbon, the
ROMs, the display data paths. A half-seated ribbon reads as the exact
same phantom reset.

**3. Rails perfect, resets anyway.** The CPU's own reset circuitry: the
WPC reset filter (cap/resistor on the reset line), then the battery
holder corrosion (corrosion on the board grounds = phantom resets —
the battery-clean is the vinegar+rinse+dry ladder), then the CPU
crystal.

**4. Resets ONLY during a specific feature.** That is not the rail —
that is the coil: a stuck-on coil, an arcing flipper EOS switch, a
short on the coil driver transistor. The machine resets exactly when
that feature energizes: inspect that coil's bank, not the power supply.

**The recheck discipline:** clear → five minutes in standby → then play
the exact move that used to trip it, five times. If the RED stays in
the record, the cause is gone; if it returns, the ladder's next rung is
the cause.

## the debrowning procedure, hands-on

The browned connector is heat-history made visible: the +5V line carried
too much for too long, the pin oxidized, the contact grew a higher
resistance, the rail sagged, and the CPU reset. The cure is grade-
graded — try the gentlest that matches the damage.

**Tools:** contact cleaner (DeOxit D5, or CRC 2-26) · isopropyl 99% ·
jeweler's or a proper pin extractor · trifurcon replacement pins +
crimp tool (or loose 0.045 inch pins for soldering) · fine emery (1000-
grit, pin barrel only) · magnifier · multimeter.

**Grades:**

1. **LIGHT — patina only, no char.** Spray the contact cleaner on the
   pins, mate/unmate the connector 3–4 times (that mechanical cycling
   is the real cleaner: it wipes the oxide), let it flash off, reseat
   firmly. This fixes most of the fleet's tired rails.
2. **MODERATE — pitting but the housing is sound.** Extract the affected
   pins (slide the extractor in from the wire side, depress the tang),
   and replace with fresh trifurcon pins freshly crimped to re-stripped
   wire. If you do not crimp: strip, tin, and solder a loose 0.045 inch
   trifurcon pin — the classic WPC field repair.
3. **SEVERE — charred pin, melted housing.** Do not reuse the housing.
   De-pin the harness, replace the housing (the manual's own parts list:
   the 4-pin and 5-pin STR Sq headers), re-pin with new trifurcons, and
   heat-shrink the 5V/12V runs so the next ten years do not repeat.

**The rules of the bench:** never sand the housing (only the pin barrel,
1000-grit max); never twist a pin in place (that wrecks the receptacle);
never trust a "reseat that holds for a minute" — the load test is the
truce: +5V at J21-4/5 vs J21-1, held above the low-4s while both
flippers cycle, for five minutes of standby after. The brown comes back
within a week if the cause was the rail; it stays gone if the cause was
the contact.

## REVIEW — how to know it is fixed, not quieted

- Reseat that holds for a week = the connector was the cause. Reseat
  that holds for a minute = the rail is still weak; the RED will return
  with the first coil-heavy moment.
- Verify the Clock's earnings audits survived (they are the machine's
  ledger — if the battery was dead, they reset once, honestly, and the
  record restarts from the fix timestamp).
- The review verdict is the same one the corridor teaches: the fix is
  not the symptom that stops; it is the cause that is gone.

## POLISH — after green

- Trifurcon the three harnesses, label them per the manual's absolute
  notation (J101-3 style), zip-tie the run away from heat.
- Clean the playfield's Power Ball orbit and the gumball Geneva; check
  the two flipper EOS switches while the apron is up.
- Row the fix in the ledger: the machine joins the constellation as a
  working instrument — the box's acoustic cousin, the same doctrine in
  steel and wood: capacity is not capability, the +5V rail is the
  constitutive invariant, and the machine is what it refuses to abandon.

## the honest note

I cannot hold the multimeter; this is the runbook, the hands are yours.
The constellation will witness the recovery the way it witnesses every
recovery: the RED row, the fix row, and the five-minutes-in-standby row,
all in the same ledger. If the pin action reads 5.1V green, the machine
and the vault both breathe again.

*Twilight Zone · J101 · J103 · J21 · the +5V rail is the constitutive
invariant · the RED state is the machine's own 1201/1202 · connector
kit first, caps second, regulator third · the constellation · fine touch
from within · vaked.dev · 8b-is, 2026-09-18*