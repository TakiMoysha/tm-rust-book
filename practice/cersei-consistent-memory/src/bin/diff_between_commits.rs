use std::path::Path;

use anyhow::Context;
use cersei::{prelude::*, tools::skill_tool::SkillTool};

const AGENT_PROMPT: &str = r"
You are a memory maintenance agent for a codebase.
- `AGENTS.md` - key decisions made, development principles, tuning and documentation for them (how I write code and arhicteture decisions).
- `MEMORY.md` - current project state and context (what I know about the code now). May be part of `AGENTS.md`. Can describe macro level systems (if it is required).
You should write code changes to the MEMORY.md file (it is not a changelog, just for context).

## Current MEMORY.md Content
{}

## Changes Since Last Commit
{}

Your task:
1. Analyze if the code changes invalidate or require updates to memory sections (call git-diff tools)
2. Check if current memory accurately reflects the code
3. If there are changes that are unclear (and there is no comment why this is so), they should be written down. Ask clarifying questions about why certain decisions were made.
4. Suggest specific updates, additions, or archiving
";

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let provider = OpenAi::builder()
        .base_url(&std::env::var("OPENAI_BASE_URL").context("OPENAI_BASE_URL is not set")?)
        .api_key(&std::env::var("OPENAI_API_KEY").context("OPENAI_API_KEY is not set")?)
        .model("assistant")
        .build()?;

    let config_dir = match Path::new(env!("XDG_CONFIG_HOME")) {
        p if p.exists() => p,
        _ => Path::new(env!("XDG_CONFIG_HOME")).into(),
    };

    let scills_list = SkillTool::new().with_project_root(config_dir);

    let output = Agent::builder()
        .provider(provider)
        .permission_policy(AllowReadOnly)
        .system_prompt(AGENT_PROMPT)
        .run_with("")
        .await?;

    println!("{}", output.text());
    Ok(())
}

mod tools {
    use cersei::prelude::schemars::JsonSchema;
    use cersei::prelude::serde::Deserialize;
    use cersei::prelude::*;
    use cersei::tools as cersei_tools;
    use cersei::tools::ToolExecute;

    #[derive(Deserialize, JsonSchema)]
    struct GitDiffInput {
        commit_hash: String,
    }

    #[derive(Tool)]
    #[tool(
        name = "git-diff",
        description = "Get a diff between current and get commit",
        permission = "none"
    )]
    struct GitDiff;

    #[async_trait]
    impl ToolExecute for GitDiff {
        type Input = GitDiffInput;
        async fn run(&self, input: Self::Input, _ctx: &ToolContext) -> ToolResult {
            let process_git = std::process::Command::new("git")
                .args(["diff", &input.commit_hash])
                .output()
                .expect("can't execute git");

            if process_git.status.success() {
                let output = String::from_utf8_lossy(&process_git.stdout);
                // sanitize, remove unreadable and non-recognized chars
                ToolResult::success(format!("{}", &output))
            } else {
                let output = String::from_utf8_lossy(&process_git.stderr);
                ToolResult::error(format!("Can't show diff: {}", &output))
            }
        }
    }

    #[cfg(test)]
    mod test {
        #[test]
        fn should_return_diff() {
            let hash = "c23b021";
            let process_git = std::process::Command::new("git")
                .arg("diff")
                .arg(hash)
                .output()
                .expect("can't execute git");

            println!("{}", String::from_utf8_lossy(&process_git.stdout));
        }
    }
}
