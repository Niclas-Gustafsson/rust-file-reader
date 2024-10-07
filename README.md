# Text file reader written in rust
A CLI program that reads files and counts files read, and accumulated lines read in all those files.
When the program is done it prints the results in your terminal. Files can be ignored by adding a "nicignore.txt" to the root of any folder that should be read.

## How to
- Download the repo
- open repo in IDE of your choice and run command ```cargo build --release```
- Or you can open root of repo project in your preferred terminal and run ```cargo build --release```
- Make sure to add the file ```nicignore.txt``` in the root of the folder you want the program to read. Add any filename or folder name on separate lines. The program will ignore these files and folders.
- While in root of repo project in any terminal, run ```./target/release/file_reader.exe <path to folder you want the program to read>```
- Make sure the path to the folder you want the program to read is the first argument
