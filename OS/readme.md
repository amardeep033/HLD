# Operating System Cheat Sheet — HLD

## 1. Linux Commands

### 1.1 Files And Directories

| Command | Use |
|---|---|
| `pwd` | Current directory |
| `ls -lah` | List files with size/hidden files |
| `cd <dir>` | Change directory |
| `mkdir -p <dir>` | Create directory path |
| `touch <file>` | Create empty file/update timestamp |
| `cp src dst` | Copy file/directory |
| `mv src dst` | Move/rename |
| `rm <file>` | Delete file |
| `rm -r <dir>` | Delete directory recursively |
| `find . -name "*.log"` | Find files |

### 1.2 File Viewing And Text Search

| Command | Use |
|---|---|
| `cat file` | Print whole file |
| `less file` | Page through file |
| `head -n 20 file` | First 20 lines |
| `tail -f app.log` | Follow log file |
| `grep -n "err" file` | Search text with line number |
| `rg "pattern"` | Fast recursive search |
| `wc -l file` | Line count |
| `sort file` | Sort lines |
| `uniq -c` | Count adjacent duplicates |

### 1.3 Process And System Commands

| Command | Use |
|---|---|
| `ps aux` | List processes |
| `top` / `htop` | Live process view |
| `kill <pid>` | Send SIGTERM |
| `kill -9 <pid>` | Force kill with SIGKILL |
| `jobs` | Shell background jobs |
| `bg` / `fg` | Resume job in background/foreground |
| `nohup cmd &` | Run after shell exits |
| `systemctl status svc` | Check systemd service |
| `journalctl -u svc -f` | Follow service logs |

### 1.4 Networking Commands

| Command | Use |
|---|---|
| `curl URL` | HTTP request |
| `ping host` | Basic reachability |
| `ss -tulpen` | Listening sockets/processes |
| `lsof -i :8080` | Process using port |
| `dig domain` | DNS lookup |
| `traceroute host` | Network path |
| `ip addr` | IP addresses |
| `ip route` | Routing table |

### 1.5 Permissions

| Command / Concept | Use |
|---|---|
| `chmod 755 file` | Change permissions |
| `chown user:group file` | Change owner/group |
| `rwx` | Read/write/execute |
| `u/g/o` | User/group/others |
| `sudo` | Run as privileged user |
| `umask` | Default permission mask |

## 2. Process And Scheduler

### 2.1 Process Basics

| Term | Meaning |
|---|---|
| Program | Executable file on disk |
| Process | Running program with memory/resources |
| Thread | Execution unit inside process |
| PID | Process identifier |
| PPID | Parent process identifier |
| PCB | Process Control Block metadata |
| Context switch | CPU switches from one process/thread to another |
| User mode | Restricted app execution |
| Kernel mode | Privileged OS execution |

### 2.2 Process States

| State | Meaning |
|---|---|
| New | Being created |
| Ready | Waiting for CPU |
| Running | Executing on CPU |
| Waiting/Blocked | Waiting for I/O/event/lock |
| Terminated | Finished |
| Zombie | Exited but parent has not reaped status |
| Orphan | Parent exited; adopted by init/systemd |

### 2.3 Process vs Thread

| Area | Process | Thread |
|---|---|---|
| Memory | Separate address space | Shared process address space |
| Creation | Heavier | Lighter |
| Communication | IPC needed | Shared memory possible |
| Failure isolation | Better | Worse |
| Context switch | More expensive | Less expensive |

### 2.4 Scheduling Algorithms

| Algorithm | Idea | Trade-off |
|---|---|---|
| FCFS | First come first served | Convoy effect |
| SJF | Shortest job first | Needs burst prediction |
| SRTF | Preemptive SJF | More context switches |
| Round Robin | Time quantum per process | Good responsiveness |
| Priority | Highest priority first | Starvation risk |
| Multilevel Queue | Separate queues by class | Inflexible between queues |
| Multilevel Feedback Queue | Move jobs between queues | Complex but practical |
| CFS | Linux fair scheduling | Fair CPU sharing |

### 2.5 Scheduler Terms

| Term | Meaning |
|---|---|
| Preemption | OS can interrupt running task |
| Time quantum | CPU slice duration |
| Throughput | Jobs completed per time |
| Turnaround time | Completion time - arrival time |
| Waiting time | Time spent ready but not running |
| Response time | First run time - arrival time |
| Starvation | Task waits indefinitely |
| Aging | Increase priority over time |

## 3. File System

### 3.1 File System Concepts

