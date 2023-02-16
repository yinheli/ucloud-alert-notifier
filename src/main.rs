use actix_web::{middleware, web, App, HttpServer};
use clap::Parser;
use log::info;

mod cli;
mod error;
mod feishu;
mod handler;

fn main() {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let cli = cli::Cli::parse();
    match cli.command {
        cli::Command::Serve(args) => serve(args),
    }
}

fn serve(args: cli::ServeArg) {
    let rt = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap();
    rt.block_on(async move {
        info!("ucloud alert notifier server bind: {}", &args.bind);
        HttpServer::new(move || {
            App::new()
                .app_data(web::Data::new(feishu::FeiShu::new(
                    args.webhook.clone(),
                    args.secret.clone(),
                )))
                .wrap(middleware::Logger::default())
                .route("/", web::to(handler::version))
                .route("/notify", web::post().to(handler::notify))
                .route("/push", web::post().to(handler::notify))
        })
        .bind(args.bind)?
        .run()
        .await
    })
    .unwrap();
}
