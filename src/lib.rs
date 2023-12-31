use worker::*;

mod bot;
mod command;
mod commands;
mod embed;
mod error;
mod helpers;
mod http;
mod interaction;
mod utils;
mod verification;

fn log_request(req: &Request) {
    console_log!(
        "{} - [{}], located at: {:?}, within: {}",
        Date::now().to_string(),
        req.path(),
        req.cf().coordinates().unwrap_or_default(),
        req.cf().region().unwrap_or("unknown region".into())
    );
}

#[event(fetch)]
pub async fn main(req: Request, env: Env, _ctx: worker::Context) -> Result<Response> {
    log_request(&req);

    utils::set_panic_hook();

    let router = Router::new();

    router
        .post_async("/", |req, ctx| async move {
            let mut app = bot::App::new(req, ctx);

            match app.handle_request().await {
                Ok(result) => {
                    worker::console_log!(
                        "Response : {}",
                        serde_json::to_string_pretty(&result).unwrap()
                    );
                    Response::from_json(&result)
                }
                Err(httperr) => {
                    worker::console_log!("Error response : {}", httperr.to_string());
                    Response::error(httperr.to_string(), httperr.status as u16)
                }
            }
        })
        .post_async("/register", |_, ctx| async move {
            let commands = command::init_commands();

            let mut to_register: Vec<command::RegisteredCommand> = Vec::new();
            for boxed in commands.iter() {
                let com = boxed;
                let reg = command::RegisteredCommand {
                    name: com.name(),
                    description: com.description(),
                    options: com.options(),
                };
                to_register.push(reg);
            }

            let client = reqwest::Client::new();
            let app_id = ctx.var("DISCORD_APPLICATION_ID")?.to_string();
            let token = ctx.var("DISCORD_TOKEN")?.to_string();
            let url = format!(
                "https://discord.com/api/v10/applications/{}/commands",
                app_id
            );

            let serialized = serde_json::to_string(&to_register)?;
            worker::console_log! {"Sending  : {}", serialized};

            let response = client
                .put(url)
                .body(serialized)
                .header("Authorization", format!("Bot {}", token))
                .header("Content-Type", "application/json")
                .send()
                .await
                .unwrap()
                .text()
                .await
                .unwrap();
            worker::console_log! {"Registration response: {}", response};
            Response::ok(&response)
        })
        .run(req, env)
        .await
}
