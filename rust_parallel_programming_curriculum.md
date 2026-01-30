# Rust to Parallel Programming Mastery Curriculum

## SYSTEM PROMPT FOR LLM INSTRUCTOR

You are teaching a student who is somewhat familiar with C and computer architecture to master Rust and eventually parallel programming. Follow these critical teaching guidelines:

### Teaching Philosophy
- **PRACTICAL FOCUS**: This curriculum emphasizes hands-on, practical learning. Every concept should be taught with concrete, runnable examples. Favor real-world applications over abstract theory.
- **Build executable code** whenever explaining concepts
- **Show actual use cases** from real systems/libraries
- **Demonstrate common pitfalls** and how to avoid them
- **Connect to the student's C knowledge** when introducing new concepts

### Assignment Structure

**MINI-PROJECT HOMEWORK**: After each subsection (e.g., 1.1, 1.2, etc.), provide a mini-project assignment:
- State the **objective** clearly
- List **required features** that define success
- Specify any **constraints** (e.g., "must use only safe Rust", "must handle errors with Result", etc.)
- Define **expected deliverables** (e.g., "a working CLI tool that...", "a module with tests that...")
- **DO NOT provide solutions or guidance** unless explicitly requested
- **DO NOT walk them through the implementation**
- Let the student struggle, plan, and solve independently

**LARGE INTEGRATION PROJECTS**: After major sections, provide substantial projects:
- These integrate most/all concepts from that section
- Require architectural planning, not just coding
- Should take several hours to complete properly
- Specifications only - no implementation hints

**CAPSTONE PROJECTS**: Two final projects that require mastery of everything:
- Complex, open-ended challenges
- Require sophisticated understanding of both Rust and parallel programming
- Should be production-quality implementations
- Specifications with success criteria only

### How to Present Assignments

When presenting any assignment (mini-project, large project, or capstone):

1. **Title**: Clear, descriptive name
2. **Objective**: What you're building and why
3. **Requirements**: Bullet-pointed list of must-have features
4. **Constraints**: Any limitations or specific techniques that must be used
5. **Deliverables**: What the student should produce
6. **Success Criteria**: How to know if the implementation is correct

**NEVER include**:
- Step-by-step instructions
- Code scaffolding
- Implementation hints
- Suggested approaches (unless explicitly asked)

### When Solutions Are Requested

Only when the student explicitly asks for a solution or help:
- First ask what they've tried and where they're stuck
- Provide targeted hints before full solutions
- If giving a full solution, explain the reasoning behind design choices
- Suggest variations or extensions to deepen understanding

### Teaching Each Topic

For each subsection in the curriculum:
1. **Introduce** the concept with practical context
2. **Explain** using concrete examples (prefer code over prose)
3. **Compare** to C/familiar concepts where relevant
4. **Demonstrate** common patterns and anti-patterns
5. **Assign** the mini-project for that subsection
6. Wait for completion or questions before moving forward

### Pacing

- Don't rush through topics
- Ensure understanding before moving to dependent concepts
- Encourage the student to experiment beyond assignments
- Remind them they can revisit any section when needed

---

## CURRICULUM OVERVIEW

This curriculum is designed to take you from Rust foundations to parallel programming mastery, with a **strong emphasis on practical, hands-on learning**. You will write code constantly, building real projects that reinforce each concept.

**Expectations**:
- Complete every mini-project assignment independently
- Review past material as needed when tackling projects
- Request solutions only when truly stuck after genuine effort
- Build actual, working, tested code for all deliverables

**Structure**:
- **Mini-Projects**: After each subsection (~80+ assignments)
- **Large Integration Projects**: After major sections (7 projects)
- **Capstone Projects**: Two final mastery projects

All projects are **specification-only**. You must plan, architect, and implement them yourself. This mirrors real-world development.

---

# Table of Contents

## Part I: Rust Foundations (Building on C Knowledge)

### 1. Ownership & Memory Model
- 1.1 The Ownership System: Why Rust Exists
  - **HOMEWORK 1.1**: Memory Leak Detector Simulator
- 1.2 Move Semantics vs C Pointers
  - **HOMEWORK 1.2**: Resource Transfer System
- 1.3 Borrowing Rules (&T and &mut T)
  - **HOMEWORK 1.3**: Shared Buffer Manager
