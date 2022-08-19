# VGPACK to compressed binary
Small helper tool for reducing storage size of vg packs (tabular output).
Can also be used as a library for reading the produced binary files. Part of [gfa2bin](https://github.com/MoinSebi/gfa2bin).  
I use .pb "pack binary", .pi "pack index" and pt "pack threshold" as suffix, but use whatever you want. Please consider the different coverage profiles in graph compared to flat references. 



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
```
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
#### Index
``` 
packing-index 0.1.0

Index a graph (gfa or VG pack)

USAGE:
    packing index [FLAGS] [OPTIONS] --output <output>

FLAGS:
    -h, --help       Print help information
    -q               No messages
    -V, --version    Print version information

OPTIONS:
    -v <verbose>        -v = DEBUG | -vv = TRACE

Input options:
    -g, --gfa <gfa>      gfa for index
    -p, --pack <pack>    pack format after alignment

Output options:
    -o, --output <output>    Output file

```
#### Info
``` 
packing-info 0.1.0

Information about index or binary files (not compressed pack)

USAGE:
    packing info [FLAGS] [OPTIONS]

FLAGS:
    -h, --help       Print help information
    -q               No messages
    -V, --version    Print version information

OPTIONS:
    -v <verbose>        -v = DEBUG | -vv = TRACE

Input options:
    -b, --binary <binary>    Information about the binary
    -i, --index <index>      Information about the index

Testing options:
    -a, --all    Check all entries (for concatenated index)
```

#### Convert
``` 
packing-convert 0.1.0

Convert VG PACK format for a compact index structure (partially reversible)

USAGE:
    packing convert [FLAGS] [OPTIONS]

FLAGS:
    -b, --binary         Make a presence-absence binary file
    -h, --help           Print help information
    -n, --normalize      Normalize everything
        --non-covered    Include non-covered entries (nodes or sequences) for dynamic normalizing
                         calculations (e.g mean)
    -q                   No messages
    -V, --version        Print version information

OPTIONS:
    -a, --absolute threshold <absolute threshold>
            Presence-absence according to absolute threshold

    -c <compressed pack (sequence)>
            

    -i, --index <index>
            Index file from 'packing index'

    -o, --out <out>
            Output name [default: pack]

    -p, --pack <pack>
            vg pack file

    -r, --threshold <relative threshold>
            Percentile (can be combined with 'normalize' flag

    -s, --stats <stats>
            Normalize by mean or median (always in combination relative threshold)

    -t, --type <type>
            Type of output: nodes|sequence|pack (default: nodes)

    -v <verbose>
            -v = DEBUG | -vv = TRACE
```
--- 
### Usage

**Reduce storage**  
Indexing
``` 
./packing index -g test.gfa -o test.pi 
OR: 
./packing index -p test.pack -o test.pi 
```
Coverage
```
./packing convert -p test.pack -o test.pb  
```

**Get pack file from index + coverage**
``` 
./packing convert -i test.pi -c test.bp -t pack -o test.pack   
```

**Presence-Absence file (-b)**    
Absolute threshold: 
```
On nodes: 
./packing convert -i test.pi -c test.bp -t node -b -a 5 -o output.pt
On sequence:  
./packing convert -i test.pi -c test.bp -b -a 5 -o output.a5.pt
```
Relative threshold: 
```
Mean
./packing convert -i test.pi -c test.bp -t node -s mean -b -r 50 -o output.mean.r50.pt
Nothing (percentile)
./packing convert -i test.pi -c test.bp -t node -b -r 50 -o output.r50.pt
```

---

## For index and presence-absence
### Magic bytes explained (in this order): 
- 2 bytes identifier
- 1 byte coverage|node byte  (1 = cov, 0 = node) 
- 4 bytes total length of the data set (u32)
- 2 bytes threshold (u16)
- 64 byte name of sample

In total: 73 bytes

### Additional information:    

**Threshold** == 0 --> 1 sample = 1 bit  
**Threshold** > 0 --> 1 sample = 2 byte (u16)        
  



