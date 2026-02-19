# Architecture: shrew-cuda

`shrew-cuda` provides GPU acceleration for Shrew using NVIDIA CUDA. It implements the `Backend` trait for `CudaDevice`, enabling tensors to reside in GPU memory and operations to be executed by CUDA kernels.

## Core Concepts

- **CudaBackend**: Manages device memory pointers (`CudaSlice`) and dispatches commands to the GPU stream.
- **cudarc Integration**: Built on top of `cudarc` for safe and ergonomic access to the CUDA Driver and Runtime APIs, as well as NVRTC (Runtime Compilation) for dynamic kernels.
- **Memory Pooling**: Includes an efficient caching allocator (`pool.rs`) to minimize the overhead of `cudaMalloc` and `cudaFree` during training loops.
- **Kernels**: Uses optimized CUDA kernels (PTX/C++) for tensor operations to maximize throughput.

## File Structure

| File | Description | Lines of Code |
| :--- | :--- | :--- |
| `lib.rs` | The main backend implementation. Handles device initialization, context management, memory copy (H2D, D2H), and operation dispatch. | 2423 |
| `kernels.rs` | Contains the CUDA kernel source code (as strings or loaded files) and the logic to launch them with correct grid/block dimensions. | 1533 |
| `pool.rs` | Implements a memory pool/caching allocator to reuse GPU memory allocations, critical for performance in deep learning workloads. | 365 |
