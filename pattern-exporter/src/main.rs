use std::{
    any::TypeId,
    collections::hash_map::DefaultHasher,
    fmt::Display,
    hash::{Hash, Hasher},
};

use clap::{Parser, ValueEnum};
use recordkeeper::{SaveData, SystemData};
use recordkeeper_data_model::{Model, ModelName, TypeModel};

mod imhex;

#[derive(Parser)]
#[command(author, version, about, arg_required_else_help = true)]
pub struct Cli {
    #[arg(short, long, value_enum)]
    output: OutputFormat,
    #[arg(value_enum)]
    file: FileType,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
#[clap(rename_all = "lowercase")]
enum OutputFormat {
    ImHex,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum FileType {
    Save,
    System,
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct UniqueTypeName(pub String);

fn main() {
    let cli = Cli::parse();

    let model = match cli.file {
        FileType::Save => SaveData::model(),
        FileType::System => SystemData::model(),
    };
    let TypeModel::Struct(model) = model else {
        panic!()
    };

    match cli.output {
        OutputFormat::ImHex => imhex::export_imhex(&model),
    }
}

fn hash_type_id(type_id: &TypeId) -> u64 {
    let mut hash = DefaultHasher::new();
    type_id.hash(&mut hash);
    hash.finish()
}

impl From<ModelName> for UniqueTypeName {
    fn from(value: ModelName) -> Self {
        Self(value.to_string())
    }
}

impl From<(ModelName, TypeId)> for UniqueTypeName {
    fn from(value: (ModelName, TypeId)) -> Self {
        Self(format!("{}_{}", value.0, hash_type_id(&value.1)))
    }
}

impl Display for UniqueTypeName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}
