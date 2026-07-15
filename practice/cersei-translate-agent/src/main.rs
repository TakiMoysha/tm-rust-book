use std::path::Path;

/// cersei translated agent
/// cersei-agent - основной движок
/// cersei-tools - тузлы, facilities для агента, есть пресеты, например coding (fs, git, shell, etc.)
/// cersei-tools-derive - макросы для создания тузлов
/// cersei-memory - персистентность, сохранение ссесий, состояния, контектс и гидрация (аля rag).
/// graph - использует grafeo для работы с графами
///
/// TODO:
/// - read `contributing.md`: требования к contributing
/// - перевод (можно использовать api deepl (с глосарием)), вычитка от нейронки (терминология внутренний глоссарий), вычитка человеком
/// -
use cersei::prelude::*;
use cersei::tools::skill_tool::SkillTool;
use clap::{Command, arg};
use log::info;

const IS_DEV: bool = cfg!(debug_assertions);

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config_dir = match Path::new(env!("XDG_CONFIG_HOME")) {
        p if p.exists() => p,
        _ => Path::new(env!("XDG_CONFIG_HOME")).into(),
    };

    info!("Config dir: {}", config_dir.display());

    let args = Command::new("translate")
        .version("0.1.0")
        .about("translate file")
        .arg(arg!([FILE] "Target file to translate").required(IS_DEV))
        .arg(arg!([GLOSSARY] "Path to glossary"))
        .arg(arg!([VERBOSE] "Verbose mode (mode logs)"))
        .get_matches();

    env_logger::init();

    const PROMPT_ROLE: &str = r#"
        Ты работаешь как агент по переводу текста.
    "#;

    let agent = Agent::builder()
        .provider(OpenAi::from_env()?)
        .model("gpt-5-nano")
        .tools(cersei::tools::coding())
        .permission_policy(AllowReadOnly)
        .system_prompt(PROMPT_ROLE);

    {
        // TODO: added glossary (skill) and proofread (skill)
        //
        // let scills_list = SkillTool::new().with_project_root(config_dir);
        //
        // args.get_one::<String>("GLOSSARY").map(|glossary| {
        //     agent.add_skill(scills_list.with_glossary(glossary.to_string()));
        // });
    }

    Ok(())
}

mod tools {
    use cersei::prelude::schemars::JsonSchema;
    use cersei::prelude::*;
    use serde::Deserialize;

    // monkey patch for cersei derives
    // use cersei::tools as cersei_tools;

    #[derive(Tool)]
    #[tool(name = "deepl_translate", description = "translate text via deepl")]
    struct DeeplTool {
        api_key: String,
    }

    #[derive(Deserialize, JsonSchema)]
    struct DeeplTranslateInput {
        text: String,
    }

    #[async_trait]
    impl ToolExecute for DeeplTool {
        type Input = DeeplTranslateInput;
        async fn run(&self, input: DeeplTranslateInput, _ctx: &ToolContext) -> ToolResult {}
    }
}
