use std::{thread::{JoinHandle, self}, sync::{Arc, Mutex}, io::Write};

use crate::{chat::{ChatError, Chat}, task::{TaskManager, Program}};

pub(crate) type Writer = Arc<Mutex<dyn Write + Send>>;

fn init_programmer(prev: Chat, next: Chat, tm: Arc<TaskManager>, name: String, writer: Writer) -> JoinHandle<Result<(), ChatError>> {
    thread::spawn(move || -> Result<(), ChatError> {
        let prev = prev;
        let next = next;
        let task_manager = tm;
        let name = name;
        let mut writer = writer;

        let mut prog = None;
        let mut exit = false;

        while !exit {
            if prog.is_none() {
                let task = task_manager.take_task();
                prog = task.map(|task| Program::new(task, &name, &mut writer));
                if let Some(prog) = prog {
                    next.send_program(prog)?;
                }
            }

            let review_res = next.check_review()?;
            if let Some(Ok(_)) = review_res {
                prog = None;
            }
            if let Some(Err(_)) = review_res {
                prog = prog.map(|prog| prog.rewrite(&name, &mut writer));
                if let Some(prog) = prog {
                    next.send_program(prog)?;
                }
            }

            let review_prog = prev.get_program()?;
            if let Some(prog) = review_prog {
                prev.send_review(prog.review(&name, &mut writer))?;
            }

            exit = task_manager.update_exit();
        }
        Ok(())
    })
}

pub fn init_programmers(n: usize, task_manager: Arc<TaskManager>, writer: Writer) -> Vec<JoinHandle<Result<(), ChatError>>> {

    let mut vec = Vec::new();
    
    let (next_for_last_raw, prev_raw) = Chat::new();

    let mut next_for_last = Some(next_for_last_raw);
    let mut next = None;
    let mut prev = Some(prev_raw);

    let names = ["Иван", "Николай", "Олег", "Алексей", "Ярослав", "Михайил", "Юрий", "Лев"];

    for i in 0..n {
        let mut future_prev = None;
        if i == n - 1 {
            next = next_for_last.take();
        } else {
            let (new_next, future_prev_raw) = Chat::new();
            future_prev = Some(future_prev_raw);
            next = Some(new_next);
        }

        let tm  = task_manager.clone();
        let j = init_programmer(
            prev.take().unwrap(), 
            next.take().unwrap(), 
            tm, 
            format!("{} {}", names[i], i),
            writer.clone(),
        );
        prev = future_prev;
        vec.push(j);
    }

    vec
}
