use std::convert::TryFrom;

use serde::{Deserialize, Serialize};

mod keywords {
    pub const INPUT_FILTER: &'static str = "input-filter";
    pub const CONNECTION_URI: &'static str = "connection-uri";
    pub const DATABASE_NAME: &'static str = "database-name";
    pub const COLLECTION_NAME: &'static str = "collection-name";
    pub const PIPELINE: &'static str = "pipeline";
    pub const INPUT_DOCUMENTS: &'static str = "input-documents";
    pub const INPUT_FILE: &'static str = "input-file";
    pub const PROJECT: &'static str = "project";
    pub const PIPELINE_INDEX: &'static str = "pipeline-index";
    pub const LIST: &'static str = "list";
    pub const LIMIT: &'static str = "limit";
    pub const CONFIG_FILE: &'static str = "config-file";
}

enum MongoDbCommand {
    Create,
    Aggregate,
    Find,
    FindOne,
    Count,
    DeleteMany,
    DeleteOne,
}

impl MongoDbCommand {
    fn to_str(self) -> &'static str {
        match self {
            MongoDbCommand::Create => "create",
            MongoDbCommand::Aggregate => "aggregate",
            MongoDbCommand::Find => "find",
            MongoDbCommand::FindOne => "find-one",
            MongoDbCommand::Count => "count",
            MongoDbCommand::DeleteOne => "delete",
            MongoDbCommand::DeleteMany => "delete-many",
        }
    }
}

pub fn main_app() -> clap::App<'static, 'static> {
    clap::App::new(clap::crate_name!())
        .version(clap::crate_version!())
        .author(clap::crate_authors!())
        .about(clap::crate_description!())
        .subcommand(aggregate_app())
        .subcommand(create_app())
        .subcommand(find_app())
        .subcommand(find_one_app())
        .subcommand(count_app())
        .subcommand(delete_many_app())
        .subcommand(delete_one_app())
        .arg(
            clap::Arg::with_name(keywords::CONNECTION_URI)
                .long(keywords::CONNECTION_URI)
                .required(false)
                .multiple(false)
                .takes_value(true),
        )
        .arg(
            clap::Arg::with_name(keywords::COLLECTION_NAME)
                .long(keywords::COLLECTION_NAME)
                .required(false)
                .takes_value(true),
        )
        .arg(
            clap::Arg::with_name(keywords::DATABASE_NAME)
                .long(keywords::DATABASE_NAME)
                .required(false)
                .takes_value(true),
        )
        .arg(
            clap::Arg::with_name(keywords::CONFIG_FILE)
                .long(keywords::CONFIG_FILE)
                .required(false)
                .takes_value(true),
        )
}

pub fn aggregate_app() -> clap::App<'static, 'static> {
    clap::App::new(MongoDbCommand::Aggregate.to_str())
        .about("Perform aggregation on a collection")
        .arg(
            clap::Arg::with_name(keywords::PIPELINE)
                .long(keywords::PIPELINE)
                .help("The pipeline to be executed as a string")
                .required(false)
                .takes_value(true),
        )
        .arg(
            clap::Arg::with_name("pipeline-name")
                .long("pipeline-name")
                .help(
                    "Name of the pipeline to be called. \
                    See list",
                )
                .required(false)
                .takes_value(true),
        )
        .arg(
            clap::Arg::with_name(keywords::PIPELINE_INDEX)
                .long(keywords::PIPELINE_INDEX)
                .help(
                    "Index of the pipeline to be called. \
                    See --list",
                )
                .required(false)
                .takes_value(true),
        )
        .arg(
            clap::Arg::with_name(keywords::LIST)
                .help(
                    "List all available pipelines from the configuration file. \
                    Will be empty if '--config-file' is not passed. \
                    In this case, please pass the pipeline directly to be executed through '--pipeline'",
                )
                .long(keywords::LIST)
                .required(false),
        )
}

pub fn create_app() -> clap::App<'static, 'static> {
    clap::App::new(MongoDbCommand::Create.to_str())
        .about("Insert documents into the collection")
        .arg(
            clap::Arg::with_name(keywords::INPUT_DOCUMENTS)
                .long(keywords::INPUT_DOCUMENTS)
                .help("Get the documents directly as an argument. Supports JSON lines")
                .takes_value(true)
                .required(false),
        )
        .arg(
            clap::Arg::with_name(keywords::INPUT_FILE)
                .long(keywords::INPUT_FILE)
                .help("Get the documents from a file. Support JSON lines")
                .takes_value(true)
                .required(false),
        )
}

