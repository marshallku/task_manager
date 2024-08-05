use crate::data::{status::TaskStatus, task::Task};

fn draw_line(
    start_character: &str,
    middle_character: &str,
    separate_character: &str,
    end_character: &str,
    widths: &Vec<usize>,
) {
    print!("{}", start_character);
    for (i, width) in widths.iter().enumerate() {
        if i == widths.len() - 1 {
            print!("{}", middle_character.repeat(width + 2));
        } else {
            print!(
                "{}{}",
                middle_character.repeat(width + 2),
                separate_character
            );
        }
    }
    println!("{}", end_character);
}

fn status_to_string(status: TaskStatus) -> String {
    match status {
        TaskStatus::Todo => "Todo".to_string(),
        TaskStatus::InProgress => "Doing".to_string(),
        TaskStatus::Paused => "Paused".to_string(),
        TaskStatus::Done => "Done".to_string(),
    }
}

pub fn table_view(task: &Vec<Task>) {
    let headers = vec![
        "ID".to_string(),
        "Name".to_string(),
        "Status".to_string(),
        "Deadline".to_string(),
        "Priority".to_string(),
        "Estimated (h)".to_string(),
        "Time (h)".to_string(),
    ];

    assert!(headers.len() == 7);

    let cells = task.iter().map(|task| {
        vec![
            task.id.to_string(),
            task.name.clone(),
            status_to_string(task.status.clone()),
            task.deadline.to_string(),
            task.priority.clone(),
            task.estimated_hours.to_string(),
            task.time.to_string(),
        ]
    });

    // calculate width via headers and cells.
    let mut widths = headers
        .iter()
        .map(|header| header.len())
        .collect::<Vec<usize>>();

    for row in cells.clone() {
        for (i, cell) in row.iter().enumerate() {
            if cell.len() > widths[i] {
                widths[i] = cell.len();
            }
        }
    }

    draw_line("┏", "━", "┳", "┓", &widths);

    for (i, header) in headers.iter().enumerate() {
        print!("┃ {:width$} ", header, width = widths[i]);
    }
    println!("┃");

    draw_line("┣", "━", "╋", "┫", &widths);

    for (i, row) in cells.clone().enumerate() {
        for (i, cell) in row.iter().enumerate() {
            print!("┃ {:width$} ", cell, width = widths[i]);
        }
        println!("┃");

        // if last row, draw bottom line
        if i == task.len() - 1 {
            draw_line("┗", "━", "┻", "┛", &widths);
        } else {
            draw_line("┣", "━", "╋", "┫", &widths);
        }
    }
}
