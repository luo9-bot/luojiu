# Growth Lab — Phase A (observation only)

Growth Lab exists to measure whether Luojiu's behavior improves across comparable situations. Phase A establishes an observation baseline; it does **not** learn, tune, or deploy behavior.

## Scope

The first instrumented signal is the existing group-chat SpeakScore gate in `src/conversation/handler.rs`.

For batches that already pass through the ordinary SpeakScore gate, the observer records:

- timestamp and schema version;
- batch size (not message content);
- score, threshold, and the existing `pass` / `silent` decision;
- the existing score breakdown: relevance, attention, unanswered-question bonus, freshness, addressing, fatigue, social risk, recent reply, and seconds since the previous reply when known.

The event is recorded after the score has been computed. The observer's return value is never consulted by the conversation handler. It cannot change whether Luojiu speaks.

## Storage

- Runtime file: `data/plugin_ai_chat/growth_lab/events.jsonl` (under the configured plugin data directory).
- Format: one JSON object per line; `schema_version` starts at `1`.
- Writes run on a dedicated background thread behind a bounded queue (4,096 pending events).
- Queue-full behavior is deliberately lossy: the observation is dropped rather than blocking the chat path.
- The observer does not write raw messages, group IDs, user IDs, names, prompts, memory contents, or model responses.

The file contains behavioral metadata and can still reveal usage patterns. Protect it as operational data, restrict filesystem access, and apply a retention policy appropriate to your deployment. Phase A does not transmit data over the network.

## WebUI

The authenticated admin console exposes the live report at **实验 → 成长观测** via `GET /api/growth-lab`. The observer reconstructs its aggregate counters once at startup, then maintains a bounded in-memory summary as new events are written. The API returns aggregate counters and at most 200 recent events; it does not rescan the entire JSONL file on each page refresh.

## What Phase A can and cannot prove

It can establish distributions such as gate scores, silence/pass rates, and score-feature trends over time. By itself, that is **not proof of growth**: a score distribution may shift because the traffic mix changed.

It cannot yet establish that replies are more correct, more relevant, more socially appropriate, or more consistent with Luojiu's identity. Those claims require a fixed, versioned replay dataset and held-out evaluation in a later phase.

## Operational checks

1. Build and run the existing test suite on the target platform.
2. Confirm that `events.jsonl` appears after ordinary group messages reach the SpeakScore path.
3. Compare decisions before and after enabling observation on the same replay input; they must be identical.
4. Verify that the JSONL file contains no message text or user/group identifiers.
5. Simulate an unavailable data directory or a full queue; message processing must continue.
6. Keep Phase A telemetry observational. Do not let the model edit its identity kernel, beliefs, prompts, memory, thresholds, or weights.

## Exit criteria for Phase A

- A stable schema and documented privacy boundary.
- No behavior changes attributable to the observer.
- A representative baseline dataset with known coverage gaps.
- A report describing the baseline distribution and limitations.
- Only then design Phase B: controlled offline evaluation against a frozen baseline and held-out cases.

## Known coverage gap

This first increment records only ordinary group batches that reach the SpeakScore gate. Directly addressed messages, forced replies, crisis paths, asleep/passive paths, and other early returns are not captured. Do not interpret the dataset as a census of all incoming messages.
