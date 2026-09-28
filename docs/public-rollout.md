# Public rollout

## Intended outcome

An independently operated successor that workspace admins can install from a
permanent Add to Slack page. Start with Troop and Spotnana as separate installations
while they migrate to Spotnana. Keep DirtyDieselCorp as the development workspace.
Do not assume event data, user IDs or channel IDs automatically migrate between
workspaces; any migration needs an explicit mapping and validation.

## Current status

The fork has a working local development path and a verified participant-edit fix.
A temporary developer-machine tunnel is not a public hosting commitment. There is
no production installation URL yet, and the original Slack listing is not owned
by this fork.

## Before an external pilot

- Clarify the upstream MIT declaration and missing licence file.
- Choose an operator, hosting account, budget and permanent HTTPS origin.
- Package a reproducible runtime image that excludes .env, credentials and build
  logs. Store runtime secrets separately from source and images.
- Provision MongoDB with backups and a separate production database.
- Add an OAuth installation entry point with state validation; remove credential
  and authorization-code logging, including at trace level.
- Verify workspace isolation on every event read/write, including Slack Connect
  shared-channel cases; test simultaneous installations and participant updates.
- Replace the single SPECIAL_TEAM_ID exception with a documented event-limit
  policy that applies consistently to all installed workspaces.
- Test scheduled delivery, restart recovery, daylight-saving changes and year
  boundaries. Run one service instance until scheduler coordination is implemented.
- Configure the production Slack app callbacks and complete the distribution
  checklist. Keep production credentials separate from the test app.
- Pilot installation, creation, editing, picking, scheduling and uninstall/reinstall
  in independent workspaces. Check that uninstalling stops scheduled activity and
  define how workspace data is removed.

## Before general availability

Publish accurate privacy/data-retention and support information, add monitoring
and a backup restore procedure, and assign maintenance ownership. Check Slack's
current Marketplace requirements before broad or commercial distribution. Keep
an unlisted pilot distinct from a reviewed Marketplace app.

References:

- [Slack distribution](https://docs.slack.dev/app-management/distribution/)
- [Slack Marketplace](https://docs.slack.dev/slack-marketplace/distributing-your-app-in-the-slack-marketplace/)
- [Slack hosting](https://docs.slack.dev/app-management/hosting-slack-apps/)
