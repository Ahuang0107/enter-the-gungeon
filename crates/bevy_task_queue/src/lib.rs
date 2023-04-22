use bevy::prelude::*;

#[cfg(test)]
mod test;

#[derive(Eq, PartialEq, FromReflect, Reflect)]
pub enum AsyncTask {
    LoadingResource,
}

#[derive(Resource, Reflect)]
pub struct TaskQueue {
    async_task_queue: Vec<AsyncTask>,
}

impl TaskQueue {
    pub fn new() -> Self {
        Self {
            async_task_queue: vec![],
        }
    }
    pub fn push(&mut self, task: AsyncTask) {
        self.async_task_queue.push(task);
    }
    pub fn remove(&mut self, task: AsyncTask) -> bool {
        if let Some(index) = self.async_task_queue.iter().position(|t| t == &task) {
            self.async_task_queue.remove(index);
            true
        } else {
            false
        }
    }
    pub fn clear(&mut self) {
        self.async_task_queue.clear();
    }
    /// 主要就是在只有异步任务的state不断检测该队列是否为空，为空就可以跳转到下一state了
    pub fn is_empty(&self) -> bool {
        self.async_task_queue.is_empty()
    }
    pub fn len(&self) -> usize {
        self.async_task_queue.len()
    }
    pub fn contains(&self, task: AsyncTask) -> bool {
        self.async_task_queue.contains(&task)
    }
}
