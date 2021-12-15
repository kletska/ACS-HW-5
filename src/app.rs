use std::{sync::Arc, io::BufRead, env};

use crate::{task::TaskManager, programmer::{init_programmers, Writer}, command::Command};

pub(crate) struct Config {
    pub n: usize,
    pub input: Option<String>,
    pub output: Option<String>,
}

pub(crate) fn parse_args() -> Result<Config, ()> {
    let args = env::args().collect::<Vec<_>>();

    if args.len() == 3 {
        println!("specify input and output paths or nither of them");
        return Err(());
    }

    if args.len() > 4 {
        println!("too much arguments");
        return Err(());
    }

    let n = if args.len() <= 1 {
        3
    } else {
        args[1].parse::<usize>().map_err(|_| ())?
    };

    if args.len() <= 2 {
        return Ok(Config {
            n,
            input: None,
            output: None,
        });
    }

    let input = args[2].clone();

    let output = args[3].clone();

    Ok(Config {
        n,
        input: Some(input),
        output: Some(output),
    })
}


pub fn run_app(n: usize, reader: Box<dyn BufRead>, writer: Writer) -> Result<(), Box<dyn std::error::Error>> {
    let task_manager = Arc::new(TaskManager::new());

    let handlers = init_programmers(n, task_manager.clone(), writer.clone());

    for result in reader.lines() {
        let command = result?
            .parse::<Command>()?;
        
        match command.make_task() {
            Some(tasks) => {
                for t in tasks {
                    task_manager.send_task(t);
                }
            }
            None => {
                task_manager.exit();
                break;
            }
        }
    }

    for j in handlers {
        match j.join() {
            Ok(Ok(_)) => (),
            Ok(Err(err)) => return Err(Box::new(err)),
            Err(_err) => {
                if let Ok(mut writer) = writer.lock() {
                    write!(writer, "this thread has panicked")?;
                } else {
                    println!("output is broken");
                }
            }
        }
    }

    Ok(())
}
