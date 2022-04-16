# exiftool-rs

Simple image metadata scrubber.
Will remove EXIF, XMP and IPTC metadata.

## Packages

 - [Arch User Repository](https://aur.archlinux.org/packages/exiftool-rs-git)

## Usage

```
Jae Lo Presti
A small tool to scrub metadata from images.

USAGE:
    exiftool-rs [OPTIONS] <PATH>

OPTIONS:
    -h, --help       Print help information
    -V, --version    Print version information

SWITCHES:
    -i, --inplace    Do not change the image name

FILE:
    <PATH>    The image you want to apply the changes to
```

## Compiling

```
git clone https://github.com/jae1911/exiftool-rs
cd exiftool-rs
cargo build -r
```

The program will be then be available in `target/release/exiftool-rs`

## Benchmarks

Some benchmarks because everybody loves them.  
Everything was tested on a Ryzen 7 3700x with 32GB of ram.

`exiftool` (Perl version)
```
$ sudo chrt -f 99 perf stat -ddd exiftool -v3 -overwrite_original -all='' base.jpg 2>&1 > /dev/null 

 Performance counter stats for 'exiftool -v3 -overwrite_original -all= base.jpg':

            108.19 msec task-clock                #    0.996 CPUs utilized          
                 0      context-switches          #    0.000 /sec                   
                 0      cpu-migrations            #    0.000 /sec                   
             5,963      page-faults               #   55.115 K/sec                  
       444,730,491      cycles                    #    4.111 GHz                      (28.92%)
        18,008,995      stalled-cycles-frontend   #    4.05% frontend cycles idle     (31.69%)
        43,509,390      stalled-cycles-backend    #    9.78% backend cycles idle      (34.47%)
       604,892,502      instructions              #    1.36  insn per cycle         
                                                  #    0.07  stalled cycles per insn  (37.24%)
       133,585,903      branches                  #    1.235 G/sec                    (40.01%)
         3,552,814      branch-misses             #    2.66% of all branches          (41.59%)
       285,062,290      L1-dcache-loads           #    2.635 G/sec                    (40.58%)
        12,138,954      L1-dcache-load-misses     #    4.26% of all L1-dcache accesses  (37.81%)
   <not supported>      LLC-loads                                                   
   <not supported>      LLC-load-misses                                             
       137,952,992      L1-icache-loads           #    1.275 G/sec                    (35.03%)
         2,939,284      L1-icache-load-misses     #    2.13% of all L1-icache accesses  (32.26%)
         2,013,636      dTLB-loads                #   18.612 M/sec                    (29.49%)
           144,403      dTLB-load-misses          #    7.17% of all dTLB cache accesses  (27.73%)
           643,638      iTLB-loads                #    5.949 M/sec                    (27.73%)
             9,805      iTLB-load-misses          #    1.52% of all iTLB cache accesses  (27.73%)
        13,447,839      L1-dcache-prefetches      #  124.295 M/sec                    (27.73%)
   <not supported>      L1-dcache-prefetch-misses                                   

       0.108629192 seconds time elapsed

       0.098442000 seconds user
       0.010183000 seconds sys
```

`mat2`  
```
$ sudo chrt -f 99 perf stat -ddd mat2 --inplace -V base.jpg
DEBUG: Cleaning base.jpg…

 Performance counter stats for 'mat2 --inplace -V base.jpg':

            105.55 msec task-clock                #    0.980 CPUs utilized          
                21      context-switches          #  198.958 /sec                   
                 0      cpu-migrations            #    0.000 /sec                   
             9,414      page-faults               #   89.190 K/sec                  
       419,503,063      cycles                    #    3.974 GHz                      (43.43%)
        47,206,812      stalled-cycles-frontend   #   11.25% frontend cycles idle     (40.00%)
       120,228,136      stalled-cycles-backend    #   28.66% backend cycles idle      (35.12%)
       541,992,518      instructions              #    1.29  insn per cycle         
                                                  #    0.22  stalled cycles per insn  (33.06%)
       112,718,354      branches                  #    1.068 G/sec                    (31.96%)
         3,916,087      branch-misses             #    3.47% of all branches          (29.28%)
       260,983,861      L1-dcache-loads           #    2.473 G/sec                    (31.91%)
        14,862,648      L1-dcache-load-misses     #    5.69% of all L1-dcache accesses  (32.01%)
   <not supported>      LLC-loads                                                   
   <not supported>      LLC-load-misses                                             
       100,939,097      L1-icache-loads           #  956.317 M/sec                    (32.15%)
         1,260,732      L1-icache-load-misses     #    1.25% of all L1-icache accesses  (31.81%)
         4,296,617      dTLB-loads                #   40.707 M/sec                    (31.81%)
           282,793      dTLB-load-misses          #    6.58% of all dTLB cache accesses  (35.06%)
           496,595      iTLB-loads                #    4.705 M/sec                    (38.30%)
            20,134      iTLB-load-misses          #    4.05% of all iTLB cache accesses  (39.95%)
        10,456,775      L1-dcache-prefetches      #   99.070 M/sec                    (40.89%)
   <not supported>      L1-dcache-prefetch-misses                                   

       0.107734521 seconds time elapsed

       0.070084000 seconds user
       0.039753000 seconds sys
```

`exiftool-rs`
```
$ sudo chrt -f 99 perf stat -ddd exiftool-rs -i base.jpg   
> Found a path base.jpg, processing!


> Attempting to clean...

> EXIF data found!

> Cleared all EXIF data!

> XMP data found!

> Cleared all XMP data!

> No IPTC data found (or not supported)

> Saving modified image to "base.jpg"

 Performance counter stats for 'exiftool-rs -i base.jpg':

              5.11 msec task-clock                #    0.941 CPUs utilized          
                 0      context-switches          #    0.000 /sec                   
                 0      cpu-migrations            #    0.000 /sec                   
               649      page-faults               #  127.111 K/sec                  
        21,154,087      cycles                    #    4.143 GHz                      (18.98%)
         1,877,017      stalled-cycles-frontend   #    8.87% frontend cycles idle     (77.69%)
         6,297,158      stalled-cycles-backend    #   29.77% backend cycles idle    
        25,747,365      instructions              #    1.22  insn per cycle         
                                                  #    0.24  stalled cycles per insn
         4,911,235      branches                  #  961.898 M/sec                  
           120,902      branch-misses             #    2.46% of all branches          (81.02%)
        12,393,053      L1-dcache-loads           #    2.427 G/sec                    (22.31%)
     <not counted>      L1-dcache-load-misses                                         (0.00%)
   <not supported>      LLC-loads                                                   
   <not supported>      LLC-load-misses                                             
     <not counted>      L1-icache-loads                                               (0.00%)
     <not counted>      L1-icache-load-misses                                         (0.00%)
     <not counted>      dTLB-loads                                                    (0.00%)
     <not counted>      dTLB-load-misses                                              (0.00%)
     <not counted>      iTLB-loads                                                    (0.00%)
     <not counted>      iTLB-load-misses                                              (0.00%)
     <not counted>      L1-dcache-prefetches                                          (0.00%)
   <not supported>      L1-dcache-prefetch-misses                                   

       0.005423761 seconds time elapsed

       0.005444000 seconds user
       0.000000000 seconds sys


Some events weren't counted. Try disabling the NMI watchdog:
    echo 0 > /proc/sys/kernel/nmi_watchdog
    perf stat ...
    echo 1 > /proc/sys/kernel/nmi_watchdog
```

As you can see, `exiftool-rs` is faster than `mat2` and `exiftool` (Perl) according to this very professional benchmark.  
Sample EXIF file was taken from [ianare/exif-samples](https://github.com/ianare/exif-samples/blob/master/jpg/gps/DSCN0010.jpg).
