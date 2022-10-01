use clap::{Parser, Subcommand};
use dotenv::dotenv;
use owo_colors::OwoColorize;
use zenode::{Operator, StrTuple};

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    #[clap(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Create schema, requires name, description and fields
    #[clap(id = "cs")]
    CreateSchema {
        /// Name of the schema
        #[clap(short, long)]
        name: String,
        /// Description of the schema
        #[clap(short, long)]
        description: String,
        /// Fields in the shape: `-f name:str -f id:int`
        #[clap(short, long)]
        fields: Vec<String>,
        /// If present, logs the operation
        #[clap(short, long)]
        log: bool,
    },
    /// Create instance, requires schema_id and fields
    #[clap(id = "ci")]
    CreateInstance {
        /// schema_id in the shape: `<schema-name>_0203a905630971fa...`
        #[clap(short, long)]
        schema_id: String,
        /// Fields in the shape: `-f name:bob -f id:123`
        #[clap(short, long)]
        fields: Vec<String>,
        /// If present, logs the operation
        #[clap(short, long)]
        log: bool,
    },
    /// Update instance, requires schema_id, view_id, and fields to update
    #[clap(id = "ui")]
    UpdateInstance {
        /// schema_id in the shape: `<schema-name>_0203a905630971fa...`
        #[clap(short, long)]
        schema_id: String,
        /// view_id in the shape: `00202aaa1ef8ef9d...`
        #[clap(short, long)]
        view_id: String,
        /// Fields in the shape: `-f name:bob -f id:123`
        #[clap(short, long)]
        fields: Vec<String>,
        /// If present, logs the operation
        #[clap(short, long)]
        log: bool,
    },
    /// Delete instance, requires schema_id, last view_id
    #[clap(id = "di")]
    DeleteInstance {
        /// schema_id in the shape: `<schema-name>_0203a905630971fa...`
        #[clap(short, long)]
        schema_id: String,
        /// view_id in the shape: `00202aaa1ef8ef9d...`
        #[clap(short, long)]
        view_id: String,
        /// If present, logs the operation
        #[clap(short, long)]
        log: bool,
    },
}

#[tokio::main]
async fn main() -> Result<(), String> {
    use Commands::*;
    let cli = Cli::parse();
    dotenv().ok();
    let op = Operator::default();

    if let Some(ref command) = cli.command {
        match command {
            CreateSchema {
                name,
                description,
                fields,
                log,
            } => {
                let mut fields = map_to_str_fields(fields);
                if *log {
                    print_create_schema(name, description, &fields);
                }

                let id = op.create_schema(name, description, &mut fields).await?;
                println!("ID: {}", id.cyan());
            }
            CreateInstance {
                schema_id,
                fields,
                log,
            } => {
                let mut fields = map_to_str_fields(fields);
                if *log {
                    print_create_instance(schema_id, &fields);
                }

                let id = op.create_instance(schema_id, &mut fields).await?;
                println!("ID: {}", id.cyan());
            }
            UpdateInstance {
                schema_id,
                view_id,
                fields,
                log,
            } => {
                let mut fields = map_to_str_fields(fields);
                if *log {
                    print_update_instance(schema_id, view_id, &fields);
                }

                let id = op.update_instance(schema_id, view_id, &mut fields).await?;
                println!("ID: {}", id.cyan());
            }
            DeleteInstance {
                schema_id,
                view_id,
                log,
            } => {
                if *log {
                    print_delete_instance(schema_id, view_id);
                }

                let id = op.delete_instance(schema_id, view_id).await?;
                println!("ID: {}", id.cyan());
            }
        };
    }

    Ok(())
}

/// Utility functions to map cli fields to Operator fields
/// maps `"name:str"` -> `("name", "str")`
fn map_to_str_fields(fields: &[String]) -> Vec<(&'_ str, &'_ str)> {
    let x = fields
        .iter()
        .map(|it| -> (&str, &str) {
            let (a, b) = it
                .split_once(':')
                .expect("Missing delimiter ':' in field\nExpected field shape `a:b`. Got: {:?}");
            (a.trim(), b.trim())
        })
        .collect::<Vec<_>>();
    x
}

fn print_create_schema(name: &str, description: &str, fields: &Vec<StrTuple<'_>>) {
    println!("Creating schema...");
    println!(
        "name: {}\ndescription: {}",
        name.magenta(),
        description.yellow()
    );

    print_fields(fields);
    println!();
}

fn print_create_instance(schema_id: &str, fields: &Vec<StrTuple<'_>>) {
    println!("Creating instance...");
    println!("schema_id: {}", schema_id.magenta(),);
    print_fields(fields);
    println!();
}

fn print_update_instance(schema_id: &str, view_id: &str, fields: &Vec<StrTuple<'_>>) {
    println!("Updating instance...");
    println!(
        "schema_id: {}\nview_id: {}",
        schema_id.magenta(),
        view_id.yellow(),
    );
    print_fields(fields);
    println!();
}

fn print_delete_instance(schema_id: &str, view_id: &str) {
    println!("Deleting instance...");
    println!(
        "schema_id: {}\nview_id: {}",
        schema_id.magenta(),
        view_id.yellow()
    );
    println!();
}

fn print_fields(fields: &Vec<StrTuple<'_>>) {
    print!("fields: ");
    fields.iter().cloned().enumerate().for_each(|(i, (a, b))| {
        print!("{}: {}", a.cyan(), b.bright_green());
        if i != fields.len() - 1 {
            print!(", ")
        }
    });

    println!()
}
