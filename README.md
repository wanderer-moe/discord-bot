## Serverless Discord Bot Created with Rust & Cloudflare Workers

[![Rust Report Card](https://rust-reportcard.xuri.me/badge/github.com/wanderer-moe/discord-bot)](https://rust-reportcard.xuri.me/report/github.com/wanderer-moe/discord-bot)

## Setup

- Clone repo `git clone https://github.com/wanderer-moe/discord-bot` and use it in your own repository.
- Install wrangler & create a cf worker for this project
- Modify `wrangler.toml` to match your needs (D1 binding, Worker Name, etc)
- Enter appropiate secrets using `wrangler secret put` for your bots `DISCORD_TOKEN`, `DISCORD_APPLICATION_ID` and `DISCORD_PUBLIC_KEY`
- Generate a Cloudflare API based off the "Edit Cloudflare Workers" template, and add it as `CF_API_TOKEN` in your repository secrets, this is used for the github action to deploy the worker.
- Add your Interactions Endpoint URL on your App - this allows interactions to be recieved over POST requests.
- You can publish your worker using `wrangler publish`, then create a POST request to `https://<worker-url>/register` - this will register your application commands with discord.

## Run locally

```bash
wrangler dev
```

## Deploy

```bash
wrangler publish
```

## Actions

There is a github action that will automatically deploy the worker on push to main branch & will check for idiomatic code & formatting on pull requests, as `@cloudflare/wrangler-action` doesn't support rust-wasm workers yet,

A workaround is to run `npx i -g wrangler` where you pass in your `CF_API_TOKEN` repository secret into the env in the action before running `wrangler publish`.

## Credits

This code is based off [mcdallas/rust-discord-bot](https://github.com/mcdallas/rust-discord-bot/tree/master)

## License

This repository is licensed under the [GNU Affero General Public License v3.0](https://choosealicense.com/licenses/gpl-3.0/) license — **You must state all significant changes made to the original software, make the source code available to the public with credit to the original author, original source, and use the same license.**