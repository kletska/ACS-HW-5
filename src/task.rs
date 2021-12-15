use std::{sync::Mutex, collections::VecDeque};

use crate::{chat::ReviewRes, programmer::Writer};

#[derive(Debug, Clone, Copy)]
pub struct Task {
    content: usize
}

impl Task {
    pub fn new(content: usize) -> Task {
        Task { content }
    }
}

pub struct TaskManager {
    exit: Mutex<bool>,
    task_queue: Mutex<VecDeque<Task>>
}

impl TaskManager {
    pub fn new() -> Self {
        TaskManager {
            exit: Mutex::new(false),
            task_queue: Mutex::new(VecDeque::new()),
        }
    }

    pub fn send_task(&self, t: Task) {
        match self.task_queue.lock() {
            Ok(mut queue) => {
                queue.push_front(t);
            }
            Err(err) => {
                println!("painc while borrowing {:?}", err)
            }
        }
    }

    pub fn take_task(&self) -> Option<Task> {
        match self.task_queue.lock() {
            Ok(mut queque) => queque.pop_back(),
            Err(err) => {
                println!("painc while borrowing {:?}", err);
                None
            }
        }
    }

    pub fn exit(&self) {
        if let Ok(mut res) = self.exit.lock() {
            *res = true;
        }
    }

    pub fn update_exit(&self) -> bool {
        if let Ok(res) = self.exit.lock() {
            *res
        } else {
            true
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Program {
    source: usize,
}

impl Program {
    fn log(writer: &Writer, message: String) {
        if let Ok(mut writer) = writer.lock() {
            let res = writeln!(writer, "{}", message);
            if let Err(err) = res {
                println!("output is broken: {}", err);
            }
        } else {
            println!("output is broken");
        }
    }

    pub fn new(issue: Task, name: &String, writer: &Writer) -> Self {
        Self::log(writer, format!("Programmer {} wirte program {}", name, issue.content));
        
        std::thread::sleep(std::time::Duration::from_secs(1));

        Program {
            source: issue.content
        }
    }

    pub fn rewrite(&self, name: &String, writer: &Writer) -> Self {
        let mut new_source = 3 * self.source + 1;
        while new_source % 2 == 0 {
            new_source /= 2;
        }

        Self::log(writer, format!("Programmer {} rewirte program from {} to {}", name, self.source, new_source));

        std::thread::sleep(std::time::Duration::from_secs(1));

        Program {
            source: new_source,
        }
    }

    pub fn review(&self, name: &String, writer: &Writer) -> ReviewRes {
        Self::log(writer, format!("Programmer {} review program {}", name, self.source));

        std::thread::sleep(std::time::Duration::from_secs(1));

        if self.source == 1 {
            Ok(())
        } else {
            Err(())
        }
    }
}