| Term | Meaning |
|---|---|
| File | Named data object |
| Directory | Mapping from names to files/inodes |
| Inode | Metadata for file: owner, permissions, blocks |
| Data block | Actual file content block |
| Superblock | File system metadata |
| Mount | Attach filesystem to directory tree |
| Path | Absolute/relative file location |
| Hard link | Another name for same inode |
| Symbolic link | Pointer path to another file |

### 3.2 File Operations

| Operation | Meaning |
|---|---|
| `open()` | Get file descriptor |
| `read()` | Read bytes from descriptor |
| `write()` | Write bytes to descriptor |
| `lseek()` | Move file offset |
| `fsync()` | Flush file data to disk |
| `close()` | Release descriptor |
| `stat()` | Read metadata |
| `unlink()` | Remove directory entry |

### 3.3 Storage Concepts

| Concept | Meaning |
|---|---|
| Block | Fixed-size storage unit |
| Page cache | Kernel memory cache for file data |
| Buffer cache | Cache for block device data/metadata |
| Journaling | Log metadata/data changes for crash recovery |
| Fragmentation | File blocks scattered on disk |
| Sequential I/O | Contiguous access; faster |
| Random I/O | Scattered access; slower |

### 3.4 Common File Systems

| File System | Notes |
|---|---|
| ext4 | Common Linux general-purpose FS |
| XFS | Good for large files/parallel I/O |
| Btrfs | Snapshots, checksums, copy-on-write |
| ZFS | Checksums, snapshots, pools |
| tmpfs | Memory-backed filesystem |
| NFS | Network file system |

### 3.5 Linux Directory Tree

| Path | Purpose |
|---|---|
| `/` | Root of the filesystem tree |
| `/bin` | Essential user binaries |
| `/sbin` | Essential system/admin binaries |
| `/usr/bin` | Non-essential user commands/apps |
| `/usr/sbin` | Non-essential system/admin commands |
| `/etc` | System configuration files |
| `/dev` | Device files |
| `/proc` | Virtual filesystem for process/kernel info |
| `/sys` | Virtual filesystem for devices/kernel objects |
| `/var` | Variable data: logs, spool, cache, app state |
| `/var/log` | System/application logs |
| `/tmp` | Temporary files, often cleaned automatically |
| `/home` | User home directories |
| `/root` | Root user's home directory |
| `/lib` / `/lib64` | Essential shared libraries/kernel modules |
| `/opt` | Optional third-party software |
| `/mnt` | Temporary manual mounts |
| `/media` | Removable media mounts |
| `/boot` | Bootloader/kernel files |
| `/run` | Runtime state since boot |

## 4. Sockets And File Descriptors

### 4.1 File Descriptor Basics

| FD | Meaning |
|---|---|
| `0` | stdin |
| `1` | stdout |
| `2` | stderr |
| `3+` | Files, sockets, pipes, devices |

### 4.2 Descriptor Types

| Type | Use |
|---|---|
| File descriptor | Open file/socket/pipe handle |
| Socket descriptor | Network endpoint handle |
| Pipe | One-way IPC stream |
| FIFO | Named pipe |
| Eventfd | Kernel event counter |
| Epoll fd | Multiplex many descriptors |

### 4.3 Socket Basics

| Term | Meaning |
|---|---|
| Socket | Endpoint for network communication |
| IP | Host address |
| Port | Process/service endpoint on host |
| TCP | Reliable byte stream |
| UDP | Datagram, no delivery guarantee |
| Bind | Attach socket to local address/port |
| Listen | Mark TCP socket as accepting connections |
| Accept | Create connected socket for client |
| Connect | Client opens connection to server |

### 4.4 Server Socket Flow

| Step | Call |
|---|---|
| 1 | `socket()` |
| 2 | `bind()` |
| 3 | `listen()` |
| 4 | `accept()` |
| 5 | `read()` / `write()` |
| 6 | `close()` |

### 4.5 I/O Models

| Model | Idea |
|---|---|
| Blocking I/O | Thread waits until data ready |
| Non-blocking I/O | Call returns immediately if not ready |
| I/O multiplexing | `select` / `poll` / `epoll` watches many FDs |
| Async I/O | Kernel notifies completion later |
| Reactor | Event loop dispatches ready events |
| Proactor | Completion-based async handling |

## 5. Deadlock And Concurrency

### 5.1 Concurrency Primitives

