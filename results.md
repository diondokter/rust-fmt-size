# Output

## Fmt-comparison

### Fmt

Test of builtin fmt machinery.

|features|text|rodata|total flash|
|--------|---:|-----:|----------:|
||320|0|320|
|raw|376|4|380|
|fmt-no-args|924|36|960|
|raw,fmt-no-args|940|36|976|
|fmt-u32|1832|232|2064|
|raw,fmt-u32|1852|236|2088|
|fmt-i32|1872|232|2104|
|fmt-no-args,fmt-u32|1868|244|2112|
|raw,fmt-i32|1892|236|2128|
|fmt-no-args,fmt-i32|1908|244|2152|
|fmt-u32,fmt-i32|1936|232|2168|
|fmt-f32|9480|1700|11180|
|raw,fmt-f32|9500|1708|11208|
|fmt-no-args,fmt-f32|9516|1716|11232|
|fmt-u32,fmt-f32|10144|1900|12044|
|fmt-i32,fmt-f32|10180|1900|12080|
|raw,fmt-no-args,fmt-u32,fmt-i32,fmt-f32|10300|1916|12216|

### Ufmt

Test of the ufmt crate.

*NOTE:* The f32 implementation has many limitations.

|features|text|rodata|total flash|
|--------|---:|-----:|----------:|
||320|0|320|
|raw|376|4|380|
|ufmt-no-args|376|4|380|
|raw,ufmt-no-args|428|4|432|
|ufmt-u32|616|0|616|
|ufmt-i32|648|0|648|
|raw,ufmt-u32|660|4|664|
|ufmt-no-args,ufmt-u32|660|4|664|
|raw,ufmt-i32|684|4|688|
|ufmt-no-args,ufmt-i32|684|4|688|
|ufmt-u32,ufmt-i32|728|0|728|
|ufmt-f32|2144|12|2156|
|raw,ufmt-f32|2164|16|2180|
|ufmt-no-args,ufmt-f32|2164|16|2180|
|ufmt-u32,ufmt-f32|2172|12|2184|
|ufmt-i32,ufmt-f32|2196|12|2208|
|raw,ufmt-no-args,ufmt-u32,ufmt-i32,ufmt-f32|2260|16|2276|

## Dyn-comparison

See that `raw` is similar to `ufmt` and `dyn` is similar to `fmt` of the fmt-comparison test.

|features|text|rodata|total flash|
|--------|---:|-----:|----------:|
||312|0|312|
|raw-str|388|4|392|
|raw-u32|744|0|744|
|raw-str,raw-u32|788|4|792|
|dyn-str|912|28|940|
|raw-str,dyn-str|932|28|960|
|dyn-u32|1292|24|1316|
|raw-str,dyn-u32|1316|28|1344|
|dyn-str,raw-u32|1316|28|1344|
|dyn-str,dyn-u32|1328|28|1356|
|raw-u32,dyn-u32|1416|24|1440|
|raw-str,dyn-str,raw-u32,dyn-u32|1480|28|1508|

