use crate::{ctx::Ctx, Error, Result};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Task {
    pub title: String,
    pub id: u64,
    pub is_completed: bool,
    pub user_id: u64,
}

#[derive(Clone)]
pub struct ModelController {
    tasks_store: Arc<Mutex<Vec<Option<Task>>>>,
}

impl ModelController {
    pub fn new() -> Self {
        Self {
            tasks_store: Arc::default(),
        }
    }
}

impl ModelController {
    pub async fn get_tasks(&self, ctx: Ctx) -> Result<Vec<Task>> {
        let tasks = self
            .tasks_store
            .lock()
            .map_err(|e| Error::DbError)?
            .iter()
            .filter_map(|item| item.clone())
            .filter(|task| {
                if task.user_id == ctx.user_id() {
                    return true;
                }
                false
            })
            .collect::<Vec<_>>();
        Ok(tasks)
    }

    pub async fn add_task(&mut self, ctx: Ctx, input: AddTask) -> Result<Task> {
        let mut task_store = self.tasks_store.lock().or_else(|_| Err(Error::DbError))?;
        let task_id = task_store.len() as u64;
        let task = Task {
            title: input.title,
            user_id: ctx.user_id(),
            id: task_id,
            is_completed: false,
        };
        task_store.push(Some(task.clone()));
        Ok(task)
    }

    pub async fn remove_task(&mut self, input: RemoveTask) -> Result<Task> {
        let mut task_store = self.tasks_store.lock().or_else(|_| Err(Error::DbError))?;
        let task = task_store
            .get_mut(input.task_id as usize)
            .ok_or(Error::TaskRemoveFailIdNotFound)?
            .take()
            .ok_or(Error::TaskRemoveFailTaskNotFound)?;
        Ok(task)
    }

    pub async fn update_task(&mut self, input: UpdateTask) -> Result<Task> {
        let mut task_store = self.tasks_store.lock().or_else(|_| Err(Error::DbError))?;
        let task = task_store
            .get_mut(input.task_id as usize)
            .ok_or(Error::TaskUpdateFailIdNotFound)?
            .as_mut()
            .and_then(|task| {
                task.is_completed = input.is_completed;
                Option::Some(task)
            })
            .ok_or(Error::TaskUpdateFailTaskNotFound)?
            .clone();

        Ok(task)
    }
}

#[derive(Debug, Deserialize)]
pub struct AddTask {
    title: String,
}

#[derive(Debug, Deserialize)]
pub struct RemoveTask {
    task_id: u64,
}

#[derive(Debug, Deserialize)]
pub struct UpdateTask {
    task_id: u64,
    is_completed: bool,
}
