# Packing - Compressing table-like pack format
Tool for compressed representation of coverage information from a tabular (plain-text) pack file (e.g. VG pack).
Can either be used for reduced storage or in combination with [gfa2bin](https://github.com/MoinSebi/gfa2bin).  

**Data fromats**  
- ```pc``` pack compressed: Compressed representation of a pack file (```compress```. 
- ```pn``` pack normalized: Compressed represenation of pack file after normalization (```normalize```).
- ```pb``` pack binary: Represents presence-absence information (from ```bit``` subcommand). 
- ```pi```pack index: Index of the graph structure (```index```).  


I use .pc "pack compressed", .pi "pack index" and pb "pack binary" as suffix, but use whatever you want. Please consider the different coverage profiles in graph compared to flat references (see [here](./images/cov_dis.png)). 



___ 
## Install: 

```
git clone https://github.com/MoinSebi/packing
cd packing
cargo build --release
```
___
## Usage
### Index

Index a graph or (plain-text) pack file. Index is needed if you want to convert reconvert from pc to pack.  

``` 
./packing index -g test.gfa -o test.pi 
OR: 
./packing index -p test.pack -o test.pi 
```
### Compress
This is the command if you want to compress your plain-text coverage file. 

``` 
./packing compress -p pack.pack -o pack.pc 
```


### Conversion 
#### Bit 
Create a presence-absence file (binary, ```pb```) based on a custom threshold. Convert a (plain-text) pack file, a compressed pack (```pc```) or a normalized pack (```pn```) to a binary pack (```pt```). An index file is needed if you input other than plain-text file for the conversion. Without any additional parameters expect in and output, the threshold will be set to 1. Values which are equal or above the threshold will be set to 1, all others to 0.

#### Normalization
Create a normalized coverage file (normalize, ```pn```) based on a custom threshold. Parameters and functionality is similar to the ```bit``` subcommand expect that the output is a value-based pack file (normalized). 

**Thresholds**  
A threshold is used to perform a normalization or presence-absence conversion. The main modifications of the threshold are  
- Absolute threshold ````-a````: A plain number, which will be used as a threshold and is the highest priority.
- Method ```-m```: Dynamic computation of the threshold based on a method [mean, median, percentile]. 
- Fraction ```-f```: A relative threshold (fraction) which will be multiplied with the computed value.
- Standard deviation ```-s```: Multiplier for the standard deviation.

**Comment**  
If an absolute threshold is provided, other inputs will be ignored. If a method is provided with ```-m```, we will firstly calculate a value (mean or median) which will later modified by standard deviation and fraction.   
The standard deviation input is a scaling factor. We calculate the single standard deviation and scale it by ```-s``` input, which will be reduced by the previous calculated mean or median. The result will be scaled by the relative threshold ```-r```. Default values of standard deviation is 0.0, relative threshold is 1.0.If

**Percentile method**  
Percentile method will be used directly and is only affected by the ```-f``` parameter (f = 0.5 -> 50% percentile)

**Excluding Zeros**  
Any of these "dynamic" methods can include all entries (default: off, activate with ```--non-covered```) or only the covered entries. The coverage profile on graphs is different compared to flat references, therefore it might be useful to exclude the zeros.


**Example computation of threshold**  
Coverage is: 1, 1, 2, 8, 4, 4  
Mean: 4  
Fraction: 0.5
Standard deviation: 0
Real threshold: 2  
Normalized coverage (e.g. pc): 0, 0, 1, 4, 2, 2  
Binary version (e.g. pt): 0, 0, 1, 1, 1, 1  

**Nodes and sequence**
If convert your data your data can either be on sequence and node level, which is also stored in the header of the file. By default we use the sequence based format, but you can change it with the ```--node``` flag.



**Example** 
```
./packing bit -p test.pack -o test.pt -a 5 
./packing normlaize -p test.pack -o test.pt -a 5  

On nodes: 
./packing bit -i test.pi -c test.pc --node -o pack.out

Include zeros:   
./packing normalize -i test.pi -c test.pc --non-covered -o pack.out 
```



### Info
Information about the index or binary file.
``` 
./packing info -i test.pi 
./packing info -c test.pc
./packing info -c test.pt
```


### View
Show the compressed file in plain text. If the input is a compressed pack and a index, you receive a plain-text pack file (comparable with the original pack file). If you don't provide an index, there will be no sequence/node information, just a plain vector. 
``` 
./packing view -c test.pc -o test.pc.txt
./packing view -c test.pt -o test.pt.txt
./packing view -c test.pc -i test.pi -o test.pc.full.txt
```

### Stats
Calculate some stats of (plain-text) pack files, compressed pack or threshold packs. Returns information about mean, median, standard deviation with and without zeros. If the input is sequence level, the output also includes node-level coverage information. 
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
The header of the file is also compressed, therefore you can only read it which ```packing info``` or ```packing view```.

| Field          | Description                    | Possible values                                 | Bytes |
|----------------|--------------------------------|-------------------------------------------------|-------|
| MB             | Magic bytes                    | [35, 38]                                        | 2     |
| Sequence       | Is sequence                    | 1 (sequence), 0 (node)                          | 1     |
| PA             | DataType                       | 0 = Bit, 1 = Compress, 2 = Normalized           | 1     |
| Method         | Normalization method           | 0 (Nothing), 1 (Mean), 2(Median), 3(Percentile) | 1     |
| fraction       | Fraction                       | Float (f32)                                     | 4     |
| Std            | Standard deviation multiplier  | Float (f32)                                     | 4     |
| real_threshold | Real threshold                 | Float (f32)                                     | 4     |
| length         | Number of entries              | -                                               | 4     |
| name           | Name of the sample             | -                                               | 64    |



In total: 85 bytes

### Additional information:
- If method == Nothing but a relative real threshold was set -> Absolute method
- Compressed == 
- If you are binary, the "real" threshold is enforced: x > threshold
- If the method == Nothing but there is a threshold, it was computed by the "absolute threshold"
- Absolute threshold is always highest priority
  

## TODO
- [ ] Z-score normalization
- [ ] Robust normalization

