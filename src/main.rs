use clap::{Parser, Subcommand};
use dotenv::dotenv;
use zenode::Operator;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    #[clap(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Creates an schema, requires name, description and fields
    CreateSchema {
        #[clap(short = 'n', long, value_parser)]
        name: String,
        #[clap(short = 'd', long, value_parser)]
        description: String,
        /// Fields in the shape: name:str, id:int
        #[clap(short = 'f', long, value_parser)]
        fields: String,
    },
    /// Creates a schema instance, requires schema_id and fields
    CreateInstance {
        /// schema_id in the shape: <schema-name>_0020cae3b
        #[clap(short = 's', long, value_parser)]
        schema_id: String,
        /// Fields in the shape: name:str, id:int
        #[clap(short = 'f', long, value_parser)]
        fields: String,
    },
    /// Updates a schema instance, requires schema_id, view_id (instance document_id if is new), and fields to update
    UpdateInstance {
        /// schema_id in the shape: <schema-name>_0020cae3b
        #[clap(short = 's', long, value_parser)]
        schema_id: String,
        /// view_id in the shape: 00cae4b2a
        #[clap(short = 'v', long, value_parser)]
        view_id: String,
        /// Fields in the shape: name:str, id:int
        #[clap(short = 'f', long, value_parser)]
        fields: String,
    },
    /// Deletes a schema instance, requires schema_id, last view_id (instance document_id if is new)
    DeleteInstance {
        /// schema_id in the shape: <schema-name>_0020cae3b
        #[clap(short = 's', long, value_parser)]
        schema_id: String,
        /// view_id in the shape: 00cae4b2a
        #[clap(short = 'v', long, value_parser)]
        view_id: String,
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
            } => {
                println!("{} {} {:?}", name, description, fields);
                let fields = field_string_to_vec(fields);
                let mut fields = map_to_str_fields(&fields);
                let id = op.create_schema(name, description, &mut fields).await?;
                println!("ID: {}", id);
            }
            CreateInstance { schema_id, fields } => {
                println!("{} {:?}", schema_id, fields);
                let fields = field_string_to_vec(fields);
                let mut fields = map_to_str_fields(&fields);
                let id = op.create_instance(schema_id, &mut fields).await?;
                println!("ID: {}", id);
            }
            UpdateInstance {
                schema_id,
                view_id,
                fields,
            } => {
                println!("{} {} {:?}", schema_id, view_id, fields);
                let fields = field_string_to_vec(fields);
                let mut fields = map_to_str_fields(&fields);
                let id = op.update_instance(schema_id, view_id, &mut fields).await?;
                println!("ID: {}", id);
            }
            DeleteInstance { schema_id, view_id } => {
                println!("{} {}", schema_id, view_id);
                let id = op.delete_instance(schema_id, view_id).await?;
                println!("ID: {}", id);
            }
        };
    }

    Ok(())
}

fn field_string_to_vec(field_string: &String) -> Vec<String> {
    let x = field_string
        .replace(" ", "")
        .split(",")
        .map(|x| x.to_string())
        .collect::<Vec<_>>();
    x
}

/// Utility functions to map cli fields to Operator fields
/// maps `"name:str"` -> `("name", "str")`
fn map_to_str_fields<'a>(fields: &'a Vec<String>) -> Vec<(&'a str, &'a str)> {
    let x = fields
        .iter()
        .map(|it| -> (&str, &str) {
            let s = it.split(":").into_iter().collect::<Vec<_>>();
            match s.as_slice() {
                [a, b] => (a.clone(), b.clone()),
                _ => {
                    panic!(
                        "Invalid field shape:\nExpected field shape `a:b`.Got: {:?}",
                        it
                    )
                }
            }
        })
        .collect::<Vec<_>>();
    x
}