pub fn find_args() -> Vec<clap::Arg<'static, 'static>> {
    vec![
        clap::Arg::with_name(keywords::INPUT_FILTER)
            .long(keywords::INPUT_FILTER)
            .help("The filter to be applied")
            .takes_value(true)
            .required(false),
        clap::Arg::with_name(keywords::PROJECT)
            .long(keywords::PROJECT)
            .help("Project the resulting documents")
            .takes_value(true)
            .required(false),
    ]
}

pub fn find_one_app() -> clap::App<'static, 'static> {
    clap::App::new(MongoDbCommand::FindOne.to_str())
        .about("Find the first document that matches a given filter")
        .args(&find_args())
}

pub fn count_app() -> clap::App<'static, 'static> {
    clap::App::new(MongoDbCommand::Count.to_str())
        .about("Returns the number of documents that match a given filter")
        .arg(
            clap::Arg::with_name(keywords::INPUT_FILTER)
                .long(keywords::INPUT_FILTER)
                .help("The filter to be applied")
                .takes_value(true)
                .required(false),
        )
}

pub fn delete_args() -> Vec<clap::Arg<'static, 'static>> {
    vec![clap::Arg::with_name(keywords::INPUT_FILTER)
        .long(keywords::INPUT_FILTER)
        .help("The filter to be applied")
        .takes_value(true)
        .required(false)]
}

pub fn delete_many_app() -> clap::App<'static, 'static> {
    clap::App::new(MongoDbCommand::DeleteMany.to_str())
        .about("Delete the documents that match a given filter")
        .args(&find_args())
}

pub fn delete_one_app() -> clap::App<'static, 'static> {
    clap::App::new(MongoDbCommand::DeleteOne.to_str())
        .about("Delete the first document that matches a given filter")
        .args(&find_args())
}

pub fn find_app() -> clap::App<'static, 'static> {
    let mut args = find_args();
    args.push(
        clap::Arg::with_name(keywords::LIMIT)
            .long(keywords::LIMIT)
            .help("Limit the result to N documents")
            .takes_value(true)
            .required(false),
    );
    clap::App::new(MongoDbCommand::Find.to_str())
        .about("find all the documents that matches the given filter")
        .args(&args)
}

#[derive(Serialize, Deserialize, Debug)]
struct Pipeline {
    name: String,
    description: Option<String>,
    stages: Vec<mongodb::bson::document::Document>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    connection_uri: String,
    collection_name: String,
    database_name: String,
    pipelines: Vec<Pipeline>,
}

#[derive(Debug)]
pub enum InsertResult {
    One(mongodb::results::InsertOneResult),
    Many(mongodb::results::InsertManyResult),
}

#[derive(Debug)]
enum InputType {
    Stdin(std::io::Stdin),
    Arg(String),
    BufReader(std::io::BufReader<std::fs::File>),
}

