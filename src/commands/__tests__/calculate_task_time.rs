#[cfg(test)]
mod tests {
    use crate::{
        commands::functions::calculate_task_time,
        data::{
            status::TaskStatus,
            task::{Task, TaskError},
        },
    };

    use chrono::{NaiveDate, NaiveDateTime};

    #[test]
    fn test_calculate_task_time_not_started() {
        let task = Task {
            id: 1,
            name: "Test Task".to_string(),
            status: TaskStatus::Todo,
            deadline: NaiveDate::from_ymd_opt(2024, 8, 7).unwrap(),
            priority: "High".to_string(),
            time: 0.0,
            estimated_hours: 5.0,
            started_at: None,
            completed_at: None,
            paused_at: None,
        };

        let result = calculate_task_time(&task);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 5.0);
    }

    #[test]
    fn test_calculate_task_time_completed() {
        let task = Task {
            id: 2,
            name: "Completed Task".to_string(),
            status: TaskStatus::Done,
            deadline: NaiveDate::from_ymd_opt(2024, 8, 7).unwrap(),
            priority: "Medium".to_string(),
            time: 0.0,
            estimated_hours: 3.0,
            started_at: Some(
                NaiveDateTime::parse_from_str("2024-08-07 10:00:00", "%Y-%m-%d %H:%M:%S").unwrap(),
            ),
            completed_at: Some(
                NaiveDateTime::parse_from_str("2024-08-07 13:30:00", "%Y-%m-%d %H:%M:%S").unwrap(),
            ),
            paused_at: None,
        };

        let result = calculate_task_time(&task);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 3.5);
    }

    #[test]
    fn test_calculate_task_time_paused() {
        let task = Task {
            id: 3,
            name: "Paused Task".to_string(),
            status: TaskStatus::InProgress,
            deadline: NaiveDate::from_ymd_opt(2024, 8, 7).unwrap(),
            priority: "Low".to_string(),
            time: 0.0,
            estimated_hours: 4.0,
            started_at: Some(
                NaiveDateTime::parse_from_str("2024-08-07 09:00:00", "%Y-%m-%d %H:%M:%S").unwrap(),
            ),
            completed_at: None,
            paused_at: Some(
                NaiveDateTime::parse_from_str("2024-08-07 11:45:00", "%Y-%m-%d %H:%M:%S").unwrap(),
            ),
        };

        let result = calculate_task_time(&task);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 2.75);
    }

    #[test]
    fn test_calculate_task_time_no_end_time() {
        let task = Task {
            id: 4,
            name: "No End Time Task".to_string(),
            status: TaskStatus::InProgress,
            deadline: NaiveDate::from_ymd_opt(2024, 8, 7).unwrap(),
            priority: "Medium".to_string(),
            time: 0.0,
            estimated_hours: 2.0,
            started_at: Some(
                NaiveDateTime::parse_from_str("2024-08-07 14:00:00", "%Y-%m-%d %H:%M:%S").unwrap(),
            ),
            completed_at: None,
            paused_at: None,
        };

        let result = calculate_task_time(&task);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            TaskError::InvalidState("No end time".to_string()).to_string()
        );
    }
}
