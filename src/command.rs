use std::str::FromStr;

use rand::random;

use crate::task::Task;

pub enum Command {
    RandomTask,
    ManualTask(Vec<usize>),
    Exit,
}

impl Command {
    pub fn make_task(self) -> Option<Vec<Task>> {
        match self {
            Command::ManualTask(content) => {
                Some(content
                    .into_iter()
                    .map(Task::new)
                    .collect())
            }
            Command::RandomTask => {
                let mut result = vec!();
                for _ in 0 .. random::<usize>() % 10 {
                    result.push(Task::new(rand::random::<usize>() % 1000))
                }
                Some(result)
            }
            Command::Exit => None,
        }
    }
}

impl FromStr for Command {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let normal_form = s.to_lowercase();
        let mut word_iter  = normal_form.split(' ');
        
        let typ = word_iter.next().ok_or("no command")?;
        match typ.trim() {
            "random" => Ok(Command::RandomTask),
            "manual" => {
                let val = word_iter
                .map(|x| x.trim().parse::<usize>())
                .collect::<Result<Vec<_>, _>>()
                .map_err(|_| "couldn't parse int")?;

                Ok(Command::ManualTask(val))
            }
            "exit" => {
                Ok(Command::Exit)
            }
            _ => {
                Err(format!("type didn't match: {:?}", typ))
            }
        }
    }
}
