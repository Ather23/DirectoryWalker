# DirectoryWalker
Walks the directory path and prints the folder structure. The output can be used in READMEs

Run the following command after cloning:
```
cargo run -- --dir-path {YourPathToFolder}
```

Sample output
```
.
├─./testf2
├────./testf3
├─────── testf3_file1
├───────./testf4
├──────────./testf5
```

### TODO:
Add an ignore list