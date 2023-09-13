# huff-stack-generator

The goal of this project is to automatically generate the stack comments.

From this:

```huff
#define macro SET_VALUE() = takes (0) returns (0) {
    0x04 
    calldataload   
    [VALUE_LOCATION]    
    sstore              
}
```

To this:

```huff
#define macro SET_VALUE() = takes (0) returns (0) {
    0x04                // [value]
    calldataload        // [value]
    [VALUE_LOCATION]    // [ptr, value]
    sstore              // []
}
```