// TODO: Implement our own error type
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = main_app().get_matches();

    let config = if let Some(config_file) = matches.value_of(keywords::CONFIG_FILE) {
        let file = std::fs::File::open(config_file)?;
        serde_json::from_reader(file)?
    } else {
        match (
            matches.value_of(keywords::CONNECTION_URI),
            matches.value_of(keywords::DATABASE_NAME),
            matches.value_of(keywords::COLLECTION_NAME),
        ) {
            (Some(connection_uri), Some(database_name), Some(collection_name)) => Config {
                connection_uri: connection_uri.into(),
                database_name: database_name.into(),
                collection_name: collection_name.into(),
                pipelines: vec![],
            },
            _ => {
                return Err("Please provide the connection-uri, database-name and collection-name by passing them as arguments or through config-file".into());
            }
        }
    };
    let client = mongodb::sync::Client::with_uri_str(&config.connection_uri)?;
    let database = client.database(&config.database_name);
    let collection =
        database.collection::<mongodb::bson::document::Document>(&config.collection_name);

    if let Some(aggregate_matches) = matches.subcommand_matches(MongoDbCommand::Aggregate.to_str())
    {
        if aggregate_matches.is_present(keywords::LIST) {
            // TODO: Improve this to print using a table? But is it queryable?
            config.pipelines.iter().enumerate().for_each(|(idx, p)| {
                println!(
                    "{}. {} {}",
                    idx,
                    p.name,
                    p.description
                        .as_ref()
                        .map(|s| format!("=> {}", s))
                        .unwrap_or_default()
                )
            });
        } else if let Some(pipeline_str) = aggregate_matches.value_of(keywords::PIPELINE) {
            let value = serde_json::from_str::<serde_json::Value>(pipeline_str)?;
            match value {
                serde_json::Value::Array(pipeline) => {
                    let documents = pipeline
                        .into_iter()
                        .map(|s| match s {
                            serde_json::Value::Object(o) => {
                                mongodb::bson::document::Document::try_from(o)
                            }
                            // TODO: Properly handle this error.
                            _ => panic!("Each stage must be a valid object"),
                        })
                        .collect::<Result<Vec<_>, _>>()?;
                    let cursor = collection.aggregate(documents, None)?;
                    for result in cursor {
                        println!("{}", stringify_document(&result?));
                    }
                }
                _ => {
                    return Err("Aggregation pipeline must be an array".into());
                }
            };
        } else if let Some(pipeline_index) = aggregate_matches.value_of(keywords::PIPELINE_INDEX) {
            let index = usize::from_str_radix(pipeline_index, 10)?;
            let pipeline_count = config.pipelines.len();

            if let Some(pipeline) = config.pipelines.into_iter().nth(index) {
                let cursor = collection.aggregate(pipeline.stages, None)?;
                for result in cursor {
                    println!("{}", stringify_document(&result?));
                }
            } else {
                return Err(format!(
                    "There are only {} pipeline{} available. \
                Note that it is 0-indexed",
                    pipeline_count,
                    if pipeline_count == 1 { "" } else { "s" },
                )
                .into());
            }
        } else {
        }
    } else if let Some(create_matches) = matches.subcommand_matches(MongoDbCommand::Create.to_str())
    {
        let handle = if let Some(file) = create_matches.value_of(keywords::INPUT_FILE) {
            InputType::BufReader(std::io::BufReader::new(std::fs::File::open(file)?))
        } else if let Some(arg) = create_matches.value_of(keywords::INPUT_DOCUMENTS) {
            // TODO: Possible to avoid allocation here?
            InputType::Arg(arg.to_string())
        } else if !atty::is(atty::Stream::Stdin) {
            InputType::Stdin(std::io::stdin())
        } else {
            return Err("Please provide an input either by piping something in or specifying a file with '--input-file <file>'".into());
        };
        let doc = match handle {
            InputType::Stdin(s) => create_values_from_reader(s.lock()),
            InputType::Arg(s) => create_values_from_reader(std::io::BufReader::new(s.as_bytes())),
            InputType::BufReader(b) => create_values_from_reader(b),
        }?;
        let result = match doc.as_slice() {
            [doc] => InsertResult::One(collection.insert_one(
                convert_json_value_to_bson_document(doc).expect("Only documents can be inserted"),
                None,
            )?),
            o => InsertResult::Many(
                collection.insert_many(
                    o.iter()
                        .map(|s| convert_json_value_to_bson_document(s))
                        .collect::<Option<Vec<_>>>()
                        .expect("Only documents can be inserted"),
                    None,
                )?,
            ),
        };
        match result {
            InsertResult::One(o) => println!(
                "Successfully inserted one document with _id:{}\n",
                stringify_bson(&o.inserted_id)
            ),
            InsertResult::Many(m) => {
                println!(
                    "Successfully inserted {} document{} with _id:",
                    m.inserted_ids.len(),
                    if m.inserted_ids.len() == 1 { "" } else { "s" },
                );
                m.inserted_ids
                    .values()
                    .for_each(|s| println!("{}", stringify_bson(s)));
            }
        }
    } else if let Some(find_matches) = matches.subcommand_matches(MongoDbCommand::Find.to_str()) {
        let find_filter = find_matches
            .value_of(keywords::INPUT_FILTER)
            .map(|s| mongodb::bson::from_slice(s.as_bytes()))
            .transpose()?;
        let find_limit = find_matches
            .value_of(keywords::LIMIT)
            .map(|s| i64::from_str_radix(s, 10))
            .transpose()?;
        let find_project = find_matches
            .value_of(keywords::PROJECT)
            .map(|s| serde_json::from_slice::<serde_json::Value>(s.as_bytes()))
            .transpose()?
            .map(|v| convert_json_value_to_bson_document(&v))
            .unwrap_or_default();
        let find_options = mongodb::options::FindOptions::builder()
            .limit(find_limit)
            .projection(find_project)
            .build();
        let cursor = collection.find(find_filter, find_options)?;
        for result in cursor {
            println!("{}", stringify_document(&result?));
        }
    } else if let Some(find_one_matches) =
        matches.subcommand_matches(MongoDbCommand::FindOne.to_str())
    {
        let find_filter = find_one_matches
            .value_of(keywords::INPUT_FILTER)
            .map(|s| mongodb::bson::from_slice(s.as_bytes()))
            .transpose()?;
        let find_project = find_one_matches
            .value_of(keywords::PROJECT)
            .map(|s| serde_json::from_slice::<serde_json::Value>(s.as_bytes()))
            .transpose()?
            .map(|v| convert_json_value_to_bson_document(&v))
            .unwrap_or_default();
        let find_one_options = mongodb::options::FindOneOptions::builder()
            .projection(find_project)
            .build();
        let cursor = collection.find_one(find_filter, find_one_options)?;
        if let Some(result) = cursor {
            println!("{}", stringify_document(&result));
        } else {
            println!("No such documents");
        }
    } else if let Some(delete_one_matches) =
        matches.subcommand_matches(MongoDbCommand::DeleteOne.to_str())
    {
        let delete_one_filter = delete_one_matches
            .value_of(keywords::INPUT_FILTER)
            .map(|s| serde_json::from_slice::<serde_json::Value>(s.as_bytes()))
            .transpose()?
            .map(|v| convert_json_value_to_bson_document(&v))
            .flatten()
            .unwrap_or_default();
        let cursor = collection
            .delete_one(delete_one_filter, None)?
            .deleted_count;
        println!(
            "Deleted {} document{}",
            cursor,
            if cursor == 1 { "" } else { "s" }
        );
    } else if let Some(delete_many_matches) =
        matches.subcommand_matches(MongoDbCommand::DeleteMany.to_str())
    {
        let delete_many_filter = delete_many_matches
            .value_of(keywords::INPUT_FILTER)
            .map(|s| serde_json::from_slice::<serde_json::Value>(s.as_bytes()))
            .transpose()?
            .map(|v| convert_json_value_to_bson_document(&v))
            .flatten()
            .unwrap_or_default();
        let cursor = collection
            .delete_many(delete_many_filter, None)?
            .deleted_count;
        println!(
            "Deleted {} document{}",
            cursor,
            if cursor == 1 { "" } else { "s" }
        );
    } else if let Some(count_matches) = matches.subcommand_matches(MongoDbCommand::Count.to_str()) {
        let count_filter = count_matches
            .value_of(keywords::INPUT_FILTER)
            .map(|s| mongodb::bson::from_slice(s.as_bytes()))
            .transpose()?;
        let count_options = mongodb::options::CountOptions::builder().build();
        let count = collection.count_documents(count_filter, count_options)?;
        println!("{}", count);
    } else if let Some(subcommand) = matches.subcommand_name() {
        return Err(format!(
            "There are no subcommand '{}'. Please see --help",
            subcommand
        )
        .into());
    } else {
        main_app().print_long_help()?;
    }
    Ok(())
}