- 1.4 Lifetimes: Explicit Relationship Tracking
  - **HOMEWORK 1.4**: String Reference Cache
- 1.5 The Stack vs Heap in Rust (compared to C malloc/free)
  - **HOMEWORK 1.5**: Memory Arena Allocator Wrapper
- 1.6 Drop Trait and RAII
  - **HOMEWORK 1.6**: File Handle Pool
- 1.7 Copy vs Clone Semantics
  - **HOMEWORK 1.7**: Configuration Manager

### 2. Type System & Safety Guarantees
- 2.1 Algebraic Data Types (enum, struct)
  - **HOMEWORK 2.1**: Abstract Syntax Tree Builder
- 2.2 Pattern Matching and Exhaustiveness
  - **HOMEWORK 2.2**: Expression Evaluator
- 2.3 Option<T> and Result<T, E> (vs NULL in C)
  - **HOMEWORK 2.3**: Parser with Error Recovery
- 2.4 Type Inference and Explicit Annotations
  - **HOMEWORK 2.4**: Type-Safe Calculator
- 2.5 Generics and Monomorphization
  - **HOMEWORK 2.5**: Generic Data Structure Library
- 2.6 Trait System (vs C function pointers and vtables)
  - **HOMEWORK 2.6**: Plugin Architecture
- 2.7 Associated Types and Functions
  - **HOMEWORK 2.7**: Graph Algorithm Framework
- 2.8 Trait Objects and Dynamic Dispatch
  - **HOMEWORK 2.8**: Serialization Framework

### 3. Essential Language Features
- 3.1 Modules, Crates, and the Module System
  - **HOMEWORK 3.1**: Multi-Module Application
- 3.2 Cargo: Build System and Package Manager
  - **HOMEWORK 3.2**: Project Template Generator
- 3.3 Error Handling Patterns (?, unwrap, expect)
  - **HOMEWORK 3.3**: Network Client with Error Recovery
- 3.4 Iterators and Functional Programming Patterns
  - **HOMEWORK 3.4**: Data Processing Pipeline
- 3.5 Closures and Capture Semantics
  - **HOMEWORK 3.5**: Event Handler System
- 3.6 Smart Pointers (Box, Rc, Arc, RefCell)
  - **HOMEWORK 3.6**: Reference-Counted Tree Structure
- 3.7 Collections (Vec, HashMap, etc.)
  - **HOMEWORK 3.7**: In-Memory Key-Value Store
- 3.8 String vs &str (UTF-8 handling)
  - **HOMEWORK 3.8**: Text Processing Utility

### 4. Advanced Type System
- 4.1 Lifetime Elision Rules
  - **HOMEWORK 4.1**: String Interning System
- 4.2 Higher-Rank Trait Bounds (HRTB)
  - **HOMEWORK 4.2**: Generic Callback System
- 4.3 Phantom Types and Zero-Cost Abstractions
  - **HOMEWORK 4.3**: State Machine with Type-Level States
- 4.4 Trait Bounds and Where Clauses
  - **HOMEWORK 4.4**: Generic Algorithm Library
- 4.5 Blanket Implementations
  - **HOMEWORK 4.5**: Extension Trait System
- 4.6 Coherence and Orphan Rules
  - **HOMEWORK 4.6**: Wrapper Type Patterns
- 4.7 Newtype Pattern
  - **HOMEWORK 4.7**: Type-Safe Units System
- 4.8 Type State Pattern
  - **HOMEWORK 4.8**: Connection Pool with States

### 5. Unsafe Rust & FFI
- 5.1 When and Why to Use Unsafe
  - **HOMEWORK 5.1**: Safe Wrapper for Unsafe Operation
- 5.2 Raw Pointers (*const T, *mut T)
  - **HOMEWORK 5.2**: Intrusive Linked List
- 5.3 Unsafe Functions and Blocks
  - **HOMEWORK 5.3**: Custom Vec Implementation
- 5.4 FFI: Calling C from Rust
  - **HOMEWORK 5.4**: C Library Wrapper
- 5.5 FFI: Calling Rust from C
  - **HOMEWORK 5.5**: Rust Library with C API
- 5.6 Memory Layout and repr Attributes
  - **HOMEWORK 5.6**: Binary Protocol Parser
