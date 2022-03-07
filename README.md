# VGPACK to binary
Small helper tool for reducing storage size of vg packs (tabular output).
Can also be used as a library for reading the binary files. 


___ 
### Install: 

```
git clone https://github.com/MoinSebi/packing
cd packing
cargo build --release

```
___
### Usage
#### General
```asm
panSV 0.1.0

Sebastian V

packing

USAGE:
    packing [SUBCOMMAND]

FLAGS:
    -h, --help       Print help information
    -V, --version    Print version information

SUBCOMMANDS:
    convert    
    help       Print this message or the help of the given subcommand(s)
    index    

```
#### Index
```asm 
packing-index 0.1.0

USAGE:
    packing index [OPTIONS] --output <output>

FLAGS:
    -h, --help       Print help information
    -V, --version    Print version information

OPTIONS:
    -g, --gfa <gfa>          gfa for index
    -o, --output <output>    Output file
    -p, --pack <pack>        pack format after alignment

```

#### Convert

--- 
### Usage

**Reduce storage**
```
./packing index -g test.gfa -o out.indexless
Alterantive: 
./packing index -p test.pack -o out.indexless

./packing convert -p test.pack --outcov test
  
```


```
./packing index -g test.gfa -o out.indexless
Alterantive: 
./packing index -p test.pack -o out.indexless

./packing convert -p test.pack --outcov test
  
```

---

Smart **HEADER** explained: 
- 2 byte identifier
- 1 magic byte (cov (1) or node (0))
- 4 byte total length of the data set 
- 2 byte threshold 
- 64 byte name of sample

In total: 73 bytes

Additional information:    
  
- if threshold == 0:  
    -  1 sample = 1 bit (bool)  
-  else:  
    - 1 sample = 2 byte (u16)
        
        
**Comments**:
  



