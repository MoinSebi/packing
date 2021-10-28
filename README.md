# VGPACK to binary
Small helper tool for reducing storage size of vg packs. Will maybe just use GAF later.
Can also be used as a library for reading the binary files










---

**Binary** explained: 
- 2 byte identifier
- 1 magic byte (cov (1) or node (0))
- 4 byte total length of the data set 
- 2 byte threshold 
- 64 byte name of sample

--------------------
--> Header is 73 bytes 

- data
    -   if threshold == 0: 
        - 1 sample = 1 bit (bool)  
    -  else:  
        - 1 sample = 2 byte (u16)
        
        
- Comment:  

