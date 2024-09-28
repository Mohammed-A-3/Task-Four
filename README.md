# Task Four

In this task, we will create two programs in Kali Linux. The first one is a Rust program that prints "Hello Mohammed A, right now the time is <current time>". The second is a PHP file that outputs the same message. This document will guide you through the process of installing the required software and writing the programs.

## Rust Program

### Step 1: Install Rust in Kali Linux
1. Open Terminal
2. Update Package List: Run the following command to update our package list:
```
sudo apt update
```
3. Install Rust using rustup: Run the following command to install rustup.
```
sudo apt install rustup
```
4. Verify Installation: Check if Rust is installed correctly by running:
```
rustc --version
```
### Step 2: Create a New Rust Project using cargo.
1. 
```
cargo new Rust
cd Rust
```
### Step 3: Edit the Main Rust File
1. Open the Main File: Open src/main.rs in our preferred text editor.
```
nano src/main.rs
```
2. Write the Rust Code: Replace the contents of src/main.rs with the following code: [main.rs](https://github.com/Mohammed-A-3/Task-Four/blob/main/main.rs)
```
use chrono::Local;

fn main() {
    // Get the current local time
    let current_time = Local::now();

    // Print the message
    println!("Hello Mohammed A, right now the time is {}", current_time.format("%Y-%m-%d %H:%M:%S"));
}
```
### Step 4: Add the Chrono Dependency
1.Open Cargo.toml: This file is located in the root of our project directory. Open it with:
```
nano Cargo.toml
```
2. Add the Chrono Dependency: Under the [dependencies] section, add the following line:
```
chrono = "0.4"
```
### Step 5: Run Our Program
1. Run the Program: We can run the program with:
```
cargo run
```
### Final Output [Rust](https://github.com/Mohammed-A-3/Task-Four/blob/main/Rust.png)
``
Hello Mohammed A, right now the time is 2024-09-27 16:41:24
``

## php Program

### Step 1: Install PHP
1. Open Terminal

2. Update Package List: Run the following command to update our package list:

```
sudo apt update
```
3. Install PHP: Run the following command to install PHP and the Apache web server:
```
sudo apt install php
```
4. Verify PHP Installation: Check if PHP is installed correctly by running:

```
php -v
```
### Step 2: Create a PHP File
1. Create a New PHP File: Use a text editor to create a new PHP file.
```
nano File.php
```
3. Write the PHP Code: Add the following code to the hello.php file: [File.php](https://github.com/Mohammed-A-3/Task-Four/blob/main/File.php)

```
<?php
	date_default_timezone_set('Asia/Kolkata'); 
	$current_time = date('Y-m-d H:i:s');
	echo "Hello Mohammed A, right now the time is $current_time";
?>
```
Note: Replace Our/Timezone with your actual timezone (e.g., Asia/Kolkata, America/New_York). We can find the list of supported timezones here.
4. Save and Exit:
- If you're using nano, press CTRL + S, then CTRL + X.
### Step 3: Run Our Program
1. We can run the program with:
```
php hello.php
```
### Final Output [PHP](https://github.com/Mubeena777/taskfour/blob/main/PHP.png)
``
Hello Mohammed A, right now the time is 2024-09-27 15:30:45
``
