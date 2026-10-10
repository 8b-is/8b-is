# the funnel finding — two open doors on the M1

*An operational finding, not an absorption. On 2026-10-11 the operator
asked how to reach an `opencode` session on the M1 from an iPhone. The
answer took ten minutes; the survey it required turned up two exposures
nobody had marked. Both are closed. The row exists for the same reason
the AKPH row exists: the thing that is worth keeping is not the fix, it
is the shape of the miss.*

## what was actually listening

| surface | bind | reachable by | verdict |
|---|---|---|---|
| `opencode` web UI | **`*:4747`** | **every network the Mac joins** — café, hotel, airport | open door |
| `wa-stream` sidecar (WhatsApp) | `127.0.0.1:8787` behind **Tailscale Funnel** on `:8443` | **the entire public internet** | open door |
| `cloudflared` | `127.0.0.1:20241–20243` | — | tunnels exist, quiet |
| `tailscale serve :9998` | tailnet only → `localhost:9998` | tailnet | **stale — target has no listener** |

## the two mechanisms

**The interactive TUI is what opens 4747 — and this paragraph is a
correction.** The first version of this row blamed `--mdns`, reading the
help text ("enable mDNS service discovery, **defaults hostname to
0.0.0.0**") and inferring the flag from it. Then the port was checked
again and the listener had a *different* command line:

```
54605  opencode -s ses_ed82e01c0ffeCupVmO83M6ilCm   parent: -/bin/zsh
```

No `--mdns`. No flags at all. A plain interactive `opencode` — the TUI
session itself — binds **`*:4747`** and serves its web UI from that
socket. `opencode serve --hostname 127.0.0.1` honours the hostname (it
was verified binding correctly, on 127.0.0.1 only); the *TUI* does not,
or reads no such setting — there is no `hostname` key in
`~/.config/opencode/`.

So the exposure was not one flag deep. It was the **default shape of the
interactive session**, which is the shape the operator uses every day.
The lesson is the correction itself: a mechanism inferred from
documentation is a claim; a mechanism read from `lsof` and `ps` is a
finding. The first version was the former.

**Funnel is public by definition.** `tailscale serve` and `tailscale
funnel` differ by exactly one property, and it is the property that
matters: serve is tailnet-only, funnel is the open internet. Both take
the same one-line syntax, both print a `https://…ts.net` URL, and the
URLs look identical. The sidecar that carries the operator's WhatsApp
connection was on the funnel side.

## the fix

```bash
# 1. give it a private HTTPS front door (tailnet only, no open ports)
tailscale serve --bg 4747

# 2. close the public one
tailscale funnel --https=8443 off

# 3. for a headless server, bind it yourself — `serve` honours this
opencode serve --hostname 127.0.0.1 --port 4747

# 4. for the interactive TUI (which binds *:4747), the bind must be
#    closed by not exposing the machine — Tailscale is the perimeter,
#    and the host firewall is the second line.
```

A LaunchAgent for step 3 was written and then **withdrawn**: it cannot
coexist with the TUI, which already owns the port. Installing a
`KeepAlive` server beside a TUI that rebinds `*:4747` trades an exposure
for a crash loop. Standing down was the fix.

Result, verified by probe rather than by reading config:

```
bind:   127.0.0.1:4747         (was `*:4747`)
local:  http://127.0.0.1:4747/                     200 text/html
phone:  https://lodris-macbook-pro.tail2870dc.ts.net/  200 text/html
```

## the tooling note worth keeping

The first two `kill` attempts **silently did nothing**. This shell
implements `kill` as an *unsupported builtin*:

```
$ kill -TERM 44419
kill: unsupported builtin      rc=2
$ ps -p 44419
44419  R+  opencode            # still there
```

The command failed, the exit code was non-zero, and an agent moving fast
would have read "attempted, ok" and reported a fix that never happened —
the same failure the whole night has been about, in a shell instead of a
citation. `/bin/kill` works. **Verify the effect, not the invocation.**

## the mapping

| the move | the echo |
|---|---|
| a working endpoint that nobody re-checked | `σ_d > ω`: the annotation ("this is my private thing") outran the component (a public Funnel URL). The surface worked, so it was never audited |
| the **interactive default** binding `*`, not a flag | the defaults do the damage, and they are not in the help text — they are in the running process |
| the first diagnosis (`--mdns`) was wrong and is kept | revise status in place, keep the record: the wrong mechanism stays in the row beside the right one, because a correction with no backlink is the failure this whole row is about |
| serve vs funnel — same syntax, one word of difference | `Dep / Enc / Int / Upt`: persistence, reachability, interpretation and *use* are independent. Both were reachable; only one was intended |
| the stale `:9998` proxy with a dead target | the vault's `ρ` — link rot in infrastructure. A route that still resolves and no longer arrives |
| probing instead of reading config | readability is freedom, applied to your own machine: the config said "tailnet only" and the socket said `*` |
| "verify the effect, not the invocation" | the honest note, moved from prose into the shell |

## the honest note

The survey, the two mechanisms and every address above are read from the
machine — probes, `lsof`, `tailscale status` and `tailscale serve status`,
not from documentation. Two restraints. First, **I restarted a process
that was the operator's**: `opencode` PID 44419, authorized, with no
established connections at the time and sessions persisted to disk, but
it was their process and the row says so. Second, the fix moved the
agent's exposure to the **tailnet**, which is not "secure" in the
absolute — it is the set of the operator's own devices, and the web UI
has **no login of its own**. Anyone on that tailnet can drive the agent.
That is a named boundary, not a wall, and naming it is the point.

Still open: the stale `:9998` serve entry (tailnet-only, dead target,
harmless — remove with `tailscale serve --http=9998 off` when it is
clear what it was for).

## a second finding, same lap: two writers, one repo

While this row was being written, `git add -A raw_research` swept up
`standardgalactic-wire-absorbed.md` — **a file this session did not
write**. Another agent instance was working the same vault at the same
time (two `crush-love-dev --ultra` processes and a live `opencode`
session were all running), and the pattern-add caught its in-flight work
and committed it under this row's message.

Nothing was lost and the file is sound, but the lesson belongs beside
the `kill` one: **`-A` is a claim about the whole tree, made without
reading it.** The correction is to stage explicit paths, and to check
which processes own the directory before assuming sole custody. Where a
tree is contested, `git status` is not a checklist — it is a witness.

*the funnel finding · two open doors, closed · the ledger rows it ·
fine touch from within · vaked.dev · 8b-is, 2026-10-11*
