# cli-todo
Basic todo application that lets you keep track of what you need to complete.
  
# Storage
  Will create, read, and write to file: `<username>_tasks.json` where `<username>` is chosen on start
  
# Commands
  - Add: Add a task to your todos
  - Update: Update a task's title by its position in the list
  - Remove: Remove a task by its position in the list
  - Clear: Remove ALL tasks
  - Complete: Complete a task by its position in the lis. Can uncomplete a task by invoking on a completed task
  - Undo: Undo a given amount of commands. Does NOT reset the cache after an undo, so you can undo your undos
  - Help: Provides a list of all commands with an example use case
  - Quit: Quits the app and saves to file
  
# Furture commands
  - Swap: 'swap 1 2' moves task 1 into position 2 and task 2 into position 1 
  - Search: 'search banana' prints task position
  - Move: 'move 4 2' moves task 4 to position 2, bumping task 2 to 3, 3 to 4, etc

# Crates added
  - `Colored` - change the color of the terminal text and background
  - `serde` - serialize and deserialize structs
  - `serde_json` - read/write to file
