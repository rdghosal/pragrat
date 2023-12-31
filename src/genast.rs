use std::{
    fs::File,
    io::{Result, Write},
};

pub fn generate_ast(output_dir: &str) {
    define_ast(
        output_dir,
        "Expr",
        &vec![
            "BoolOp         = op: Token, values: Vec<Box<dyn Expr<T>>>",
            "NamedExpr      = target: Box<dyn Expr<T>>, value: Box<dyn Expr<T>>",
            "BinOp          = left: Box<dyn Expr<T>>, op: Token, right: Box<dyn Expr<T>>",
            "UnaryOp        = op: Token, operand: Box<dyn Expr<T>>",
            "Lambda         = args: Arguments, body: Box<dyn Expr<T>>",
            "IfExp          = test: Box<dyn Expr<T>>, body: Box<dyn Expr<T>>, orelse: Box<dyn Expr<T>>",
            "Dict           = keys: Vec<Box<dyn Expr<T>>>, values: Vec<Box<dyn Expr<T>>>",
            "Set            = elts: Vec<Box<dyn Expr<T>>>",
            "ListComp       = elt: Box<dyn Expr<T>>, generators: Vec<Comprehension>",
            "SetComp        = elt: Box<dyn Expr<T>>, generators: Vec<Comprehension>",
            "DictComp       = key: Box<dyn Expr<T>>, value: Box<dyn Expr<T>>, generators: Vec<Comprehension>",
            "GeneratorExp   = key: Box<dyn Expr<T>>, value: Box<dyn Expr<T>>, generators: Vec<Comprehension>",
            "AwaitExpr      = value: Box<dyn Expr<T>>",
            "YieldExpr      = value: Option<Box<dyn Expr<T>>>",
            "YieldFrom      = value: Box<dyn Expr<T>>",
            "Compare        = left: Box<dyn Expr<T>>, ops: Vec<Token>, comparators: Vec<Box<dyn Expr>>",
            "Call           = func: Box<dyn Expr<T>>, Args: Vec<Box<dyn Expr<T>>>, keywords: Vec<Keyword>",
            "FormattedValue = value: Box<dyn Expr<T>>, conversion: Int, format_spec: Box<dyn Expr<T>>",
            "JoinedStr      = values: Vec<Box<dyn Expr<T>>>",
            "Constant       = value: Literal, kind: Option<String>",
            "Attribute      = value: Box<dyn Expr<T>>, attr: Identifier, ctx: ExprContext",
            "Subscript      = value: Box<dyn Expr<T>>, slice: Box<dyn Expr>, ctx: ExprContext",
            "Starred        = value: Box<dyn Expr<T>>, ctx: ExprContext",
            "Name           = id: Identifier, ctx: ExprContext",
            "List           = elts: Vec<Box<dyn Expr<T>>>, ctx: ExprContext",
            "Tuple          = elts: Vec<Box<dyn Expr<T>>>, ctx: ExprContext",
            "Slice          = lower: Option<Box<dyn Expr<T>>>, upper: Option<Box<dyn Expr<T>>>, step: Option<Box<dyn Expr<T>>>",
        ],
    )
    .expect("Failed to generate AST expressions");
}

fn define_ast(output_dir: &str, base_name: &str, types: &Vec<&str>) -> Result<()> {
    let path: String = format!("{}/{}.rs", output_dir, base_name.to_lowercase());
    match File::create(path) {
        Ok(mut f) => {
            f.write(b"use crate::types::{Literal, Token};\n\n")?;
            define_visitor(&mut f, base_name, &types)?;
            f.write(format!("trait {}<T> {{\n", base_name).as_bytes())?;
            // The base accept() method.
            f.write(b"\tfn accept(&self, visitor: Box<dyn Visitor<T>>) -> T;\n")?;
            f.write(b"}\n\n")?;
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
    f.write(format!("trait Visitor<T> {{\n").as_bytes())?;
    for t in types {
        let substrs: Vec<&str> = t.split("=").collect();
        let class_name = substrs[0].trim();
        let fields = substrs[1].trim();
        let is_generic = fields.contains("<T>");
        let generic_cls_name = if is_generic {
            format!("{}<T>", class_name)
        } else {
            class_name.to_string()
        };
        f.write(
            format!(
                "\tfn visit_{}_{}(&self, {}: &{}) -> T;\n",
                class_name.to_lowercase(),
                base_name.to_lowercase(),
                class_name.to_lowercase(),
                generic_cls_name,
            )
            .as_bytes(),
        )?;
    }
    f.write(b"}\n\n")?;
    Ok(())
}

fn define_type(f: &mut File, base_name: &str, class_name: &str, field_list: &str) -> Result<()> {
    let is_generic = field_list.contains("<T>");
    let generic_cls_name = if is_generic {
        format!("{}<T>", class_name)
    } else {
        class_name.to_string()
    };

    f.write(format!("struct {} {{\n", generic_cls_name).as_bytes())?;

    // Fields.
    let fields: Vec<&str> = field_list.split(", ").collect();
    for field in &fields {
        f.write(format!("\t{},\n", field).as_bytes())?;
    }
    f.write(b"}\n\n")?;

    // Visitor pattern.
    f.write(format!("impl<T> Expr<T> for {} {{\n", generic_cls_name).as_bytes())?;
    f.write(b"\tfn accept(&self, visitor: Box<dyn Visitor<T>>) -> T {\n")?;
    f.write(
        format!(
            "\t\treturn visitor.visit_{}_{}(self);\n",
            class_name.to_lowercase(),
            base_name.to_lowercase()
        )
        .as_bytes(),
    )?;
    f.write(b"\t}\n")?;
    f.write(b"}\n\n")?;
    Ok(())
}
