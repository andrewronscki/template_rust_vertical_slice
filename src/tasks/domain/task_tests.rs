use utoipa::ToSchema;

use super::task::{Status, Task};

#[test]
fn status_as_str() {
    assert_eq!(Status::Open.as_str(), "OPEN");
    assert_eq!(Status::Doing.as_str(), "DOING");
    assert_eq!(Status::Done.as_str(), "DONE");
}

#[test]
fn status_from_str() {
    assert_eq!(Status::from_str("OPEN"), Some(Status::Open));
    assert_eq!(Status::from_str("DOING"), Some(Status::Doing));
    assert_eq!(Status::from_str("DONE"), Some(Status::Done));
    assert_eq!(Status::from_str("UNKNOWN"), None);
}

#[test]
fn create_new_task() {
    let title = String::from("New Task");
    let description = String::from("This is a new task.");
    let task = Task::new(title.clone(), description.clone());

    assert_eq!(task.title, title);
    assert_eq!(task.description, description);
    assert!(task.id.is_none());
    assert!(!task.removed);
    assert_eq!(task.status, Status::Open);
}

#[test]
fn set_task_id() {
    let mut task = Task::new(String::from("Task"), String::from("Description"));
    assert!(task.id.is_none());

    task.set_id(10);
    assert_eq!(task.id, Some(10));
}

#[test]
fn task_clone() {
    let original = Task::new(String::from("Task Title"), String::from("Task Description"));
    let clone = original.clone();
    assert_eq!(original, clone);
    assert!(!std::ptr::eq(&original, &clone));
}

#[test]
fn task_debug() {
    let task = Task::new(String::from("Task Title"), String::from("Task Description"));
    let task_debug = format!("{:?}", task);
    assert!(task_debug.contains("Task Title"));
    assert!(task_debug.contains("Task Description"));
}

#[test]
fn task_serde() {
    let task = Task::new(String::from("Task Title"), String::from("Task Description"));
    let serialized = serde_json::to_string(&task).expect("Failed to serialize");
    let deserialized: Task = serde_json::from_str(&serialized).expect("Failed to deserialize");
    assert_eq!(task, deserialized);
}

#[test]
fn task_to_schema() {
    Task::schema();
}

#[test]
fn status_to_schema() {
    Status::schema();
}
