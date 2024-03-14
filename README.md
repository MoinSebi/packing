# Packing - Compressing table-like pack format
Tool for binary representation of coverage and presence-absence information from vb pack files.
Can either be used for reduced storage or in combination with [gfa2bin](https://github.com/MoinSebi/gfa2bin).  

**Data fromats**  
- ```pc``` pack compressed: Compressed representation of a pack file. Alternatively also a normalized pack file.
- ```pb``` pack binary: Represents presence-absence information. Header contains the threshold and the name of the sample.
- ```pi```pack index: Index of the graph structure.  


I use .pb "pack compressed", .pi "pack index" and pt "pack threshold" as suffix, but use whatever you want. Please consider the different coverage profiles in graph compared to flat references (see [here](./images/cov_dis.png)). 



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
Convert a (plain-text) pack file to a compressed pack or vice versa. This command can also result in pack threshold (e.g. presence-absence).  
Either for normalization or presence-absence, you need to provide a compressed file + index OR a (plain-text) pack file. Dependent on output (default: compressed, can be changed when using ```-b```), the threshold provided will be used for normalization or presence-absence. 

**Thresholds**  
If an absolute threshold is provided, other inputs will be ignored. If a method is provided ```-m```, we will firstly calculate a value (mean or median) which will later scaled by the relative threshold ```-r```. If no relative threshold is provided, it will always be set to 100. Percentile method will be used directly on sorted data vector. Any of these "dynamic" methods can include all entries (```--non-covered```) or only the covered ones.

**Example computation**  
Coverage is: 1, 1, 2, 8, 4, 4  
Mean: 4  
Relative value: 50  
Real threshold: 2  
New coverage (e.g. pc): 0, 0, 1, 4, 2, 2  
Binary version (e.g. pt): 0, 0, 1, 1, 1, 1  


**Example** 
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
  



