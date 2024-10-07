use serenity::builder::CreateCommand;
use serenity::model::application::ResolvedOption;

//Função que manda a mensagem quando o slash command for trigerrado
pub fn run(_options: &[ResolvedOption]) -> String {
    "too!".to_string()
}

//Função necessaria pra registrar o comando na main.rs
pub fn register() -> CreateCommand {
    CreateCommand::new("too").description("too command!")
}