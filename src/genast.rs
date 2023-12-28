use std::{
    env,
    fs::File,
    io::{Result, Write},
    process,
};

pub fn generate_ast(output_dir: &str) {
    define_ast(
        output_dir,
        "Expr",
        &vec![
            "Binary   =  left: Expr, operator: Token, right: Expr",
            "Grouping =  expression: Expr",
            "Literal  =  value: any",
            "Unary    =  operator: Token, right: Expr",
        ],
    )
    .expect("Failed to generate AST expressions");
}

fn define_ast(output_dir: &str, base_name: &str, types: &Vec<&str>) -> Result<()> {
    let path: String = format!("{}/{}.ts", output_dir, base_name.to_lowercase());
    match File::create(path) {
        Ok(mut f) => {
            f.write(b"import Token from './token';\n\n")?;
            define_visitor(&mut f, base_name, &types)?;
            f.write(format!("export abstract class {} {{\n", base_name).as_bytes())?;
            // The base accept() method.
            f.write(b"\tabstract accept<R>(visitor: Visitor<R>): R;\n")?;
            f.write(b"};\n\n")?;
            for t in types {
                let substrs: Vec<&str> = t.split("=").collect();
                let class_name = substrs[0].trim();
                let fields = substrs[1].trim();
                define_type(&mut f, base_name, class_name, fields)?;
            }
        }
        Err(e) => panic!("{}", e),
    };
    Ok(())
}

fn define_visitor(f: &mut File, base_name: &str, types: &Vec<&str>) -> Result<()> {
    f.write(format!("export interface Visitor<R> {{\n").as_bytes())?;
    for type_ in types {
        let type_name: &str = type_.split("=").collect::<Vec<&str>>()[0].trim();
        f.write(
            format!(
                "\tvisit{}{}({}: {}): R;\n",
                type_name,
                base_name,
                base_name.to_lowercase(),
                type_name
            )
            .as_bytes(),
        )?;
    }
    f.write(b"};\n\n")?;
    Ok(())
}

fn define_type(f: &mut File, base_name: &str, class_name: &str, field_list: &str) -> Result<()> {
    f.write(format!("export class {} extends {} {{\n", class_name, base_name).as_bytes())?;

    // Fields.
    let fields: Vec<&str> = field_list.split(", ").collect();
    for field in &fields {
        f.write(format!("\tpublic readonly {};\n", field).as_bytes())?;
    }

    // Constructor.
    f.write(format!("\tconstructor({}) {{\n", field_list).as_bytes())?;
    f.write(b"\t\tsuper();\n")?;
    for field in &fields {
        let name = field.split(": ").collect::<Vec<&str>>()[0];
        f.write(format!("\t\tthis.{} = {};\n", name, name).as_bytes())?;
    }
    f.write(b"\t}\n")?;

    // Visitor pattern.
    f.write(b"\n")?;
    f.write(b"\taccept<R>(visitor: Visitor<R>): R {\n")?;
    f.write(
        format!(
            "\t\treturn visitor.visit{}{}(this);\n",
            class_name, base_name
        )
        .as_bytes(),
    )?;
    f.write(b"\t}\n")?;

    f.write(b"};\n\n")?;
    Ok(())
}
