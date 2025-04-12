# RUI-Library

This is a small library built using Macroquad to provide a much more streamlined experience making basic UIs.



## TRSSH 
A program used to SSH onto a remote server (currently only supporting Username/Password authentication) , and provide easy tools to navigate around that session, and provide uploading and downloading functionality all in one place

### Recommended Setup 
#### (If you have Rust and Cargo installed)

1) Clone the repository into your home directory in a folder called 'repos' and run
`cargo build -r`

3) Create another folder in your home directory called 'scripts'. Then, create a file which contains:

```
#!/bin/bash

# run trssh 
[executable name, on linux this may just be 'trssh', on windows 'trssh.exe']
```
and save it as 'trssh'

3) Navigate to your .bashrc and at the bottom you should add:

```
export PATH="$PATH:[INSTALL_PATH_HERE]/RUI-Library/target/release"
export PATH="~/scripts/:$PATH"

chmod u+x $HOME/scripts/trssh
```

4) Run `source ./.bashrc` 

#### (If you do not have Rust or Cargo installed)

1) Download a copy of the provided executable for your OS

2) Create a folder at your home directory called 'repos' and store the executable here

2) Create another folder in your home directory called 'scripts'. Then, create a file which contains:

```
#!/bin/bash

# run trssh 
[executable name, on linux this may just be 'trssh', on windows 'trssh.exe']
```
and save it as 'trssh'

3) Navigate to your .bashrc and at the bottom you should add:

```
export PATH="$PATH:~/repos"
export PATH="~/scripts/:$PATH"

chmod u+x $HOME/scripts/trssh
```

4) Run `source ./.bashrc` 

#### This will allow you to call the command `trssh` anywhere and allow you to run TRSSH, uploading and downloading files from the directory you are in
