use crate::{
    agents::{self, Agent},
    commands::Command,
    error::CommonError,
    opt::{Opt, SubCommand},
    utils::{self, exclude, is_a_git_clone_url},
};
use clipboard::{ClipboardContext, ClipboardProvider};

#[derive(Debug)]
pub struct Parser {
    pub command: Command,
    args: Option<Vec<String>>,
}

impl Parser {
    pub fn parser_opt(opt: &Opt) -> Result<Parser, CommonError> {
        if opt.frozen {
            return Ok(Parser {
                command: Command::Frozen,
                args: None,
            });
        }

        let parser = Parser::parse_cmd(opt)?;

        Ok(parser)
    }

    fn parse_cmd(opt: &Opt) -> Result<Parser, CommonError> {
        match &opt.cmd {
            None => Ok(Parser {
                command: Command::Install,
                args: None,
            }),
            Some(sub_command) => match sub_command {
                SubCommand::Un { package_name } => match opt.global {
                    true => Ok(Parser {
                        command: Command::GlobalUninstall,
                        args: Some(package_name.clone()),
                    }),
                    false => Ok(Parser {
                        command: Command::Uninstall,
                        args: Some(package_name.clone()),
                    }),
                },
                SubCommand::Rm => Ok(Parser {
                    command: Command::RemoveNodeModules,
                    args: None,
                }),
                SubCommand::Rl => Ok(Parser {
                    command: Command::RemoveLockFile,
                    args: None,
                }),
                SubCommand::Info => Ok(Parser {
                    command: Command::PkgInfo,
                    args: None,
                }),
                SubCommand::Repo => Ok(Parser {
                    command: Command::PkgRepo,
                    args: None,
                }),
                SubCommand::R { run_name } => match run_name {
                    None => {
                        let package_json = utils::read_json_file("package.json")?;
                        let script = package_json.scripts.ok_or(CommonError::NotFound(
                            "package.json scripts field not found!".to_string(),
                        ))?;

                        let script_choices = script
                            .iter()
                            .map(|(k, v)| format!("{} - {}", k, v))
                            .collect::<Vec<String>>();

                        match script_choices.len() {
                            0 => Err(CommonError::JsonParseError(
                                "package.json scripts field is empty!".to_string(),
                            )),
                            _ => {
                                let ans = utils::select_a_choice(
                                    &script_choices,
                                    "run",
                                    "Script to run",
                                )?;

                                Ok(Parser {
                                    command: Command::Run,
                                    args: Some(vec![ans]),
                                })
                            }
                        }
                    }
                    Some(name) => Ok(Parser {
                        command: Command::Run,
                        args: Some(vec![name.to_string()]),
                    }),
                },
                SubCommand::Cl { src } => match src {
                    Some(src) => Ok(Parser {
                        command: Command::GitClone,
                        args: Some(vec![src.to_string()]),
                    }),
                    None => {
                        // TODO: catch error
                        let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
                        let content = ctx.get_contents().unwrap();
                        let mut args = vec![];
                        if is_a_git_clone_url(&content) {
                            args.push(content);
                        }
                        Ok(Parser {
                            command: Command::GitClone,
                            args: Some(args),
                        })
                    }
                },
                SubCommand::Pl => Ok(Parser {
                    command: Command::GitPull,
                    args: None,
                }),
                SubCommand::Ps => Ok(Parser {
                    command: Command::GitPush,
                    args: None,
                }),
                SubCommand::Log => Ok(Parser {
                    command: Command::GitLog,
                    args: None,
                }),
                SubCommand::Rd => Ok(Parser {
                    command: Command::Run,
                    args: Some(vec!["dev".to_string()]),
                }),
                SubCommand::Other(v) => Ok(Parser::parser_other_args(v.clone())),
            },
        }
    }

    fn parser_other_args(args: Vec<String>) -> Parser {
        if args.contains(&String::from("-g")) {
            return Parser {
                command: Command::Global,
                args: Some(exclude(args, "-g")),
            };
        }
        Parser {
            command: Command::Add,
            args: Some(args),
        }
    }
}

impl Parser {
    pub fn gene_command(&mut self, opt: &Opt) -> Result<String, CommonError> {
        match self.command {
            Command::IgnoredCommand => Ok("".to_string()),
            Command::GitClone => {
                let src = self.args.as_ref().unwrap();
                if src.len() == 0 {
                    return Err(CommonError::NotFound(
                        ("repository url not found").to_string(),
                    ));
                }
                let src = &src[0];
                Ok(format!("git clone {}", src))
            }
            Command::GitPull => Ok("git pull".to_string()),
            Command::GitPush => Ok("git push".to_string()),
            Command::GitLog => Ok("git log --graph --oneline --decorate".to_string()),
            Command::RemoveNodeModules => {
                let is_remove = utils::ask_confirm_question("Do you want to remove node_modules?")?;

                if is_remove & !opt.debug {
                    utils::remove_dir_all_file_with_path("node_modules")?;
                    println!("node_modules removed success!")
                }
                Ok("".to_string())
            }
            Command::RemoveLockFile => {
                let is_remove = utils::ask_confirm_question("Do you want to remove lockfile?")?;

                if is_remove & !opt.debug {
                    utils::remove_lock_files()?;
                    println!("lockfile removed success!")
                }
                Ok("".to_string())
            }
            Command::PkgRepo => {
                let package_json = utils::read_json_file("package.json")?;
                match package_json.repository {
                    Some(repository) => {
                        let url = repository.get("url");
                        match url {
                            Some(url) => {
                                // TODO: use regex
                                let url = if url.ends_with(".git") {
                                    url.replace(".git", "")
                                } else {
                                    url.to_string()
                                };
                                let url = if url.starts_with("git+") {
                                    url.replace("git+", "")
                                } else {
                                    url.to_string()
                                };
                                // TODO: validate url
                                Ok(format!("start {}", url))
                            }
                            None => Err(CommonError::NotFound(
                                "package.json repository url field not found!".to_string(),
                            )),
                        }
                    }
                    None => Err(CommonError::NotFound(
                        "package.json repository field not found!".to_string(),
                    )),
                }
            }
            Command::PkgInfo => {
                let package_json = utils::read_json_file("package.json")?;

                let name = package_json.name.unwrap_or("None".to_string());
                let version = package_json.version.unwrap_or("None".to_string());

                let info = format!("{} v{}", name, version);
                println!("{}", info);

                if let Some(manager) = package_json.package_manager {
                    println!("package manager: {}", manager);
                }

                Ok("".to_string())
            }
            _ => {
                let agent = agents::get_current_agent()?;

                let hash_map = Agent::get_agent_hash_map(agent);

                // instand of yarn install xxx => yarn add xxx
                match &agent {
                    Agent::Yarn | Agent::Pnpm => {
                        if self.command == Command::Install && self.args.is_some() {
                            self.command = Command::Add
                        }
                    }
                    _ => (),
                };

                match hash_map.get(&self.command) {
                    Some(cmd) => match &cmd {
                        Some(cmd) => {
                            let command = cmd.clone();
                            if command.contains("$0") {
                                match &self.args {
                                    None => Ok(command.replace("$0", "").trim().to_string()),
                                    Some(arg) => Ok(command.replace("$0", &arg.join(" "))),
                                }
                            } else {
                                Ok(command)
                            }
                        }
                        None => Ok("".to_string()),
                    },
                    None => Ok("".to_string()),
                }
            }
        }
        // don't need get agent or execute command
    }
}
