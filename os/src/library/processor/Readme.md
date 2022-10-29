# Concept

1. Processor 抽象 CPU
 - current 表示執行項目
 - idle..  表示當前項目之context addr, pagetable 或是 init_proc context addr, pagetable (當 current process 為非 initproc 時)
> 利用 ProcessManager 管理需要執行之項目對列
2. context switch
 - 取得下一個任務 process
 - 提取 process context 地址，(rust 利用 args struct 特性解析資料結構
 > process context 保存在 process kernel stack 之最上層區塊 <TrapFrame>
 - 提取 process mmap (memory_set)
 - 更改 process 狀態 > Running
 - 更新 Processor.current
 - 更新 Cr3 之 PageTable
 - context switch
    - dest: idle_process_context_ptr
    - src:  process.inner.process_context_ptr
 > 此設計架構只有當 process 放棄CPU時，才會執行下一個任務