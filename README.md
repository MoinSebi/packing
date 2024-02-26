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
cargo build --release
```
___
### Usage
#### Index

Index a graph or pack file. Index is needed if you want to convert reconvert from pc to pack.  
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
    -g, --gfa <gfa>      Graphical Fragment Assembly file
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
    -c, --compressed <pack compressed>    Information about the binary
    -i, --index <index>                   Information about the index
```

#### Convert
Convert a pack file (tabular) to a binary file or vice versa. This command can also perform binary operations (e.g. presence-absence).
```
packing-convert 0.1.0

Convert VG PACK format for a compact index structure (partially reversible)

USAGE:
    packing convert [FLAGS] [OPTIONS]

FLAGS:
    -h, --help       Print help information
    -q               No messages
    -V, --version    Print version information

OPTIONS:
    -v <verbose>        -v = DEBUG | -vv = TRACE

Input options:
    -c, --compressed <pack compressed>    Compressed pack file.
    -i, --index <index>                   Index file from 'packing index'
    -p, --pack <pack>                     vg pack file

Normalization parameters:
    -a, --absolute threshold <absolute threshold>
            Presence-absence according to absolute threshold

    -b, --binary
            Make a presence-absence file

    -n, --name <name>
            Name of the sample [default: name of the file]

        --non-covered
            Include non-covered entries (nodes or sequences) for dynamic normalizing calculations
            (e.g mean)

        --normalize
            Normalize the data set (and return a value based pack)

    -r, --threshold <relative threshold>
            Percentile (can be combined with 'normalize' flag

    -s, --stats <stats>
            Normalization method (mean|median|percentile|nothing) [default: nothing]

Output options:
        --nc             Non-compressed output
    -o, --out <out>      Output name [default: pack]
    -t, --type <type>    Type of output: node|sequence|pack [default: sequence]

```

#### View
Show the compressed file in plain text.
``` 
packing-view 0.1.0

Shows the compressed binary data in plain text

USAGE:
    packing view [FLAGS] [OPTIONS] --compressed <pack compressed>

FLAGS:
    -h, --help       Print help information
    -q               No messages
    -V, --version    Print version information

OPTIONS:
    -c, --compressed <pack compressed>    compressed pack file
    -i, --index <index>                   Index file
    -o, --output <output>                 Output file name
    -v <verbose>                          -v = DEBUG | -vv = TRACE
```


#### Stats
Calculate some stats. 
``` 
packing-stats 0.1.0

Statistics on pack files

USAGE:
    packing stats [FLAGS] [OPTIONS]

FLAGS:
    -h, --help       Print help information
    -q               No messages
    -V, --version    Print version information

OPTIONS:
    -v <verbose>        -v = DEBUG | -vv = TRACE

Input options:
    -c, --compressed <pack compressed>
            Compressed pack file. Original can only be accessed if the file is not normalized.

    -i, --index <index>
            Index file from 'packing index'

    -o, --output <output>
            Output file name

    -p, --pack <pack>
            vg pack file

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
./packing convert -i test.pi -c test.pc -t pack -o test.pack   
```

**Presence-Absence file (-b)**    
Absolute threshold: 
```
On nodes: 
./packing convert -i test.pi -c test.pc -t node -b -a 5 -o output.pt
On sequence:  
./packing convert -i test.pi -c test.pc -b -a 5 -o output.a5.pt
```
Relative threshold: 
```
Mean
./packing convert -i test.pi -c test.bp -t node -s mean -b -r 50 --method percentile -o output.mean.r50.pt
Nothing (percentile)
./packing convert -i test.pi -c test.bp -t node -b -r 50 -o output.r50.pt
```

---

## PC - Pack Compressed - Header explained
### Magic bytes explained (in this order): 
The header of the file is also compressed, therefore you can only read it which ```packing info``` or ```packing view```.

| Field              | Description          | Possible values                                 | Type |
|--------------------|----------------------|-------------------------------------------------|------|
| MB                 | Magic bytes          | [35, 38]                                        | u16  |
| Sequence           | Is sequence          | 1 (sequence), 0 (node)                          | u8   |
| PA                 | Presence-absence     | 1 (PA), 0 (Value-based)                         | u8   |
| NM                 | Normalization method | 0 (Nothing), 1 (Mean), 2(Median), 3(Percentile) | u8   |
| relative_threshold | Relative threshold   | -                                               | u16  |
| real_threshold     | Real threshold       | -                                               | u16  |
| Length             | Number of entries    | -                                               | u32  |
| Name               | Name of the sample   | -                                               | u64  |



In total: 77 bytes

### Additional information:
- If Method == Nothing and real threshold == 0 -> No changes to the input file, used for reduced storage
- If you are binary, the "real" threshold is enforced: x > threshold
- If the method == Nothing but there is a threshold, it was computed by the "absolute threshold"
- Absolute threshold is always highest priority
  