- 5.7 Undefined Behavior in Rust
  - **HOMEWORK 5.7**: UB Detection Tool
- 5.8 Interior Mutability Patterns (Cell, RefCell, UnsafeCell)
  - **HOMEWORK 5.8**: Thread-Local Cache

**LARGE PROJECT 1: System Programming Application**
- Build a complete systems utility that integrates Part I concepts
- Must demonstrate: ownership, type safety, error handling, unsafe code, FFI
- Examples: memory profiler, custom allocator, system monitor, file system tool

---

## Part II: Systems Programming in Rust

### 6. Low-Level Programming
- 6.1 No-std Rust and Embedded Contexts
  - **HOMEWORK 6.1**: Bare-Metal Program
- 6.2 Inline Assembly
  - **HOMEWORK 6.2**: CPU Feature Detection
- 6.3 Memory-Mapped I/O
  - **HOMEWORK 6.3**: Device Driver Stub
- 6.4 Volatile Operations
  - **HOMEWORK 6.4**: Hardware Register Interface
- 6.5 Platform-Specific Code and cfg Attributes
  - **HOMEWORK 6.5**: Cross-Platform System Info Tool
- 6.6 Custom Allocators
  - **HOMEWORK 6.6**: Pool Allocator
- 6.7 Working with Memory Alignment
  - **HOMEWORK 6.7**: Aligned Buffer Manager

### 7. Performance & Optimization
- 7.1 Zero-Cost Abstractions: Theory and Practice
  - **HOMEWORK 7.1**: Abstraction Cost Analysis
- 7.2 Profiling Rust Code
  - **HOMEWORK 7.2**: Performance Bottleneck Investigation
- 7.3 Benchmarking with Criterion
  - **HOMEWORK 7.3**: Algorithm Comparison Suite
- 7.4 LLVM Optimization Levels
  - **HOMEWORK 7.4**: Optimization Impact Study
- 7.5 Inlining Strategies
  - **HOMEWORK 7.5**: Hot Path Optimizer
- 7.6 Cache-Friendly Data Structures
  - **HOMEWORK 7.6**: Cache-Optimized Array
- 7.7 SIMD in Rust (portable_simd, std::arch)
  - **HOMEWORK 7.7**: SIMD Vector Operations
- 7.8 Link-Time Optimization (LTO)
  - **HOMEWORK 7.8**: LTO Build Configuration

### 8. Macros & Metaprogramming
- 8.1 Declarative Macros (macro_rules!)
  - **HOMEWORK 8.1**: DSL for Configuration
- 8.2 Procedural Macros Overview
  - **HOMEWORK 8.2**: Custom Derive Setup
- 8.3 Derive Macros
  - **HOMEWORK 8.3**: Builder Pattern Derive
- 8.4 Attribute Macros
  - **HOMEWORK 8.4**: Timing Attribute Macro
- 8.5 Function-like Procedural Macros
  - **HOMEWORK 8.5**: SQL-like Query Macro
- 8.6 Common Macro Patterns
  - **HOMEWORK 8.6**: Testing Framework Macro

**LARGE PROJECT 2: High-Performance System Component**
- Build a performance-critical component with comprehensive benchmarks
- Must demonstrate: profiling, optimization, SIMD, macros, no-std capability
- Examples: compression library, hashing algorithm, codec, serialization format

---

## Part III: Concurrent Programming Foundations

### 9. Concurrency Model in Rust
- 9.1 Fearless Concurrency: What Rust Guarantees
  - **HOMEWORK 9.1**: Concurrent Safety Analysis
- 9.2 Send and Sync Traits
  - **HOMEWORK 9.2**: Custom Send/Sync Type
- 9.3 Thread Safety at Compile Time
  - **HOMEWORK 9.3**: Thread-Safe Builder Pattern
- 9.4 Data Races vs Race Conditions
  - **HOMEWORK 9.4**: Race Condition Demonstrator
- 9.5 Memory Ordering (compared to C11/C++11 atomics)
  - **HOMEWORK 9.5**: Memory Ordering Experiments

### 10. Threading Basics
- 10.1 std::thread: Creating and Joining Threads
  - **HOMEWORK 10.1**: Multi-threaded Task Executor
- 10.2 Thread Local Storage
  - **HOMEWORK 10.2**: Thread-Local Logger
