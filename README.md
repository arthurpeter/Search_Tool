# Search Tool

## Description
  This tool allows you to search through your file system based on a specific pattern
  and displays the path to all the files that you have based on that pattern.
  
## How to install
  In the main directory follow theese commands:
  cargo build --rebase
  cargo install --path .

## How to use
  To use this tool just type:
  search [Options]... [Pattern]
  
  ### For example:
  If you want to search for a file called Passwords.txt just navigate
  to a parent directory of that file and call:
  search Passwords.txt or search Pass or
  any other strings contained in that file's name.
  Alternatively you could use the option -d to specify
  a child directory from which you would like to start the search from,
  or --directory-path to specify the full or a relative path from which to
  start searching.
