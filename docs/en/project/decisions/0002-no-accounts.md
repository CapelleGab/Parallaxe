# ADR-0002 — No user accounts

- **Status:** accepted
- **Date:** 2026-10-17

## Decision

Parallaxe has **no user accounts**, mandatory or optional: no sign-up, no login, no identity server run by the project.

## Rationale

Local-first and privacy by design are core principles. An account system would add a server to operate, personal data to protect and a dependency on the network — for no benefit to the MVP.

## Consequences

Future features such as file sharing, shared network storage or a browser extension must work **without accounts**. Directions to explore, each in its own future ADR:

- **Sharing:** capability links — end-to-end encrypted files with the key in the URL fragment, never sent to a server, with expiry and download limits; or direct peer-to-peer transfer. The declared license travels with the asset.
- **Identity without accounts:** a key pair generated locally per device when a group needs to recognize its members.
- **Network storage:** a network folder is just another root. The SQLite catalog stays local to each machine; sharing annotations would go through an export file, not a server.
- **Browser extension:** talks to the local desktop app (native messaging), not to an online service.