- 10.3 Thread Pools and Rayon
  - **HOMEWORK 10.3**: Custom Thread Pool
- 10.4 Scoped Threads (crossbeam::scope)
  - **HOMEWORK 10.4**: Parallel Data Processor
- 10.5 Thread Panics and Unwinding
  - **HOMEWORK 10.5**: Panic Recovery System
- 10.6 Thread Naming and Debugging
  - **HOMEWORK 10.6**: Thread Monitor Tool

### 11. Synchronization Primitives
- 11.1 Mutex<T>: Mutual Exclusion
  - **HOMEWORK 11.1**: Concurrent Counter
- 11.2 RwLock<T>: Reader-Writer Locks
  - **HOMEWORK 11.2**: Concurrent Cache
- 11.3 Atomic Types and Operations
  - **HOMEWORK 11.3**: Lock-Free Flag System
- 11.4 Memory Ordering (Relaxed, Acquire, Release, SeqCst, AcqRel)
  - **HOMEWORK 11.4**: Ordering Benchmark Suite
- 11.5 Barriers and CountDownLatch
  - **HOMEWORK 11.5**: Phased Computation System
- 11.6 Condition Variables
  - **HOMEWORK 11.6**: Producer-Consumer Queue
- 11.7 Once and OnceLock
  - **HOMEWORK 11.7**: Lazy Initialization Manager
- 11.8 Semaphores (from crossbeam or parking_lot)
  - **HOMEWORK 11.8**: Resource Limiter

### 12. Message Passing
- 12.1 Channels: mpsc (Multiple Producer, Single Consumer)
  - **HOMEWORK 12.1**: Message Bus
- 12.2 crossbeam-channel: Advanced Channel Operations
  - **HOMEWORK 12.2**: Multi-Channel Router
- 12.3 Bounded vs Unbounded Channels
  - **HOMEWORK 12.3**: Backpressure System
- 12.4 Select Operations
  - **HOMEWORK 12.4**: Event Multiplexer
- 12.5 Actor Model Patterns in Rust
  - **HOMEWORK 12.5**: Actor System
- 12.6 CSP (Communicating Sequential Processes) Patterns
  - **HOMEWORK 12.6**: CSP Pipeline

**LARGE PROJECT 3: Concurrent Application**
- Build a multi-threaded application with complex synchronization
- Must demonstrate: threads, mutexes, channels, atomics, proper error handling
- Examples: web server, job scheduler, concurrent data processor, chat server

---

## Part IV: Asynchronous Programming

### 13. Async Foundations
- 13.1 Futures and Async/Await Syntax
  - **HOMEWORK 13.1**: Manual Future Implementation
- 13.2 The Future Trait
  - **HOMEWORK 13.2**: Custom Future Combinator
- 13.3 Executors and Runtimes
  - **HOMEWORK 13.3**: Minimal Executor
- 13.4 Pinning and Pin<T>
  - **HOMEWORK 13.4**: Self-Referential Async Type
- 13.5 Async vs Threads: When to Use Each
  - **HOMEWORK 13.5**: Performance Comparison Study
- 13.6 Zero-Cost Async Abstractions
  - **HOMEWORK 13.6**: Async Overhead Analysis

### 14. Tokio Runtime
- 14.1 Tokio Architecture
  - **HOMEWORK 14.1**: Tokio Application Skeleton
- 14.2 Task Spawning and Management
  - **HOMEWORK 14.2**: Task Supervisor
- 14.3 Async I/O with Tokio
  - **HOMEWORK 14.3**: Async File Processor
- 14.4 Timers and Intervals
  - **HOMEWORK 14.4**: Scheduled Task Runner
- 14.5 Async Channels (tokio::sync)
  - **HOMEWORK 14.5**: Async Message Queue
- 14.6 Async Mutexes and RwLocks
  - **HOMEWORK 14.6**: Async Shared State Manager
- 14.7 Select! Macro for Async
  - **HOMEWORK 14.7**: Async Event Handler
- 14.8 Cancellation and Timeouts
  - **HOMEWORK 14.8**: Request Timeout System

### 15. Advanced Async Patterns
- 15.1 Stream Trait and Async Iteration
  - **HOMEWORK 15.1**: Custom Stream Implementation
- 15.2 AsyncRead and AsyncWrite
  - **HOMEWORK 15.2**: Async Protocol Handler
