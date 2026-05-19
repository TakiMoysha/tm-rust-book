use cersei::prelude::*;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let output = Agent::builder()
        .provider()
        .tools(cersei::tools::coding())
        .permission_policy(AllowReadOnly)
        .run_with("")
        .await?;

    println("{}", output.text());
    Ok(())
}