| Primitive | Use |
|---|---|
| Mutex | Mutual exclusion |
| Semaphore | Counted permits |
| Binary semaphore | Semaphore with 0/1 permits |
| Monitor | Lock + condition variables |
| Condition variable | Wait/signal on condition |
| Spinlock | Busy-wait lock |
| Read-write lock | Many readers or one writer |
| Atomic operation | Indivisible operation |
| Barrier | Wait until all threads arrive |

### 5.2 Deadlock Conditions

| Condition | Meaning |
|---|---|
| Mutual exclusion | Resource held by one thread/process |
| Hold and wait | Holds one resource while waiting for another |
| No preemption | Resource cannot be forcibly taken |
| Circular wait | Cycle of waiting dependencies |

### 5.3 Deadlock Handling

| Approach | Idea |
|---|---|
| Prevention | Break one deadlock condition |
| Avoidance | Allocate only if safe state remains |
| Detection | Find cycle in wait-for graph |
| Recovery | Kill/rollback/preempt resources |
| Timeout | Give up after bounded wait |
| Lock ordering | Acquire locks in fixed global order |

### 5.4 Concurrency Problems

| Problem | Meaning |
|---|---|
| Race condition | Correctness depends on timing |
| Data race | Shared read/write without synchronization |
| Deadlock | Threads/processes wait forever |
| Starvation | One actor never gets resource |
| Livelock | Actors keep moving but no progress |
| Priority inversion | Low-priority task blocks high-priority task |
| Lost wakeup | Signal missed before waiter sleeps |
| Thundering herd | Many waiters wake for one event |

## 6. Classic OS Problems

### 6.1 Peterson's Solution

| Part | Meaning |
|---|---|
| Goal | Mutual exclusion for two processes |
| Shared vars | `flag[2]`, `turn` |
| Guarantees | Mutual exclusion, progress, bounded waiting |
| Limitation | Two-process theoretical algorithm; relies on memory ordering assumptions |

### 6.2 Producer-Consumer / Bounded Buffer

| Item | Reference |
|---|---|
| Problem | Producer fills buffer; consumer drains buffer |
| Risk | Overflow, underflow, race condition |
| Primitives | Mutex + semaphores / condition variables |
| Semaphores | `empty`, `full`, `mutex` |
| Real example | Blocking queue, worker queue, message buffer |

### 6.3 Readers-Writers

| Variant | Meaning |
|---|---|
| Reader preference | Readers can starve writers |
| Writer preference | Writers can starve readers |
| Fair | Queue/order prevents starvation |
| Primitive | Read-write lock |
| Use case | Cache/config/read-heavy shared data |

### 6.4 Dining Philosophers

| Item | Reference |
|---|---|
| Problem | Philosophers need two forks/resources |
| Demonstrates | Deadlock, starvation, resource ordering |
| Naive risk | Everyone picks left fork then waits forever |
| Fixes | Lock ordering, waiter/arbitrator, limit seats, try-lock timeout |

### 6.5 Sleeping Barber

| Item | Reference |
|---|---|
| Problem | Barber sleeps when no customers; customers wait if chairs available |
| Demonstrates | Semaphore coordination |
| Primitives | Customer semaphore, barber semaphore, mutex |
| Risk | Lost wakeups, incorrect waiting count |

### 6.6 Cigarette Smokers

| Item | Reference |
|---|---|
| Problem | Agent provides two resources; smoker with third proceeds |
| Demonstrates | Conditional synchronization |
| Primitives | Semaphores / condition variables |
| Risk | Wrong smoker wakes / missed signal |

## 7. Quick Decision Tables

### 7.1 Choose IPC

| Need | Option |
|---|---|
| Parent-child byte stream | Pipe |
| Named local stream | FIFO / Unix domain socket |
| Network communication | TCP/UDP socket |
| Shared fast data | Shared memory |
| Event notification | Signal / eventfd |
| Many descriptors | epoll |

### 7.2 Choose Locking Tool

| Need | Tool |
|---|---|
| One owner critical section | Mutex |
| Counted resource pool | Semaphore |
| Wait for condition | Condition variable |
| Read-heavy shared state | Read-write lock |
| Simple counter/flag | Atomic |
| Very short kernel/low-level wait | Spinlock |

### 7.3 Debug Checklist

| Symptom | Check |
|---|---|
| High CPU | `top`, busy loop, spinlock, runaway process |
| High memory | `free`, `ps`, leak, cache usage |
| Port busy | `ss -tulpen`, `lsof -i :port` |
| Stuck process | deadlock, blocking I/O, lock wait |
| Too many open files | FD leak, `ulimit -n`, `lsof` |
| Slow disk | `iostat`, random I/O, fsync pressure |
| Zombie process | Parent not reaping child |
