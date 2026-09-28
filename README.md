# Team Event Picker

A Rust Slack app for creating team events and selecting participants without repeating
someone until everyone has had a turn. MongoDB stores events and workspace OAuth tokens.

This fork revives [jotar910/team-event-picker](https://github.com/jotar910/team-event-picker).
It is a Slack backend, not a standalone web UI. Start with a development workspace.

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

Local build, unit tests, MongoDB lifecycle test, and HTTP startup are verifiable without
Slack credentials. Live OAuth, Slack messages, interactive actions, and scheduled posts
still need an end-to-end check in your workspace. The inherited dependency stack and
scheduling behavior have not received a full modernization or production audit.
