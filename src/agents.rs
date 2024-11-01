use std::{collections::HashMap, fs};
use crate::{commands::Command, error::CommonError, package_json::PackageJson, utils};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Agent {
    Bun,
    Pnpm,
    Npm,
    Yarn,
    None,
}

impl From<String> for Agent {
    fn from(agent: String) -> Self {
        match agent {
            agent if agent == "npm" => Agent::Npm,
            agent if agent == "yarn" => Agent::Yarn,
            agent if agent == "bun" => Agent::Bun,
            agent if agent == "pnpm" => Agent::Pnpm,
            _ => Agent::None,
        }
    }
}

impl From<Agent> for String {
    fn from(agent: Agent) -> Self {
        match agent {
            agent if agent == Agent::Npm => "npm".to_string(),
            agent if agent == Agent::Pnpm => "pnpm".to_string(),
            agent if agent == Agent::Yarn => "yarn".to_string(),
            agent if agent == Agent::Bun => "bun".to_string(),
            _ => "Not Found".to_string(),
        }
    }
}

impl Agent {
    pub fn get_agent_hash_map(agent: Agent) -> HashMap<Command, Option<String>> {
        match agent {
            Agent::Npm | Agent::None => HashMap::from([
                (Command::Agent, Some("npm $0".to_string())),
                (Command::Run, Some("npm run $0".to_string())),
                (Command::Install, Some("npm i $0".to_string())),
                (Command::Frozen, Some("npm ci".to_string())),
                (Command::Global, Some("npm i -g $0".to_string())),
                (Command::Add, Some("npm i $0".to_string())),
                (Command::Upgrade, Some("npm update $0".to_string())),
                (Command::UpgradeInteractive, None),
                (Command::Execute, Some("npx $0".to_string())),
                (Command::Uninstall, Some("npm uninstall $0".to_string())),
                (
                    Command::GlobalUninstall,
                    Some("npm uninstall -g $0".to_string()),
                ),
            ]),
            Agent::Bun => HashMap::from([
                (Command::Agent, Some("bun $0".to_string())),
                (Command::Run, Some("bun run $0".to_string())),
                (Command::Install, Some("bun install $0".to_string())),
                (Command::Frozen, Some("bun install --no-save".to_string())),
                (Command::Global, Some("bun add -g $0".to_string())),
                (Command::Add, Some("bun add $0".to_string())),
                (Command::Upgrade, None),
                (Command::UpgradeInteractive, None),
                (Command::Execute, None),
                (Command::Uninstall, Some("bun remove $0".to_string())),
                (
                    Command::GlobalUninstall,
                    Some("npm remove -g $0".to_string()),
                ),
            ]),
            Agent::Yarn => HashMap::from([
                (Command::Agent, Some("yarn $0".to_string())),
                (Command::Run, Some("yarn run $0".to_string())),
                (Command::Install, Some("yarn install $0".to_string())),
                (
                    Command::Frozen,
                    Some("yarn install --frozen-lockfile".to_string()),
                ),
                (Command::Global, Some("yarn global add $0".to_string())),
                (Command::Add, Some("yarn add $0".to_string())),
                (Command::Upgrade, Some("yarn upgrade $0".to_string())),
                (
                    Command::UpgradeInteractive,
                    Some("yarn upgrade-interactive $0".to_string()),
                ),
                (Command::Execute, Some("yarn dlx $0".to_string())),
                (Command::Uninstall, Some("yarn remove $0".to_string())),
                (
                    Command::GlobalUninstall,
                    Some("yarn global remove $0".to_string()),
                ),
            ]),
            Agent::Pnpm => HashMap::from([
                (Command::Agent, Some("pnpm $0".to_string())),
                (Command::Run, Some("pnpm run $0".to_string())),
                (Command::Install, Some("pnpm i $0".to_string())),
                (
                    Command::Frozen,
                    Some("pnpm i --frozen-lockfile".to_string()),
                ),
                (Command::Global, Some("pnpm add -g $0".to_string())),
                (Command::Add, Some("pnpm add $0".to_string())),
                (Command::Upgrade, Some("pnpm update $0".to_string())),
                (
                    Command::UpgradeInteractive,
                    Some("pnpm update -i $0".to_string()),
                ),
                (Command::Execute, Some("pnpm dlx $0".to_string())),
                (Command::Uninstall, Some("pnpm remove $0".to_string())),
                (
                    Command::GlobalUninstall,
                    Some("pnpm remove --global $0".to_string()),
                ),
            ]),
        }
    }
}

pub struct Agents {
    pub lock_map: HashMap<String, Agent>,
}

impl Agents {
    pub fn new() -> Agents {
        Agents {
            lock_map: HashMap::from([
                ("bun.lockb".to_string(), Agent::Bun),
                ("pnpm-lock.yaml".to_string(), Agent::Pnpm),
                ("yarn.lock".to_string(), Agent::Yarn),
                ("package-lock.json".to_string(), Agent::Npm),
                ("npm-shrinkwrap.json".to_string(), Agent::Npm),
            ]),
        }
    }
}

pub fn get_current_agent() -> Result<Agent, CommonError> {
    let package_json = PackageJson::from_path("package.json")?;

    let agent = match package_json.package_manager {
        Some(manager) => {
            let manager = manager.to_lowercase();
            let manager = manager.split('@').collect::<Vec<&str>>()[0];
            manager.to_string().into()
        }
        None => {
            let agents = Agents::new();
            for (file_name, agent) in agents.lock_map.into_iter() {
                let is_found = fs::read(&file_name).is_ok();
                if is_found {
                    println!("Current agent is {}", String::from(agent));
                    return Ok(agent);
                }
            }
            let agents = vec![Agent::Npm, Agent::Pnpm, Agent::Yarn, Agent::Bun]
                .iter()
                .map(|&a| a.into())
                .collect::<Vec<String>>();

            utils::select_a_choice(&agents, "agent", "Choose the agent")?.into()
        }
    };

    println!("Current agent is {}", String::from(agent));
    Ok(agent)
}
