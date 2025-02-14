use serenity::all::{
    AutocompleteChoice,
    AutocompleteOption,
    CommandOptionChoice,
    CommandOptionType,
    CreateCommandOption,
    ResolvedOption,
    ResolvedValue,
};
use serenity::builder::CreateCommand;
use serenity::json::Value;

pub fn autocomplete(autocomplete_option: AutocompleteOption<'_>) -> Vec<AutocompleteChoice> {
    let input = autocomplete_option.value;
    if input.len() == 0 {
        return vec![];
    }
    (1..=20)
        .map(|i| {
            AutocompleteChoice::new(format!("{i}. {input}"), format!("{i}"))
            // AutocompleteChoice(CommandOptionChoice {
            //     name: format!("{i}. {input}"),
            //     value: Value::String(String::from(i)),
            //     name_localizations: None,
            // })
        })
        .collect()
}

pub fn run(options: &[ResolvedOption]) -> String {
    let result = if let Some(ResolvedOption {
        value: ResolvedValue::String(value), ..
    }) = options.first()
    {
        format!("Selected value: `{value}`")
    } else {
        "Please provide a valid user".to_string()
    };
    dbg!(&result);

    result
}

pub fn register() -> CreateCommand {
    CreateCommand::new("wonderful_command").description("An amazing command").add_option(
        CreateCommandOption::new(
            CommandOptionType::String,
            "value",
            "A value to be auto-completed",
        )
        .set_autocomplete(true)
        .required(true),
    )
}
