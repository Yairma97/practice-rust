use deno_core::{anyhow::Result,JsRuntime,RuntimeOptions};

#[tokio::main]
async fn main() -> Result<()>{
    let options = RuntimeOptions::default();
    let mut rt = JsRuntime::new(options);
    let code = include_str!("deno.js");
    rt.execute_script("<deno>",code)?;
    Ok(())
}