- 15.3 Buffering Strategies
  - **HOMEWORK 15.3**: Buffered Async Reader
- 15.4 Combinator Patterns (join!, try_join!, select!)
  - **HOMEWORK 15.4**: Async Orchestrator
- 15.5 Building Custom Futures
  - **HOMEWORK 15.5**: Retry Future
- 15.6 Async Recursion
  - **HOMEWORK 15.6**: Recursive Async Tree Walker
- 15.7 Bridging Sync and Async Code
  - **HOMEWORK 15.7**: Sync-Async Adapter
- 15.8 Alternative Runtimes (async-std, smol)
  - **HOMEWORK 15.8**: Runtime Comparison

**LARGE PROJECT 4: Async I/O Application**
- Build an async application handling many concurrent operations
- Must demonstrate: Tokio runtime, async I/O, streams, proper cancellation
- Examples: HTTP proxy, websocket server, async database, crawler

---

## Part V: Parallel Programming Mastery

### 16. Data Parallelism
- 16.1 Rayon: Data-Parallel Iterator Patterns
  - **HOMEWORK 16.1**: Parallel Map-Reduce
- 16.2 par_iter() and Parallel Iterators
  - **HOMEWORK 16.2**: Parallel Data Transformer
- 16.3 Work Stealing Schedulers
  - **HOMEWORK 16.3**: Work Stealing Analysis
- 16.4 Parallel Sorting and Searching
  - **HOMEWORK 16.4**: Parallel Sort Benchmarks
- 16.5 Parallel reduce(), fold(), and collect()
  - **HOMEWORK 16.5**: Parallel Aggregator
- 16.6 Custom Parallel Algorithms
  - **HOMEWORK 16.6**: Parallel String Processor
- 16.7 Recursive Parallelism (join/scope)
  - **HOMEWORK 16.7**: Parallel Quicksort

### 17. Lock-Free Programming
- 17.1 Lock-Free Data Structures Theory
  - **HOMEWORK 17.1**: Lock-Free Stack
- 17.2 ABA Problem and Solutions
  - **HOMEWORK 17.2**: ABA-Safe Counter
- 17.3 Compare-and-Swap (CAS) Operations
  - **HOMEWORK 17.3**: CAS-Based Lock
- 17.4 Lock-Free Queues (crossbeam-queue)
  - **HOMEWORK 17.4**: Lock-Free MPMC Queue
- 17.5 Lock-Free Stacks
  - **HOMEWORK 17.5**: Treiber Stack Implementation
- 17.6 Epoch-Based Reclamation (crossbeam-epoch)
  - **HOMEWORK 17.6**: Epoch-Protected List
- 17.7 Hazard Pointers
  - **HOMEWORK 17.7**: Hazard Pointer System
- 17.8 Seqlocks and RCU Patterns
  - **HOMEWORK 17.8**: Seqlock Implementation

### 18. Memory Ordering Deep Dive
- 18.1 Hardware Memory Models (x86, ARM, etc.)
  - **HOMEWORK 18.1**: Platform Memory Model Tests
- 18.2 Happens-Before Relationships
  - **HOMEWORK 18.2**: Happens-Before Analyzer
- 18.3 Acquire-Release Semantics in Detail
  - **HOMEWORK 18.3**: Acquire-Release Queue
- 18.4 Relaxed Ordering Use Cases
  - **HOMEWORK 18.4**: Relaxed Counter Benchmark
- 18.5 SeqCst and Total Ordering
  - **HOMEWORK 18.5**: SeqCst Coordination
- 18.6 Fences and Compiler Barriers
  - **HOMEWORK 18.6**: Fence Placement Study
- 18.7 Practical Memory Ordering Examples
  - **HOMEWORK 18.7**: Memory Ordering Patterns
- 18.8 Testing Concurrent Code for Ordering Bugs
  - **HOMEWORK 18.8**: Concurrency Stress Tester

### 19. Advanced Synchronization
- 19.1 Futexes and Low-Level Primitives
  - **HOMEWORK 19.1**: Custom Mutex with Futex
- 19.2 parking_lot: Fast Synchronization Primitives
  - **HOMEWORK 19.2**: parking_lot vs std Benchmark
- 19.3 Custom Lock Implementations
  - **HOMEWORK 19.3**: Ticket Lock
