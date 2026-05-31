use clap::{Parser, Subcommand};

use mutsura_api::command::{add_admin, serve};

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    // serve: サーバーとして機能
    // add-admin: adminユーザーを作成
    Serve,
    AddAdmin {
        j_name: String,   // Japanese name
        e_name: String,   // English name
        email: String,    // E-mail
        password: String, // Password
    },
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    match cli.command {
        Command::Serve => {
            serve().await;
        }
        Command::AddAdmin {
            j_name,
            e_name,
            email,
            password,
        } => {
            let _ = add_admin(j_name, e_name, email, password).await;
        }
    }
}
