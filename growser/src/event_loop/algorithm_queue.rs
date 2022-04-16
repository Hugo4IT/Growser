use std::collections::VecDeque;

pub struct AlgorithmQueue {
    steps: VecDeque<Box<dyn FnMut()>>,
}

impl AlgorithmQueue {
    pub fn new() -> Self {
        Self {
            steps: VecDeque::new(),
        }
    }

    pub fn enqueue(&mut self, step: Box<dyn FnMut()>) {
        self.steps.push_back(step);
    }

    pub fn dequeue(&mut self) -> Option<Box<dyn FnMut()>> {
        self.steps.pop_front()
    }
}