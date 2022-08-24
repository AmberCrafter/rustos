## [Memory frame](https://stackoverflow.com/questions/7848771/how-can-one-see-content-of-stack-with-gdb)

1. info frame: to show the stack frame info

2. To read the memory at given addresses you should take a look at x, x uses the format syntax, you could also take a look at the current instruction via x/i $eip etc.
 - x/x $esp for hex 
 - x/d $esp for signed 
 - x/u $esp for unsigned etc. 

## Layout(https://blog.csdn.net/zhangjs0322/article/details/10152279)
layout：用于分割窗口，可以一边查看代码，一边测试。主要有以下几种用法：
layout src：显示源代码窗口
layout asm：显示汇编窗口
layout regs：显示源代码/汇编和寄存器窗口
layout split：显示源代码和汇编窗口
layout next：显示下一个layout
layout prev：显示上一个layout
Ctrl + L：刷新窗口
Ctrl + x，再按1：单窗口模式，显示一个窗口
Ctrl + x，再按2：双窗口模式，显示两个窗口
Ctrl + x，再按a：回到传统模式，即退出layout，回到执行layout之前的调试窗口。
————————————————
版权声明：本文为CSDN博主「zhangjs0322」的原创文章，遵循CC 4.0 BY-SA版权协议，转载请附上原文出处链接及本声明。
原文链接：https://blog.csdn.net/zhangjs0322/article/details/10152279

---
Reference
[https://stackoverflow.com/questions/7848771/how-can-one-see-content-of-stack-with-gdb](https://stackoverflow.com/questions/7848771/how-can-one-see-content-of-stack-with-gdb)

[https://blog.csdn.net/zhangjs0322/article/details/10152279](https://blog.csdn.net/zhangjs0322/article/details/10152279)
