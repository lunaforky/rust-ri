use crate::{agents::Agents, error::CommonError};
use requestty::{ListItem, OnEsc, Question};
use std::{fs, path::Path, process};

pub fn exclude(args: Vec<String>, v: &str) -> Vec<String> {
    args.into_iter()
        .filter(|arg| arg != v)
        .collect::<Vec<String>>()
}

pub fn select_a_choice(
    vec_choices: &Vec<String>,
    name: &str,
    message: &str,
) -> Result<String, CommonError> {
    let select = Question::select(name)
        .message(message)
        .choices(vec_choices)
        .on_esc(OnEsc::Terminate)
        .transform(|choice, _previous_answers, backend| {
            write!(
                backend,
                "{}",
                choice.text.split(" - ").collect::<Vec<&str>>()[0]
            )
        })
        .build();

    let answer = requestty::prompt_one(select)?;

    match answer {
        requestty::Answer::ListItem(ListItem { text, .. }) => {
            let ans: Vec<&str> = text.split(" - ").collect();
            Ok(ans[0].to_string())
        }
        _ => process::exit(1),
    }
}

pub fn remove_dir_all_file_with_path<P: AsRef<Path>>(path: P) -> Result<(), CommonError> {
    fs::remove_dir_all(path)?;
    Ok(())
}

pub fn remove_lock_files() -> Result<(), CommonError> {
    for (file_name, _) in Agents::new().lock_map {
        if fs::read(&file_name).is_ok() {
            fs::remove_file(file_name)?;
        }
    }
    Ok(())
}

pub fn ask_confirm_question(question_content: &str) -> Result<bool, CommonError> {
    let confirm = Question::confirm("q")
        .message(question_content)
        .default(false)
        .build();

    let confirm = requestty::prompt_one(confirm)?.as_bool();

    match confirm {
        Some(b) => Ok(b),
        None => Ok(false),
    }
}

/// judge content is a git clone url or not.
///
/// such as `git@xxx/xxx/xxx.git` or `http(s)://xxx/xxx/xxx.git`
pub fn is_a_git_clone_url(ctx: &str) -> bool {
    ctx.ends_with(".git") && (ctx.starts_with("http") || ctx.starts_with("git@"))
}
