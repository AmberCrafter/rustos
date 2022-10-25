frame (4G)
0x0000 0000 ---> Kernel start
|
|
|           ---> Kernel end 
0x0800 0000 ---> User start
|
|
|
|
0xffff ffff ---> User end


page
0x0000 0000 0000 0000 ---> bootloader page start
|
|                     ---> bootloader page end
0x0000 0000 0050 0000 
|
|
0x0000 0000 0070 0000 ---> bootloader kerenl stack botton
|                     ---> bootloader kernel stack top
0x0000 0000 007a 0000 
|
0x0000 0000 0080 0000 ---> process kerenl stack botton
|
|                     ---> process kernel stack top
0x0000 0000 0090 0000 
|
0x0000 4000 0000 0000 ---> Offset Table start (physical memory)
|
|                     ---> Offset Table end   (physical memory)
0x0000 4000 ffff ffff
|
|
0x0000 4444 4444 0000 ---> kernel heap start
|
|                     ---> kernel heap end
0x0000 4444 4445 9000 
