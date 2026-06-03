# progLangsProject
Rust text editor

# Howw to run
1. Git clone
2. cargo run (choose a file with a mouse)

# Funcionality
- Insertion and delition of text
- ctrl + c / ctrl + v
- ctrl + z / ctrl + y
- ctrl + q - exit with save
- ctrl + p - exit without safe

Uses very data structure called Piece Table, it is very efficient when working with large texts and performing redo, undo.

# Plans
- Concurrent text editing (like Google docs). Websocket server and Stomp with RabitMQ
