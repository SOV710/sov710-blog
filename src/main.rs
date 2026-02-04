pub mod ast;
pub mod html;

use orgize::Org;
use std::fs;

use crate::ast::AstExporter;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let org_content = fs::read_to_string("src/test.org")
        .map_err(|e| format!("FAILED to READ test.org: {}", e))?;

    let org = Org::parse(org_content);
    let mut exporter = AstExporter::default();
    org.traverse(&mut exporter);

    println!("{}", exporter.output);
    Ok(())
}
