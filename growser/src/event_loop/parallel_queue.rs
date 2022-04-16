use super::algorithm_queue::AlgorithmQueue;

pub struct ParallelQueue {
    algorithm_queue: AlgorithmQueue,
}

impl ParallelQueue {
    pub fn new() -> Self {
        Self {
            algorithm_queue: AlgorithmQueue::new(),
        }
    }

    pub fn run(&mut self) {
        while let Some(mut step) = self.algorithm_queue.dequeue() {
            step();
        }
    }
}