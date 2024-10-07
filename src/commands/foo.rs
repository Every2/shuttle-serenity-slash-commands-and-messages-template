use serenity::all::CreateCommand;
use serenity::model::application::ResolvedOption;

pub fn run(_options: &[ResolvedOption]) -> String {
    "foo command!".to_string()
}

pub fn register() -> CreateCommand {
    CreateCommand::new("foo").description("test command!")
}
