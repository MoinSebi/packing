# Packing - Compressing table-like pack format
Tool for compressed representation of coverage information from a tabular (plain-text) pack file (e.g. VG pack).
Can either be used for reduced storage or in combination with [gfa2bin](https://github.com/MoinSebi/gfa2bin).  

**Data fromats**  
- ```pc``` pack compressed: Compressed representation of a pack file. Also used for normalized coverage. 
- ```pb``` pack binary: Represents presence-absence information. 
- ```pi```pack index: Index of the graph structure.  


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

### Convert
Convert a (plain-text) pack file to a compressed pack (```pc```) or binary pack (```pt```). Alternative input is pack compressed (```pc```) file. This command can also re-convert a pack compressed (```pc```, **sequence-level**) to a plain-text pack file (see below). 
An index file is needed for all conversions from a pack-compressed file. On default, the output will be a sequence-level binary compressed pack file (```pb```). If you want to normalize the file, provide a threshold (see below) and the ```-normalize``` flag. If you only want to compress your plain-text, use the ```-c``` flag.

**Thresholds**  
A threshold is used to perform a normalization or presence-absence conversion. The main modifications of the threshold are  
- Absolute threshold ````-a````: A plain number, which will be used as a threshold and is the highest priority.
- Method ```-m```: Dynamic computation of the threshold based on a method [mean, median, percentile]. 
- Fraction ```-f```: A relative threshold (fraction) which will be multiplied with the computed value.

**Comment**  
If an absolute threshold is provided, other inputs will be ignored. If a method is provided with ```-m```, we will firstly calculate a value (mean or median) which will later scaled by the relative threshold ```-r```. If no relative threshold is provided, it will always be set to 100. Percentile method will be used directly on sorted data vector. Any of these "dynamic" methods can include all entries (default: off, activate with ```--non-covered```) or only the covered ones.


**Example computation**  
Coverage is: 1, 1, 2, 8, 4, 4  
Mean: 4  
Relative value: 50  
Real threshold: 2  
New coverage (e.g. pc): 0, 0, 1, 4, 2, 2  
Binary version (e.g. pt): 0, 0, 1, 1, 1, 1  

**Nodes and sequence**
This is not 100% intuitive and might change in the future. Your compressed or binary file can be either sequence and node based, which is also stored in the header of the file. By default we use the sequence based format, but you can change it with the ```--node``` flag.



**Example** 
```
./packing convert -p test.pack -o test.pc  
```

**Get pack file from index + coverage**
``` 
./packing convert -i test.pi -c test.pc --output-pack -o test.pack   
```

**Presence-Absence file (-b)**    
Absolute threshold:
```
On nodes: 
./packing convert -i test.pi -c test.pc -t node -b -a 5 -o output.pt
On sequence:  
./packing convert -i test.pi -c test.pc -b -a 5 -o output.a5.pt
```


### Relative threshold:
```
Mean
./packing convert -i test.pi -c test.bp -t node -s mean -b -r 50 --method percentile -o output.mean.r50.pt
Nothing (percentile)
./packing convert -i test.pi -c test.bp -t node -b -r 50 -o output.r50.pt
```




### Info
Information about the index or binary file.
``` 
./packing info -i test.pi 
./packing info -c test.pc
./packing info -c test.pt

```


### View
Show the compressed file in plain text. If you don't provide an index, there will be no sequence/node information, just plain vector. 
``` 
./packing view -c test.pc -o test.pc.txt
./packing view -c test.pt -o test.pt.txt
./packing view -c test.pc -i test.pi -o test.pc.full.txt
```

### Stats
Calculate some stats of (plain-text) pack files, compressed pack or threshold packs. Not very smart. Add index for more information (not sure). 
``` 
./packing stats -p test.pack -o test.packstats
./packing stats -c test.pc -i test.pi -o test.full.stats
./packing stats -c test.pt -o test.pt.stats
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
  



