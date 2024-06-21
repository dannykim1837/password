## Overview

**Project Title**: Password Manager

**Project Description**: This Password Manager is a simple program built in Rust that helps users keep their passwords safe. It runs in a command-line window, where users can add, see, search for, and delete saved passwords. The program saves data in a JSON file, focusing on keeping things secure and easy to use.

**Project Goals**: The main goal of this Password Manager project is to create a secure and user-friendly program that allows users to easily manage their passwords through a simple command-line interface. By focusing on secure data handling, intuitive user interactions, and reliable performance, the project aims to provide a complete solution for password storage and retrieval while ensuring data integrity. Additionally, the program is designed to be expandable for future enhancements, such as adding encryption and improving the user interface, with a strong emphasis on learning and providing clear documentation to support ongoing maintenance and upgrades.


## Instructions for Build and Use


Steps to build and/or run the software:

1. Set Up Rust Environment
2. Navigate to the Project Directory
3. Compile the Project and Run the Project
4. Using the Password Manager
5. Update or Modify the Project and Troubleshooting

Instructions for using the software:

1. Open your terminal or command prompt, navigate to the directory where your Password Manager is located, and start the application by typing cargo run.
2. When the application starts, use the numbers on your keyboard to choose from options like Add, List, Search, Delete entries, or Quit the program.
3. To add a new entry, select 1. Add Entry, then enter the service name, username, and password as prompted, and confirm your inputs.
4. Select 2. List Entries to see all stored password entries; if the list is empty, the application will notify you.
5. Choose 3. Search, input the service name you are looking for, and the application will show you the entry if it exists.
6. Choose 4. Delete Entry, type the service name of the password you want to remove, and confirm the deletion when prompted.
7. To exit, choose 5. Quit, and the application will close after showing a goodbye message.
8. Always ensure you type the service names accurately when searching or deleting to avoid errors, and consider backing up your data regularly for safety.

## Development Environment 

To recreate the development environment, you need the following software and/or libraries with the specified versions:

* Rust
* VS Code

## Useful Websites to Learn More

I found these websites useful in developing this software:

* https://doc.rust-lang.org/rust-by-example/flow_control/loop.html
* https://docs.rs/json-writer/latest/json_writer/
* https://www.reddit.com/r/rust/comments/ezmrzs/terminal_prompt/
* https://www.youtube.com/watch?v=br3GIIQeefY

## Future Work

The following items I plan to fix, improve, and/or add to this project in the future:

* [ ] I plan to add encryption for stored passwords to enhance security and protect user data from unauthorized access.
* [ ] I intend to develop a graphical user interface (GUI) to make the application more accessible and easier to use for non-technical users.
* [ ] I will expand the application to support multiple users, allowing each user to have their own separate and secure storage for passwords.