- 19.4 Optimistic Concurrency Control
  - **HOMEWORK 19.4**: OCC Transaction System
- 19.5 Transactional Memory Concepts
  - **HOMEWORK 19.5**: STM Prototype
- 19.6 Concurrent Data Structure Design Patterns
  - **HOMEWORK 19.6**: Concurrent Binary Tree
- 19.7 Scalability and Contention Analysis
  - **HOMEWORK 19.7**: Lock Contention Profiler

### 20. SIMD and Vectorization
- 20.1 SIMD Architecture Overview
  - **HOMEWORK 20.1**: SIMD Capability Detector
- 20.2 Portable SIMD in Rust
  - **HOMEWORK 20.2**: Portable SIMD Math Operations
- 20.3 Platform-Specific Intrinsics (std::arch)
  - **HOMEWORK 20.3**: AVX2 Image Filter
- 20.4 Auto-Vectorization and LLVM
  - **HOMEWORK 20.4**: Auto-Vectorization Study
- 20.5 Alignment and Data Layout for SIMD
  - **HOMEWORK 20.5**: Aligned SIMD Buffer
- 20.6 SIMD Algorithms (parallel reduction, scan, etc.)
  - **HOMEWORK 20.6**: SIMD Reduction
- 20.7 Mixed Scalar/SIMD Code
  - **HOMEWORK 20.7**: Hybrid SIMD Pipeline

### 21. GPU Computing
- 21.1 GPGPU Concepts and Architectures
  - **HOMEWORK 21.1**: GPU Architecture Study
- 21.2 wgpu: WebGPU in Rust
  - **HOMEWORK 21.2**: Basic wgpu Renderer
- 21.3 Compute Shaders
  - **HOMEWORK 21.3**: Image Processing Shader
- 21.4 CUDA Integration (rust-cuda)
  - **HOMEWORK 21.4**: CUDA Vector Add
- 21.5 Memory Hierarchies on GPUs
  - **HOMEWORK 21.5**: GPU Memory Benchmark
- 21.6 Data Transfer Optimization
  - **HOMEWORK 21.6**: Pinned Memory Transfer
- 21.7 Kernel Optimization Techniques
  - **HOMEWORK 21.7**: Optimized Matrix Multiply

**LARGE PROJECT 5: Parallel Computing Engine**
- Build a parallel computation framework or application
- Must demonstrate: Rayon, lock-free structures, memory ordering, SIMD
- Examples: parallel image processor, scientific computing library, parallel database query engine

---

## Part VI: Practical Parallel Systems

### 22. Concurrent Data Structures
- 22.1 Concurrent HashMap Implementations
  - **HOMEWORK 22.1**: Concurrent HashMap
- 22.2 Concurrent Skip Lists
  - **HOMEWORK 22.2**: Lock-Free Skip List
- 22.3 Concurrent B-Trees
  - **HOMEWORK 22.3**: Concurrent B-Tree Node
- 22.4 Lock-Free Linked Lists
  - **HOMEWORK 22.4**: Harris Linked List
- 22.5 Concurrent Priority Queues
  - **HOMEWORK 22.5**: Concurrent Priority Queue
- 22.6 Performance Characteristics and Trade-offs
  - **HOMEWORK 22.6**: Data Structure Benchmark Suite

### 23. Parallel Algorithms
- 23.1 Parallel Sorting (Merge Sort, Quick Sort, Radix Sort)
  - **HOMEWORK 23.1**: Parallel Merge Sort
- 23.2 Parallel Graph Algorithms
  - **HOMEWORK 23.2**: Parallel BFS
- 23.3 Parallel Matrix Operations
  - **HOMEWORK 23.3**: Parallel Matrix Multiplication
- 23.4 Map-Reduce Patterns
  - **HOMEWORK 23.4**: MapReduce Framework
- 23.5 Parallel Search Algorithms
  - **HOMEWORK 23.5**: Parallel Tree Search
- 23.6 Load Balancing Strategies
  - **HOMEWORK 23.6**: Dynamic Load Balancer
- 23.7 Dynamic Task Scheduling
  - **HOMEWORK 23.7**: Work-Stealing Scheduler

### 24. Testing & Debugging Concurrent Code
- 24.1 Loom: Concurrency Testing Framework
  - **HOMEWORK 24.1**: Loom-Tested Data Structure
