# Task Manager

![Preview Image](https://github.com/marshall-ku/assets/assets/72745119/6aee0c1d-be5c-4dea-9979-3a7a0e4066a5)

Task Manager is a simple and efficient CLI-based task manager designed to help you keep track of your tasks and improve productivity. With Task Manager, you can easily add, list, update, and delete tasks, all from the command line.

## Features

- **Add Tasks**: Quickly add new tasks with details such as name, status, deadline, priority, and estimated hours.
- **List Tasks**: View all your tasks in a structured format.
- **Update Tasks**: Modify the status and time spent on existing tasks.
- **Delete Tasks**: Remove tasks that are no longer needed.

## Installation

To install Task Manager, you'll need to have Rust installed on your system. If you haven't installed Rust yet, you can do so by following the instructions [here](https://www.rust-lang.org/tools/install).

Clone the repository and navigate into the project directory:

```sh
git clone https://github.com/marshallku/task_manager.git
cd task task_manager
```

Build the project:

```sh
cargo build --release
```

### Adding to PATH

#### Unix-based Systems (Linux, macOS)

Add the binary to your PATH by adding the following line to your shell configuration file (e.g., `~/.bashrc`, `~/.zshrc`):

```sh
export PATH=$PATH:/path/to/your/project/target/release
```

Apply the changes:

```sh
source ~/.bashrc  # or source ~/.zshrc
```

Alternatively, you can create an alias for the command by adding the following line to your shell configuration file:

```sh
alias ts='/path/to/your/project/target/release/task_manager'
```

Apply the changes:

```sh
source ~/.bashrc  # or source ~/.zshrc
```

#### Windows

Add the binary to your PATH:

- Open the Start Search, type in "env", and select "Edit the system environment variables".
- In the System Properties window, click on the "Environment Variables" button.
- In the Environment Variables window, find the "Path" variable in the "System variables" section and select it. Click on "Edit".
- Click on "New" and add the path to your binary, for example: `C:\path\to\your\project\target\release`.
- Click "OK" to close all the windows.

Alternatively, you can create a batch file named `ts.bat` with the following content and place it in a directory that is already in your PATH:

```bat
@echo off
C:\path\to\your\project\target\release\task_manager.exe %*
```

## Usage

Task Manager provides several commands to manage your tasks:

### Add a Task

```sh
tm add "Task Name" "Status" "YYYY-MM-DD" "Priority" EstimatedHours
```

Example:

```sh
tm add "Write README" "To do" "2024-06-30" "High" 2.0
```

### List Tasks

```sh
tm list
```

### Update a Task

```sh
tm update TaskID "Status" TimeSpent
```

Example:

```sh
tm update 1 "Doing" 1.5
```

### Delete a Task

```sh
tm delete TaskID
```

Example:

```sh
tm delete 1
```

## Task Attributes

- **ID**: Auto-generated unique identifier for each task.
- **Name**: The name or title of the task.
- **Status**: Current status of the task (Done, Doing, To do).
- **Deadline**: The due date for the task in YYYY-MM-DD format.
- **Priority**: Priority level (High, Medium, Low).
- **Time**: Time spent on the task.
- **Estimated Hours**: Estimated time required to complete the task.

## Contributing

Contributions are welcome! If you'd like to contribute to Task Manager, please fork the repository and create a pull request with your changes. Be sure to include detailed commit messages and follow the existing coding style.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.
