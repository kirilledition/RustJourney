# Fetch PDB

CLI tool to download structure files from Protein Data Bank

# Usage

Download single file
```
fetchpdb CODE
```
Channel file to stdout
```
fetchpdb CODE > CODE.pdb
```
Download list of files
```
fetchpdb CODE1 CODE2 CODE3
```
Download list of files to specific folder
```
fetchpdb CODE1 CODE2 CODE3 --output-directory pdbdir
```
Download list of files specified in text file
```
fetchpdb list.txt

list.txt:
CODE1
CODE2
CODE3
```

Kirill Tool Kit

krll fetchpdb
download pdb

krll parqutify
csv to parquet