use std::{
    collections::{HashSet, VecDeque},
    io::{Result, Write},
};

use recordkeeper_data_model::{StructModel, TypeModel};

use crate::UniqueTypeName;

pub fn export_imhex(root: &StructModel) {
    let mut structs = VecDeque::new();
    structs.push_back(root);
    let mut written = HashSet::new();

    let mut out = Vec::new();

    while let Some(str) = structs.pop_back() {
        let name = UniqueTypeName::from((str.name, str.type_id));
        if written.insert(name.clone()) {
            write_struct(&mut out, &str, &name, &mut structs).unwrap();
        }
    }

    println!("#pragma pattern_limit 0x80000");

    for b in [8, 16, 32, 64] {
        println!("using i{b} = s{b};");
    }

    println!("using f32 = float;");
    println!("using f64 = double;");

    println!(
        r#"
struct Array<T, auto Size> {{
    T data[Size];
}};
        "#
    );

    for str in written {
        println!("using {str};");
    }

    std::io::stdout().write_all(&out).unwrap();

    println!(
        "{} root @ 0x0;",
        UniqueTypeName::from((root.name, root.type_id))
    );
}

fn write_struct<'a>(
    mut writer: impl Write,
    model: &'a StructModel,
    name: &UniqueTypeName,
    structs: &mut VecDeque<&'a StructModel>,
) -> Result<()> {
    writeln!(&mut writer, "struct {} {{", name)?;

    let mut offset = 0;
    for field in &model.fields {
        if field.offset > offset {
            writeln!(&mut writer, "    padding[{}];", field.offset - offset)?;
        }
        offset = field.offset + field.size;
        match &field.type_model {
            TypeModel::Empty => continue,
            TypeModel::Struct(s) => structs.push_back(s.as_ref()),
            TypeModel::Array(arr) => {
                if let TypeModel::Empty = &arr.0 {
                    continue;
                }
                if let TypeModel::Struct(s) = &arr.0 {
                    structs.push_back(s.as_ref())
                }
            }
            _ => {}
        }

        writeln!(
            &mut writer,
            "    {};",
            fmt_field(&field.type_model, field.name),
        )?;
    }

    if offset < model.total_len {
        writeln!(&mut writer, "    padding[{}];", model.total_len - offset)?;
    }

    writeln!(&mut writer, "}};")?;
    Ok(())
}

fn fmt_field(ty: &TypeModel, name: &str) -> String {
    match ty {
        TypeModel::Empty => format!("char {name}[0]"),
        TypeModel::Struct(s) => format!("{} {name}", UniqueTypeName::from((s.name, s.type_id))),
        TypeModel::Array(arr) => format!("Array<{}, {}> {}", fmt_naked_type(&arr.0), arr.1, name),
        TypeModel::Primitive(t) => format!("{t} {name}"),
    }
}

fn fmt_naked_type(ty: &TypeModel) -> String {
    match ty {
        TypeModel::Empty => panic!("unsupported"),
        TypeModel::Struct(s) => UniqueTypeName::from((s.name, s.type_id)).to_string(),
        TypeModel::Array(arr) => format!("Array<{}, {}>", fmt_naked_type(&arr.0), arr.1),
        TypeModel::Primitive(t) => t.to_string(),
    }
}
