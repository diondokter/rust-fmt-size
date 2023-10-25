# Output

## Fmt

Test of builtin fmt machinery.

|features|text|rodata|total flash|
|--------|---:|-----:|----------:|
||500|0|500|
|raw|660|4|664|
|fmt-no-args|1252|36|1288|
|fmt-u32|2328|232|2560|
|fmt-i32|2332|232|2564|
|fmt-no-args,fmt-u32|2380|244|2624|
|fmt-no-args,fmt-i32|2384|244|2628|
|fmt-u32,fmt-i32|2636|232|2868|
|raw,fmt-no-args|2864|80|2944|
|raw,fmt-u32|3948|280|4228|
|raw,fmt-i32|3952|280|4232|
|fmt-f32|22016|1976|23992|
|fmt-no-args,fmt-f32|22068|1992|24060|
|fmt-u32,fmt-f32|22812|2176|24988|
|fmt-i32,fmt-f32|22836|2176|25012|
|raw,fmt-f32|23632|2028|25660|
|raw,fmt-no-args,fmt-u32,fmt-i32,fmt-f32|24788|2228|27016|

## Ufmt

Test of the ufmt crate.

*NOTE:* The f32 implementation has many limitations.

|features|text|rodata|total flash|
|--------|---:|-----:|----------:|
||500|0|500|
|raw|660|4|664|
|ufmt-no-args|660|4|664|
|raw,ufmt-no-args|744|4|748|
|ufmt-u32|1124|0|1124|
|ufmt-i32|1156|0|1156|
|raw,ufmt-u32|1180|4|1184|
|ufmt-no-args,ufmt-u32|1180|4|1184|
|raw,ufmt-i32|1204|4|1208|
|ufmt-no-args,ufmt-i32|1204|4|1208|
|ufmt-u32,ufmt-i32|1244|0|1244|
|ufmt-f32|2588|12|2600|
|raw,ufmt-f32|2624|16|2640|
|ufmt-no-args,ufmt-f32|2624|16|2640|
|ufmt-u32,ufmt-f32|2632|12|2644|
|ufmt-i32,ufmt-f32|2644|12|2656|
|raw,ufmt-no-args,ufmt-u32,ufmt-i32,ufmt-f32|2712|16|2728|

