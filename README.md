# CutLink

CutLink is a URL shortening service written in Rust. This application allows users to shorten long URLs and manage these shortened links through various functionalities, including listing, searching, and deleting links.

## Table of Contents

- [Explanation](#explanation)
- [Installation](#installation)
- [Usage](#usage)
- [Contributing](#contributing)

## Explanation

- **Installation**: Instructions on how to clone the repository, build, and run the project.
- **Usage**: How to use the various commands provided by the application.
- **Contributing**: Guidelines for contributing to the project.

## Installation

To get started with CutLink, you'll need to have Rust installed on your machine. If you don't have Rust installed, you can download it from [rust-lang.org](https://www.rust-lang.org/).

Clone the repository:

```git
git clone https://github.com/yourusername/cutlink.git
cd cutlink
```

Build the project:

```shell
cargo build
```

Run the project:

```shell
cargo run <arg1> <arg2> <arg3>
```

Build version:

To generate the build folder for the global use of this project in your system, you need to generate the `release` version using the given below command:
```shell
cargo run build --release
```

This will generate a `release` folder in your `target` folder. Copy the path of `release` folder and set it as an `environment` variable.

Now you can use it globally and instead of writing `cargo run` everytime, you can simply use `cutlink` instead of `cargo run` following the arguments.

## Usage

CutLink provides several commands to manage your links. Below are the commands available:

### **Convert Links:**

To convert links use the following command:
```shell
cargo run cvert <original_link>
```

### **Show Links:**
    
To  display the list of all links use the following command:
```shell
cargo run show all
```

### **Delete Link:**
To delete a particular link use the following command:
```shell
cargo run delete <id>
```
   
### **Clear table:**
To clear the whole table at once, use the following command:
```shell
cargo run clear
```
   
### **Search:**
To search a particular link in a table based on `original link` or `shortened link`, use the following commands:

- To search using `original link`:
    ```shell
      cargo run search -ol <link>
    ```
  
- To search using `shortened link`:
    ```shell
      cargo run search -sl <link>
    ```
  
### **List:**
To search using specific characters, use the following command:

- To search using `original link`:
    ```shell
      cargo run list -ol <link>
    ```

- To search using `shortened link`:
    ```shell
      cargo run list -sl <link>
    ```
  
### **Help Center:**
You can also access help center using the following:
```shell
cargo run help
```

* This help center contains the list of all the commands and how to use it in the terminal.

## Contributing

Contributions are welcome! Please follow these steps to contribute:

1. Fork the repository.
2. Create a new branch `git checkout -b feature-branch`.
3. Commit your changes `git commit -m 'Add new feature'`.
4. Push to the branch `git push origin feature-branch`.
5. Open a pull request.

---

You can adjust the details to match your project's specifics, such as repository URL and any additional commands or configurations you have.