# Team Event Picker

A Rust Slack app for creating team events and selecting participants without repeating
someone until everyone has had a turn. MongoDB stores events and workspace OAuth tokens.

This fork revives [jotar910/team-event-picker](https://github.com/jotar910/team-event-picker).
It is a Slack backend, not a standalone web UI. Start with a development workspace.

## Fork status

This fork includes the latest upstream commit checked on 2026-09-28, local setup
repairs, continuous integration, and a fix for edited participant selections.
Removing a participant now removes them from the saved event; retained participants
keep their pick history.

You can use the instructions below to run your own development installation. There
is **no permanent hosted service or public Add to Slack link yet**. See
[the public rollout plan](docs/public-rollout.md) for the remaining work.

## Upstream attribution and licence

The original author is [jotar910](https://github.com/jotar910/team-event-picker).
The upstream README states: “This project is licensed under the MIT License”, but
its referenced LICENSE file is missing and GitHub does not detect a licence.
That declaration is recorded here for attribution; this fork does not invent a
replacement licence grant. Clarify the applicable terms before a public service
or company deployment.

## Local setup

Requirements: Rust (tested with 1.90.0), Docker Desktop with Compose, and a Slack
workspace where you can create an app.

```sh
git clone https://github.com/dirtydiesel46/team-event-picker.git
cd team-event-picker
cp .env.tmpl .env
docker compose up -d --wait
cargo run --locked
```

If `.env` already exists, edit it instead of overwriting it. The template's database
settings match the local Compose service. Its credentials are for local development;
MongoDB is published only on `127.0.0.1:27017`, with data in a named Docker volume.

Check the server from another terminal:

```sh
curl --fail http://localhost:8080/health
```

Expected response: `OK`. Placeholder Slack credentials allow this local startup check,
but must be replaced before using Slack. The backend listens on port 8080 by default.

## Run the backend in Docker

After configuring your own Slack app and MongoDB, build and run the backend:

```sh
docker build -t team-event-picker:local .
docker run --rm --name team-event-picker --env-file .env -p 8080:8080 team-event-picker:local
```

Set `PORT=8080` for this mapping. Inside a container, `127.0.0.1` refers to that
container, so replace both database URLs with an address it can reach. For the
local Compose database on Docker Desktop, use `host.docker.internal` instead of
`127.0.0.1`. On a server, use your database's private network address and credentials.
The image runs as a non-root user and contains the compiled binary, not your .env.
Use a persistent database and a stable HTTPS reverse proxy for ongoing use. Run
only one backend instance because each instance runs its own scheduler.

## Connect a new Slack app

1. Make port 8080 reachable through a development HTTPS tunnel. Keep the server and
   tunnel running while testing.
2. Replace all three `https://YOUR-PUBLIC-HOST` URLs in `slack-manifest.yml` with that
   public origin. In [Slack app management](https://api.slack.com/apps), create an app
   **From a manifest**, select your development workspace, and paste the manifest.
3. From the app's **Basic Information**, fill in `.env`: `APP_ID`, `CLIENT_ID`,
   `CLIENT_SECRET`, and `SIGNATURE` (Slack's **Signing Secret**). Restart `cargo run --locked`.
4. In **OAuth & Permissions**, confirm the sole redirect URL ends in `/api/oauth`.
   Open the following authorization URL in your browser after replacing `YOUR_CLIENT_ID`:

   ```text
   https://slack.com/oauth/v2/authorize?client_id=YOUR_CLIENT_ID&scope=commands,channels:join,chat:write
   ```

   Complete authorization through this URL so the app's callback exchanges the code
   and stores the bot token in MongoDB. A `BOT_TOKEN` environment variable is not used.
5. Invite the bot to a test channel. Try `/picker help`, `/picker create`, then
   `/picker list` and `/picker pick <id>`. Use the interactive form to add participants.

The manifest configures the command endpoint (`/api/commands`), interactive actions
(`/api/actions`), and OAuth callback (`/api/oauth`). It follows Slack's
[manifest reference](https://docs.slack.dev/reference/app-manifest/) and
[OAuth installation flow](https://docs.slack.dev/authentication/installing-with-oauth/).
Update all callback URLs if your tunnel's hostname changes.

The inherited plan guard allows one event per channel unless `SPECIAL_TEAM_ID` matches
your workspace ID. Set that optional variable for your own workspace to lift this
restriction; `MAX_EVENTS` alone does not override that guard.

## Verification

```sh
cargo test --locked
docker compose config --quiet
TEST_MONGODB_URL='mongodb://picker:local-development-only@127.0.0.1:27017/?authSource=admin' \
  cargo test --locked --test local_mongodb -- --ignored
```

The MongoDB integration test uses a unique database, checks creation, channel isolation,
selection through a complete round and reset, persisted state, and deletion. It drops its
own database after success. A failed assertion may leave a `picker_test_*` database for inspection.

The old `test_migration` and `test_copy` functions are manual data-changing utilities.
They are ignored by default and are not required for a fresh installation. Do not run
all ignored tests indiscriminately. CI runs unit tests and the isolated MongoDB test.

## Stop and restart

Stop the Rust server with Ctrl-C. Stop MongoDB with `docker compose stop`; use
`docker compose up -d --wait` to restart it. Its named volume preserves events and tokens.

## Current verification limits

Verified: Rust 1.90 build, unit tests, isolated MongoDB lifecycle and participant-edit
regressions, and GitHub CI. OAuth installation and a signed command request were
verified with a development Slack app; a tester confirmed participant edits work
in Slack. Scheduled delivery across restarts, year boundaries and timezones still
needs validation. The inherited dependency stack and authentication have not
received a full modernization or production audit.