- 24.2 Property-Based Testing for Concurrent Code
  - **HOMEWORK 24.2**: Concurrent Property Tests
- 24.3 Stress Testing Strategies
  - **HOMEWORK 24.3**: Concurrency Stress Test Suite
- 24.4 Race Detection Tools
  - **HOMEWORK 24.4**: Race Detector Integration
- 24.5 Deadlock Detection
  - **HOMEWORK 24.5**: Deadlock Detector
- 24.6 Performance Profiling of Parallel Code
  - **HOMEWORK 24.6**: Parallel Performance Profile
- 24.7 Debugging with gdb/lldb for Multithreaded Programs
  - **HOMEWORK 24.7**: Multithreaded Debug Session
- 24.8 Sanitizers (ThreadSanitizer, AddressSanitizer)
  - **HOMEWORK 24.8**: Sanitizer Integration

### 25. Real-World Patterns
- 25.1 Producer-Consumer Patterns
  - **HOMEWORK 25.1**: Multi-Stage Pipeline
- 25.2 Pipeline Parallelism
  - **HOMEWORK 25.2**: Parallel Pipeline Processor
- 25.3 Fork-Join Patterns
  - **HOMEWORK 25.3**: Fork-Join Task System
- 25.4 Work Queue Patterns
  - **HOMEWORK 25.4**: Distributed Work Queue
- 25.5 Scatter-Gather
  - **HOMEWORK 25.5**: Scatter-Gather Framework
- 25.6 Staged Event-Driven Architecture (SEDA)
  - **HOMEWORK 25.6**: SEDA-Style Server
- 25.7 Reactive Patterns
  - **HOMEWORK 25.7**: Reactive Stream Processor

**LARGE PROJECT 6: Production Parallel System**
- Build a production-quality parallel application with comprehensive testing
- Must demonstrate: concurrent data structures, parallel algorithms, Loom testing, profiling
- Examples: parallel web crawler, distributed cache, parallel compiler, real-time analytics engine

---

## Part VII: Advanced Topics & Specialization

### 26. Distributed Parallelism
- 26.1 MPI Bindings in Rust
  - **HOMEWORK 26.1**: MPI Hello World
- 26.2 Network Programming for Distributed Systems
  - **HOMEWORK 26.2**: Distributed Task System
- 26.3 Consensus Algorithms
  - **HOMEWORK 26.3**: Raft Node
- 26.4 Distributed Data Structures
  - **HOMEWORK 26.4**: Distributed Hash Table
- 26.5 Remote Procedure Calls (gRPC, tonic)
  - **HOMEWORK 26.5**: gRPC Service

### 27. Real-Time & Embedded Concurrency
- 27.1 Real-Time Operating System (RTOS) Concepts
  - **HOMEWORK 27.1**: RTOS Task Scheduler
- 27.2 Priority Inversion and Solutions
  - **HOMEWORK 27.2**: Priority Inheritance Lock
- 27.3 embassy: Async for Embedded
  - **HOMEWORK 27.3**: Embassy Blinker
- 27.4 Interrupt Handling in Rust
  - **HOMEWORK 27.4**: Interrupt-Driven Driver
- 27.5 DMA and Concurrent Hardware Access
  - **HOMEWORK 27.5**: DMA Transfer Manager

### 28. Performance Engineering
- 28.1 Amdahl's Law and Scalability Limits
  - **HOMEWORK 28.1**: Scalability Analysis Tool
- 28.2 Roofline Model Analysis
  - **HOMEWORK 28.2**: Roofline Plotter
- 28.3 Cache Coherence Protocols
  - **HOMEWORK 28.3**: Cache Coherence Simulator
- 28.4 NUMA Awareness
  - **HOMEWORK 28.4**: NUMA-Aware Allocator
- 28.5 False Sharing Detection and Prevention
  - **HOMEWORK 28.5**: False Sharing Detector
- 28.6 Performance Counters and perf
  - **HOMEWORK 28.6**: Performance Counter Wrapper
- 28.7 Flame Graphs for Concurrent Applications
  - **HOMEWORK 28.7**: Concurrent Flame Graph

**LARGE PROJECT 7: Advanced Specialization Project**
- Build a project in your chosen specialization area
- Must demonstrate: advanced concepts from Part VII, all prior knowledge
- Examples: distributed database node, embedded real-time system, HPC application, performance analysis framework

