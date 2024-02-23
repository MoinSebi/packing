# Packing - Compressing table-like pack format
Tool for binary representation of coverage and presence-absence information from vb pack files.
Can either be used for reduced storage or in combination with [gfa2bin](https://github.com/MoinSebi/gfa2bin).  

**Data fromats**  
- ```pc``` pack compressed: Compressed representation of a pack file. Alternatively also a normalized pack file.
- ```pb``` pack binary: Represents presence-absence information. Header contains the threshold and the name of the sample.
- ```pi```pack index: Index of the graph structure.  


I use .pb "pack binary", .pi "pack index" and pt "pack threshold" as suffix, but use whatever you want. Please consider the different coverage profiles in graph compared to flat references (see [here](./images/cov_dis.png)). 



___ 
### Install: 

```
git clone https://github.com/MoinSebi/packing
cd packing
```
___
### Usage
#### subcommands
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

Index a graph or pack file. Index is needed if you want to convert your pb to pack (tabular) later. 
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
Information about the index or binary file.
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
Convert a pack file (tabular) to a binary file or vice versa. This command can also perform binary operations (e.g. presence-absence).
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
./packing convert -p test.pack -o test.pc  
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
- 1 byte presence-absence byte (1 = pa, 0 = value)
- 1 byte type for normalization (u16)
- 2 byte relative threshold (u16)
- 2 bytes threshold (u16)
- 4 bytes Number of entries (u32)
- 64 byte name of sample

In total: 77 bytes

### Additional information:
- Nothing was changed (in comparison to the input), if Method == Nothing and real threshold == 0
- If you are binary, there must be a real threshold
- If the method == Nothing but there is a threshold, it was computed by a absolute threshold
  



