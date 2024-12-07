# Packing - Compressing table-like pack format
This tool was developed to convert and compress coverage information from a plain-text tabular pack file (e.g. VG pack). The main goal is to reduce the storage size of the coverage information. It is well integrated into [gfa2bin](https://github.com/MoinSebi/gfa2bin) conversion tool for graph-based genome-wide associations studies (GWAS).   

### Inputs 
The general input for the ````packing```` is a coverage file in *pack* format. The format is a tab-separated file with sequence position (seq.pos), node ID (node.id), node offset (node.offset, 0-based) and coverage. An example is shown below.  
Sequence-to-graph alignments in GAF and GAM format can be converted to pack format using [VG](https://github.com/vgteam/vg) or [GAF2PACK](https://github.com/MoinSebi/gaf2pack). There are several methods to align sequences to graphs, for example [VG](https://github.com/vgteam/vg) or [GraphAligner](https://github.com/maickrau/GraphAligner). Alternatively you can align to the collection of sequences and inject [VG command](https://github.com/vgteam/vg) the linear alignments to the graph. 

#### Example (from data/example_data/9986.1k.txt)

| seq.pos | node.id   | node.offset  | coverage |
|---------|-----------|--------------|----------|
| 423     | 30        | 61           | 6        |
| 424     | 30        | 62           | 0        |
| 425     | 30        | 63           | 2        |
| 426     | 30        | 64           | 2        |
| 427     | 31        | 0            | 1        |
| 428     | 32        | 0            | 1        |
| 429     | 33        | 0            | 1        |
| 430     | 33        | 1            | 1        |
| 431     | 33        | 2            | 0        |


**Output data formats**  
- ```pc``` pack compressed: Compressed representation of a pack file (```packing compress```). 
- ```pn``` pack normalized: Compressed representation of pack file after normalization (```packing normalize```).
- ```pb``` pack binary: Represents presence-absence information (from ```packing bit``` subcommand). 
- ```pi```pack index: Index of the graph structure (```packing index```).  

**Note**  
1. The suffix is arbitrary, but helped me to distinguish between the different method outputs. 
2. Please consider that the coverage profiles with graphs is very different flat references. Many parts of the graph are not covered at all (see [here](./images/cov_dis.png)). 

___ 
## Install: 

```
git clone https://github.com/MoinSebi/packing
cd packing
cargo build --release
```
---
___
## Usage

---
### Index

Index a graph or (plain-text) pack file. Index is needed if you want to convert reconvert from pc to pack.  

``` 
./packing index -g test.gfa -o test.pi 
OR: 
./packing index -p test.pack -o test.pi 
```

---
### Compress
Compress a plain-text coverage file to "pack compressed". Mainly used to reduce the storage size of the coverage file. Maximum coverage in the resulting files is 65535, higher coverages are truncated.   

``` 
./packing compress -p pack.pack -o pack.pc 
```


### Conversion methods

### General information
Use a threshold to normalize the coverage or create a presence-absence representation.  


#### Thresholds
Absolute threshold: 
- ```-a```: A plain number, which will be used as a threshold and is the highest priority.

Dynamic threshold:   
- Method ```-m```: Dynamic computation of the threshold based on a method [mean, median, percentile].
- Fraction ```-f```: A relative threshold (fraction) which will be multiplied with the computed value.

**Computation**  
If an absolute threshold is provided, other inputs will be ignored. If a method is provided with ```-m```, we will firstly calculate the specific value (mean or median) which will then scaled by the fraction ```-f```. 

**Excluding Zeros**  
Any of these "dynamic" methods can include all entries (default: off, activate with ```--non-covered```) or only the covered entries. The coverage profile on graphs is different compared to flat references, therefore it might be useful to exclude the zeros.

**Nodes and sequence**
If convert your data can either be on sequence and node level, which is also stored in the header of the file. By default we use the sequence based format, but you can change it with the ```--node``` flag.

**Example computation of threshold**  
Coverage is: 1, 1, 2, 8, 4, 4  
Mean: 4  
Fraction: 0.5
Calculated (real) threshold: 2  
Normalized coverage (e.g. pc): 0, 0, 1, 4, 2, 2  
Binary version (e.g. pt): 0, 0, 1, 1, 1, 1

#### Default threshold
Without any additional parameters, the default is dynamic threshold with 10% percentile. Values which are equal or above the threshold will be set to 1, all others to 0.

#### Inputs
- ```pack```  Plain-text pack file
- ```pc``` Compressed pack
- ```pn```Normalized pack file
An index file is needed if you input other than plain-text file for the conversion.

---
### Bit 
Create a presence-absence file (binary, ```pb```) based on a custom threshold. 

**Example**
```
./packing bit -p test.pack -o test.pt -a 5 

On nodes: 
./packing bit -i test.pi -c test.pc --node -o pack.out

```

---
### Normalization
Create a normalized coverage file (normalize, ```pn```) based on a custom threshold. Parameters and functionality is similar to the ```bit``` subcommand expect that the output is a value-based pack file (normalized). 




**Example** 
```
./packing normlaize -p test.pack -o test.pt -a 5  

Include zeros:   
./packing normalize -i test.pi -c test.pc --non-covered -o pack.out 
```



### Info
Information about the index or binary/compressed file.
``` 
./packing info -i test.pi 
./packing info -c test.pc
./packing info -c test.pt
```


### View
Show/convert the compressed file in plain text. If the input is a compressed pack and an index (see example), you receive a plain-text pack file (comparable with the original pack file). If you don't provide an index, there will be no sequence/node information, just a plain vector. 
``` 
./packing view -c test.pc -o test.pc.txt
./packing view -c test.pt -o test.pt.txt
./packing view -c test.pc -i test.pi -o test.pc.full.txt
```

### Stats
Calculate some stats of (plain-text) pack files, compressed pack or threshold packs. Returns information about mean, median, standard deviation and if zeros were removed or not. If the input is sequence level, the output also includes node-level coverage information. 
``` 
./packing stats -p test.pack -o test.packstats
./packing stats -c test.pc -i test.pi -o test.full.stats
./packing stats -c test.pt -o test.pt.stats
```

### Compare
Compare two pack files. This function is helpful if you want to know if two normalized or presence-absence files have been processed with the same parameter sets.  
```
./packing compare --pack1 test1.pack --pack2 test2.pack
```

---

## PC - Pack Compressed - Header explained
### Magic bytes explained (in this order): 
The header of the file is also compressed (with zstd), therefore you can only read it which ```packing info``` or ```packing view```.

| Field          | Description                   | Possible values                                 | Bytes |
|----------------|-------------------------------|-------------------------------------------------|-------|
| MB             | Magic bytes                   | [35, 38]                                        | 2     |
| Sequence       | Is sequence                   | 1 (sequence), 0 (node)                          | 1     |
| Keep-zeros     | Keep-zeros                    | 1 (yes), 0 (no)                                 | 1     |
| PA             | DataType                      | 0 = Bit, 1 = Compress, 2 = Normalized           | 1     |
| Method         | Normalization method          | 0 (Nothing), 1 (Mean), 2(Median), 3(Percentile) | 1     |
| fraction       | Fraction                      | Float (f32)                                     | 4     |
| Std            | Standard deviation multiplier | Float (f32)                                     | 4     |
| real_threshold | Real threshold                | Float (f32)                                     | 4     |
| length         | Number of entries             | -                                               | 4     |
| name           | Name of the sample            | -                                               | 64    |



In total: 86 bytes

### Additional information:
- If method == Nothing but a relative real threshold was set -> Absolute method
- If you are presence/absence, the "real" threshold is enforced: x > threshold
- If the method == Nothing but there is a threshold, it was computed by the "absolute threshold"
- Absolute threshold is always highest priority
  

## TODO
- [ ] Z-score normalization
- [ ] Robust normalization

