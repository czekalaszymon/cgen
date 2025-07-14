use anyhow::Result;

pub fn run(project_name: Option<String>, generate_clang_format: bool) -> Result<()> {
    println!("Stub: creating new project {:?}", project_name);
    println!("Generate .clang-format: {}", generate_clang_format);
    Ok(())
}
