use std::{sync::mpsc::{Sender, Receiver, self, SendError, TryRecvError}, fmt::Display};

use crate::task::Program;

pub type ReviewRes = Result<(), ()>;

pub enum Message {
    Review(ReviewRes),
    Code(Program),   
}

#[derive(Debug)]
pub struct ChatError {
    message: String,
}

impl Display for ChatError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Chat error: {}", self.message)
    }
}

impl std::error::Error for ChatError {}

pub struct Chat {
    ouput: Sender<Message>,
    input: Receiver<Message>,
}

impl Chat {
    pub fn new() -> (Self, Self) {
        let (first_tx, first_rx) = mpsc::channel();
        let (second_tx, second_rx) = mpsc::channel();

        (Chat {
            ouput: first_tx,
            input: second_rx,
        },
        Chat {
            ouput: second_tx,
            input: first_rx,
        })
    }

    pub fn send_program(&self, p: Program) -> Result<(), ChatError> {
        match self.ouput.send(Message::Code(p)) {
            Ok(_) => Ok(()),
            Err(SendError(_)) => {
                Err(ChatError { message: "channel is closed".to_string() })
            }
        }
    }

    pub fn get_program(&self) -> Result<Option<Program>, ChatError> {
        match self.input.try_recv() {
            Ok(Message::Code(prog)) => Ok(Some(prog)),
            Ok(Message::Review(_)) => {
                Err(ChatError { message: "prev programmer shouldn't send his review".to_string() })
            }
            Err(TryRecvError::Disconnected) => Ok(None),
            Err(TryRecvError::Empty) => Ok(None),
        }
    }

    pub fn send_review(&self, r: ReviewRes) -> Result<(), ChatError> {
        match  self.ouput.send(Message::Review(r)) {
            Ok(_) => Ok(()),
            Err(SendError(_)) => {
                Err(ChatError { message: "channel is closed".to_string() })
            }
        }
    }

    pub fn check_review(&self) -> Result<Option<ReviewRes>, ChatError> {
        match self.input.try_recv() {
            Ok(Message::Code(_)) => {
                Err(ChatError { message: "next programmer shouldn't send his code".to_string() })
            }
            Ok(Message::Review(res)) => Ok(Some(res)),
            Err(TryRecvError::Empty) => Ok(None),
            Err(TryRecvError::Disconnected) => Ok(None),
        }
    }
}