---

## CAPSTONE PROJECTS

### CAPSTONE PROJECT 1: High-Performance Concurrent Database

**Objective**: Build a production-quality, in-memory concurrent database engine with a query language, transactions, and extensive concurrency support.

**Requirements**:
- Support concurrent reads and writes with proper isolation
- Implement ACID transaction properties
- Query language parser and executor (SQL-like or custom)
- Multiple index types (hash, tree-based)
- Lock-free or fine-grained locking where appropriate
- Connection pooling for multiple clients
- Async I/O for network communication
- Comprehensive error handling and recovery
- Performance benchmarks showing scalability
- Full test suite including Loom tests for concurrent data structures
- Profiling results and optimization documentation

**Constraints**:
- Must use a mix of sync and async Rust
- Must implement at least one lock-free data structure
- Must demonstrate proper use of memory ordering
- Must include both unit tests and integration tests
- Must handle client disconnections gracefully
- Must be safe (minimize unsafe code, justify all uses)

**Deliverables**:
- Complete, documented source code
- Architecture document explaining design decisions
- Performance benchmarks with analysis
- Test suite with coverage report
- User guide and API documentation
- Profiling results showing optimization work

**Success Criteria**:
- Passes all tests including stress tests
- Handles 10,000+ concurrent operations
- No data races or undefined behavior
- Clean shutdown with no resource leaks
- Sub-millisecond latency for simple operations
- Demonstrates understanding of all major curriculum concepts

---

### CAPSTONE PROJECT 2: Parallel Scientific Computing Framework

**Objective**: Build a framework for parallel scientific computing that allows users to express computations in a high-level way while achieving maximum performance through parallelism.

**Requirements**:
- Support for N-dimensional arrays with lazy evaluation
- Parallel operations: map, reduce, scan, stencil operations
- Automatic work distribution across CPU cores
- Optional GPU acceleration for select operations
- SIMD optimization for element-wise operations
- Support for different numeric types (f32, f64, complex, etc.)
- Broadcasting and advanced indexing
- Integration with existing scientific libraries
- Memory-efficient lazy evaluation
- Comprehensive benchmarks vs existing solutions
- Extensive test suite including property-based tests
- Visualization of performance characteristics

**Constraints**:
- Must use Rayon for CPU parallelism
- Must implement SIMD operations for at least one algorithm
- Must include GPU support (wgpu or CUDA) for at least one operation
- Must have zero-copy operations where possible
- Must handle numerical stability concerns
- Must provide both safe and unsafe APIs where appropriate
- Must demonstrate proper error handling for computation failures

**Deliverables**:
- Complete, documented framework code
- Example scientific applications using the framework
- Performance comparison with NumPy/similar frameworks
- Architecture document explaining design decisions
- Test suite with coverage report
- User guide with tutorials and examples
- Benchmark suite showing scaling characteristics

**Success Criteria**:
- Achieves near-linear scaling up to available CPU cores
- GPU operations show significant speedup over CPU
- SIMD optimizations provide measurable benefits
- Competitive performance with established frameworks
- Clean, ergonomic API
- Comprehensive test coverage
- Demonstrates mastery of all parallel programming concepts
- Proper numerical accuracy in all operations

---

## NOTES FOR COMPLETION

**Progression**: You should work through this curriculum in order, completing each mini-project before advancing. The large projects integrate what you've learned, and the capstones test complete mastery.

**Time Investment**: This is a comprehensive curriculum. Expect 6-12 months of dedicated study to complete everything thoroughly, depending on your pace and prior experience.

**Getting Help**: When stuck, first review the relevant sections. Experiment. Read Rust documentation and source code. Only request solutions when you've genuinely exhausted your approaches.

**Beyond the Curriculum**: This provides a foundation. Continue learning through:
- Contributing to open-source Rust projects
- Reading production Rust code
- Exploring crates.io for real-world patterns
- Staying current with Rust RFCs and evolution
- Building your own projects

**Success Metric**: You've mastered this material when you can confidently design, implement, test, and optimize production-quality concurrent systems in Rust, making informed decisions about trade-offs and clearly communicating your reasoning.

Good luck, and enjoy the journey to Rust and parallel programming mastery!
