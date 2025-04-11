# RUI-Library

This is a small library built using Macroquad to provide a much more streamlined experience making basic UIs.
The release available is a program build using RUI called TRSSH (Pronounced Trish)


## TRSSH 
A program used to SSH onto a remote server, and provide easy tools to navigate around that session, and provide uploading and downloading functionality all in one place

### Recommended Setup (If you have Rust and Cargo installed)

1) Clone the repository into your home directory in a folder called 'repos'

2) Create another folder in your home directory called 'scripts'. Then, create a file which contains:

```
#!/bin/bash

# run trssh 
trssh.exe
```

3) Navigate to your .bashrc and at the bottom you should add:

```
export PATH="$PATH:[INSTALL_PATH_HERE]/RUI-Library/target/release"
export PATH="~/scripts/:$PATH"

chmod u+x $HOME/scripts/trssh
```

4) Run `source ./.bashrc` 

This will allow you to call the command `trssh` anywhere and allow you to run TRSSH, uploading and downloading files from the directory you are in
