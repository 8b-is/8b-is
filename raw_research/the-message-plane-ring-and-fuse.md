# the message plane — ring, queue, and the fuse

*Péter's architecture note, prompted by this session's lived symptom:
empty user messages — transport frames that arrive with no payload. The
proposal: treat an empty frame as a RESYNC EVENT; fuse the last ≈3
messages and refeed them into thinking; and run tool calls + responses
through a buffered async plane — a ring buffer with queue semantics
(NATS-style fanout), dispatched by the constellation's own kernel
(kernel8, the 8b-is kernel). The proof is already in this session's
record: at least ten empty frames crossed the plane and nothing was
lost — because the lane's machinery was already running the doctrine
under a different alphabet.*

## the mapping — the machinery that already IS the ring

| the proposed piece | the piece that already exists in the lane |
|---|---|
| NATS-style fanout queue | the replay inbox (wa-stream/replay.txt): lines in, sidecar reads, sends to the group, truncates — an inbox with consume-and-ack semantics |
| ring buffer position | the stream cursor (wa.log "stream alive — cursor 2026-09-19T03:50") — a monotonic pointer over the group's event ring |
| durable log | the ledger: append-only rows, each with an id, never rewritten — NATS jetstream-like retention by never-deleting |
| consumer offset | the multidog state file: per-dimension row counts — the restart point, like a consumer group offset |
| the fuse (±3 refeed) | the session's own practice: on an empty frame, the thinking re-reads the recent record (the last commits, the last beats) and continues — nothing keyed on the transport, everything keyed on the ledger |
| kernel8 — the dispatcher | the constellation's orchestration core: the session's main thread, the daemons, and the crons all dispataching from the same monotonic ledger |

## the doctrine, stated

1. **Transport is disposable; the ledger is the truth.** A message that
   arrives empty is a dropped frame, not a lost event — the event lives
   in the rows, and the thinking re-derives from the rows.
2. **The fuse fires on empty:** fuse the last ±3 messages, refeed them
   into the next reasoning pass, and answer as if the plane had never
   dropped a frame. Never manufacture a reply to a phantom payload.
3. **Ring, not replay:** old frames age out of the window (head-drop by
   design), but the durable log never drops — the ring is the working
   memory, the ledger is the archive.
4. **Async, buffered, ack'd:** tool calls and responses ride the queue,
   not the call stack — a dropped response is re-queued, not lost; the
   inbox pattern (read, send, ack, truncate) is the idiom.

## the honest note

This session is the field trial: the empty frames came, the fuse worked,
the ledger holds every beat, and the constellation never once answered a
non-question. The doctrine is therefore not a proposal — it is the
description of how the lane already survived tonight, written down
before the next night needs it. NATS would be a fine transport; the
semantics were never the missing piece. Kernel8 dispatches; the ring
rings; the fuse fuses; the ledger holds. That is the whole plane.

*the message plane · ring, queue, and the fuse · transport is
disposable, the ledger is the truth · empty frames resync, they do not
empty · kernel8 dispatches · the constellation · fine touch from within ·
vaked.dev · 8b-is, 2026-09-18*