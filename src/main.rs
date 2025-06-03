use serde::{Deserialize, Serialize};
use std::fs;
use std::io;

#[derive(Serialize, Deserialize, PartialEq)]
struct Book {
    id: u32,
    title: String,
    author: String,
    is_issued: bool,
}

#[derive(Serialize, Deserialize)]
struct User {
    id: u32,
    name: String,
    borrowed_books: Vec<u32>,
}

#[derive(Serialize, Deserialize)]
struct Library {
    books: Vec<Book>,
    users: Vec<User>,
}

impl Library {
    fn new() -> Self {
        Library {
            books: Vec::new(),
            users: Vec::new(),
        }
    }

    fn load_from_file(filename: &str) -> Result<Self, String> {
        if std::path::Path::new(filename).exists() {
            let data = fs::read_to_string(filename).map_err(|e| format!("Failed to read file: {}", e))?;
            let library: Library = serde_json::from_str(&data).map_err(|e| format!("Failed to parse JSON: {}", e))?;
            Ok(library)
        } else {
            Ok(Library::new())
        }
    }

    fn save_to_file(&self, filename: &str) -> Result<(), String> {
        let data = serde_json::to_string(self).map_err(|e| format!("Failed to serialize to JSON: {}", e))?;
        fs::write(filename, data).map_err(|e| format!("Failed to write file: {}", e))?;
        Ok(())
    }

    fn add_book(&mut self, title: String, author: String) {
        let id = (self.books.len() as u32) + 1;
        println!("Book '{}' by '{}' added", title, author);
        self.books.push(Book {
            id,
            title,
            author,
            is_issued: false,
        });
    }

    fn add_user(&mut self, name: String) {
        if !self.users.iter().any(|u| u.name == name) {
            let id = (self.users.len() as u32) + 1;
            println!("User '{}' added", name);
            self.users.push(User {
                id,
                name,
                borrowed_books: Vec::new(),
            });
        } else {
            println!("Error: User '{}' already exists!", name);
        }
    }

    fn display_books(&self) {
        if self.books.is_empty() {
            println!("No books available.");
        } else {
            println!("\nLibrary Books:");
            for book in &self.books {
                let status = if book.is_issued { "Issued" } else { "Available" };
                println!(
                    "ID: {}, Title: {}, Author: {}, Status: {}",
                    book.id, book.title, book.author, status
                );
            }
        }
    }

    fn issue_book(&mut self, title: String, user: &str) {
        // Check if user exists
        if !self.users.iter().any(|u| u.name == user) {
            println!("No user found with name '{}'. Please register first!", user);
            return;
        }

        // Check if book exists and is available
        let book_exists_and_available = self.books.iter().any(|b| b.title == title && !b.is_issued);
        if !book_exists_and_available {
            println!("No available book found with title '{}'.", title);
            return;
        }

        // Find book and mark as issued
        let mut book_id = 0;
        for book in self.books.iter_mut() {
            if book.title == title && !book.is_issued {
                book.is_issued = true;
                book_id = book.id;
                break;
            }
        }

        // Update user's borrowed_books
        for user_record in self.users.iter_mut() {
            if user_record.name == user {
                user_record.borrowed_books.push(book_id);
                println!("Book '{}' issued to user '{}'", title, user);
                break;
            }
        }
    }

    fn return_book(&mut self, title: String, user: &str) {
        // Check if user exists
        if !self.users.iter().any(|u| u.name == user) {
            println!("No user found with name '{}'.", user);
            return;
        }

        // Check if book exists and is issued
        let book_id = match self.books.iter().find(|b| b.title == title && b.is_issued) {
            Some(book) => book.id,
            None => {
                println!("No issued book found with title '{}'.", title);
                return;
            }
        };

        // Check if user borrowed the book
        let user_borrowed = self.users.iter().any(|u| u.name == user && u.borrowed_books.contains(&book_id));
        if !user_borrowed {
            println!("User '{}' did not borrow book '{}'.", user, title);
            return;
        }

        // Update book status
        for book in self.books.iter_mut() {
            if book.title == title && book.is_issued {
                book.is_issued = false;
                break;
            }
        }

        // Remove book from user's borrowed_books
        for user_record in self.users.iter_mut() {
            if user_record.name == user {
                if let Some(index) = user_record.borrowed_books.iter().position(|&id| id == book_id) {
                    user_record.borrowed_books.remove(index);
                }
                println!("Book '{}' returned by user '{}'", title, user);
                break;
            }
        }
    }
}

fn main() {
    // Initialize the library
    let mut library = Library::load_from_file("library.json").unwrap_or_else(|e| {
        eprintln!("Error loading library: {}. Starting with empty library.", e);
        Library::new()
    });
    println!("Library initialized with {} books and {} users", library.books.len(), library.users.len());

    // Main menu loop
    loop {
        println!("\nLibrary Management System");
        println!("1. Add Book");
        println!("2. Add User");
        println!("3. Issue Book");
        println!("4. Return Book");
        println!("5. Display Books");
        println!("6. Exit");
        println!("Enter choice: ");

        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read input");

        let choice: u32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input! Please enter a number.");
                continue;
            }
        };

        match choice {
            1 => {
                println!("Enter book title: ");
                let mut title = String::new();
                io::stdin()
                    .read_line(&mut title)
                    .expect("Failed to read title");
                let title = title.trim().to_string();

                println!("Enter book author: ");
                let mut author = String::new();
                io::stdin()
                    .read_line(&mut author)
                    .expect("Failed to read author");
                let author = author.trim().to_string();

                if title.is_empty() || author.is_empty() {
                    println!("Error: Title and author cannot be empty!");
                } else {
                    library.add_book(title, author);
                }
            }
            2 => {
                println!("Enter user name: ");
                let mut name = String::new();
                io::stdin()
                    .read_line(&mut name)
                    .expect("Failed to read name");
                let name = name.trim().to_string();

                if name.is_empty() {
                    println!("Error: Name cannot be empty!");
                } else {
                    library.add_user(name);
                }
            }
            3 => {
                println!("Enter book title to issue: ");
                let mut title = String::new();
                io::stdin()
                    .read_line(&mut title)
                    .expect("Failed to read title");
                let title = title.trim().to_string();

                println!("Enter user name: ");
                let mut user = String::new();
                io::stdin()
                    .read_line(&mut user)
                    .expect("Failed to read user");
                let user = user.trim();

                if title.is_empty() || user.is_empty() {
                    println!("Error: Title and user name cannot be empty!");
                } else {
                    library.issue_book(title, user);
                }
            }
            4 => {
                println!("Enter book title to return: ");
                let mut title = String::new();
                io::stdin()
                    .read_line(&mut title)
                    .expect("Failed to read title");
                let title = title.trim().to_string();

                println!("Enter user name: ");
                let mut user = String::new();
                io::stdin()
                    .read_line(&mut user)
                    .expect("Failed to read user");
                let user = user.trim();

                if title.is_empty() || user.is_empty() {
                    println!("Error: Title and user name cannot be empty!");
                } else {
                    library.return_book(title, user);
                }
            }
            5 => library.display_books(),
            6 => {
                match library.save_to_file("library.json") {
                    Ok(()) => println!("Data saved to library.json"),
                    Err(e) => eprintln!("Error saving data: {}", e),
                }
                println!("Exiting...");
                break;
            }
            _ => println!("Invalid choice! Please select 1–6."),
        }
    }
}