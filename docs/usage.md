# Slack commands

Run these in the channel where you want to manage events:

| Command | Purpose |
| --- | --- |
| `/picker help` | Show command help |
| `/picker create` | Open the event creation form |
| `/picker list` | List channel events |
| `/picker show [id]` | Show an event, or select one |
| `/picker edit [id]` | Edit an event, or select one |
| `/picker delete [id]` | Delete an event, or select one |
| `/picker pick [id]` | Pick a participant, or select an event |
| `/picker repick <id>` | Repick a participant |

Event creation uses an interactive form, not JSON arguments. Participants are selected
without replacement until everyone has been selected, then a new round starts.
See the [setup instructions](../README.md) to connect your own Slack app.
