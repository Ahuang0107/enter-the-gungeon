use crate::{AsyncTask, TaskQueue};

#[test]
fn check_task_queue() {
    let mut task_queue = TaskQueue::new();
    assert!(task_queue.is_empty());
    task_queue.push(AsyncTask::LoadingResource);
    task_queue.push(AsyncTask::LoadingResource);
    assert!(!task_queue.is_empty());
    task_queue.remove(AsyncTask::LoadingResource);
    assert!(!task_queue.is_empty());
    task_queue.remove(AsyncTask::LoadingResource);
    assert!(task_queue.is_empty());
}
