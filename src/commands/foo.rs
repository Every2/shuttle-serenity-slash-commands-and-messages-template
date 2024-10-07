use serenity::all::CreateCommand;
use serenity::model::application::ResolvedOption;

//Função que manda a mensagem quando o slash command é trigerrado
pub fn run(_options: &[ResolvedOption]) -> String {
    "foo command!".to_string()
}

//Função pra registrar o comando na main
pub fn register() -> CreateCommand {
    CreateCommand::new("foo").description("test command!")
}