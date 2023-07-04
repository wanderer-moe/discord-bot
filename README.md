## discord bot made with rust & cloudflare workers
temporary readme until i have time to write a proper one

## setup

- clone repo
- install wrangler & create worker for this project, modify `wrangler.toml` to match your needs
- enter appropiate secrets using `wrangler secret put` for `DISCORD_TOKEN`, `DISCORD_APPLICATION_ID` and `DISCORD_PUBLIC_KEY`

## run locally

```bash
wrangler dev
```

## deploy

```bash
wrangler publish
```

## actions

there is a github action that will automatically deploy the worker on push to main branch & will check for idiomatic code & formatting on pull requests, as `@cloudflare/wrangler-action` doesn't support rust.

a workaround is to run `npx i -g wrangler` where you pass in your `CF_API_TOKEN` repository secret into the env in the action before running `wrangler publish`

## credits

this code is based off [mcdallas/rust-discord-bot](https://github.com/mcdallas/rust-discord-bot/tree/master))