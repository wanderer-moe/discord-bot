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

there is a github action that will automatically deploy the worker on push to main branch & will check for idiomatic code & formatting on pull requests.
