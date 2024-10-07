use serenity::builder::CreateCommand;
use serenity::model::application::ResolvedOption;

pub fn run(_options: &[ResolvedOption]) -> String {
    "too!".to_string()
}

pub fn register() -> CreateCommand {
    CreateCommand::new("too").description("too command!")
}