pub fn stringify_document(
    document: &mongodb::bson::document::Document,
) -> mongodb::bson::document::Document {
    document
        .iter()
        .map(|(key, value)| (key.clone(), stringify_bson(value)))
        .collect()
}

pub fn stringify_bson(document: &mongodb::bson::Bson) -> mongodb::bson::Bson {
    match document {
        mongodb::bson::Bson::ObjectId(id) => mongodb::bson::Bson::String(id.to_string()),
        mongodb::bson::Bson::DateTime(d) => mongodb::bson::Bson::String(d.to_chrono().to_rfc3339()),
        o => o.clone(),
    }
}

pub fn create_values_from_reader<R>(
    reader: R,
) -> Result<Vec<serde_json::Value>, Box<dyn std::error::Error>>
where
    R: std::io::BufRead,
{
    let mut vec = Vec::new();
    for result in serde_json::Deserializer::from_reader(reader).into_iter::<serde_json::Value>() {
        let value = result?;
        match value {
            serde_json::Value::Array(mut arr) => {
                vec.reserve(arr.len());
                vec.append(&mut arr)
            }
            o => vec.push(o),
        }
    }
    Ok(vec)
}

pub fn convert_json_value_to_bson_document(
    json: &serde_json::Value,
) -> Option<mongodb::bson::Document> {
    match json {
        serde_json::Value::Object(o) => Some(
            o.iter()
                .map(|s| (s.0.clone(), convert_json_to_bson(s.1)))
                .collect::<mongodb::bson::Document>(),
        ),
        _ => None,
    }
}

pub fn convert_json_to_bson(json: &serde_json::Value) -> mongodb::bson::Bson {
    match json {
        serde_json::Value::Null => mongodb::bson::Bson::Null,
        serde_json::Value::Bool(b) => mongodb::bson::Bson::Boolean(*b),
        serde_json::Value::Number(n) => mongodb::bson::Bson::Double(
            n.as_f64()
                .expect("Cannot convert JSON number to BSON double"),
        ),
        serde_json::Value::String(s) => mongodb::bson::Bson::String(s.clone()),
        serde_json::Value::Array(v) => {
            mongodb::bson::Bson::Array(v.iter().map(convert_json_to_bson).collect())
        }
        serde_json::Value::Object(o) => mongodb::bson::Bson::Document(
            o.iter()
                .map(|s| (s.0.clone(), convert_json_to_bson(s.1)))
                .collect::<mongodb::bson::Document>(),
        ),
    }
}
