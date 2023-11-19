use dialoguer::{theme::ColorfulTheme, Input, MultiSelect};

pub fn input(prompt: &str) -> String {
    Input::with_theme(&ColorfulTheme::default())
        .with_prompt(prompt)
        .default("wasm-project".into())
        .validate_with(|input: &String| -> Result<(), &str> {
            if input.trim().is_empty() {
                Err("请输入项目名称")
            } else {
                Ok(())
            }
        })
        .interact()
        .unwrap()
}

pub fn multi_select<'a, 'b>(
    prompt: &'b str,
    options: &'a Vec<String>,
    defaults: &'a Vec<bool>,
) -> Vec<&'a String> {
    let selected = MultiSelect::with_theme(&ColorfulTheme::default())
        .with_prompt(prompt)
        .items(options)
        // 注意defaults顺序必须得放items后面
        .defaults(defaults)
        .interact()
        .unwrap();
    let selected_items = selected
        .iter()
        .map(|i| options.get(*i).unwrap())
        .collect::<Vec<_>>();
    selected_items
}

pub fn multi_select_return_usize<'a, 'b>(
    prompt: &'b str,
    options: &'a Vec<String>,
    defaults: &'a Vec<bool>,
) -> Vec<usize> {
    MultiSelect::with_theme(&ColorfulTheme::default())
        .with_prompt(prompt)
        .items(options)
        // 注意defaults顺序必须得放items后面
        .defaults(defaults)
        .interact()
        .unwrap()
}
