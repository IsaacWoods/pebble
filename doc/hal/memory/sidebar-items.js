initSidebarItems({"enum":[["PagingError",""],["Size1GiB",""],["Size2MiB",""],["Size4KiB",""]],"fn":[["gibibytes",""],["kibibytes",""],["mebibytes",""]],"struct":[["FakeFrameAllocator","A `FrameAllocator` that can’t actually allocate or free frames. Useful if you need to pass a `FrameAllocator` to something for testing, but it’ll never actually try to allocate."],["Flags","Defines the permissions for a region of memory. Used both for abstract regions of memory (e.g. entries in a memory map) and as a architecture-common representation of paging structures."],["Frame",""],["Page",""],["PhysicalAddress","Represents a physical address. If the target architecture has any requirements for valid physical addresses, they are always enforced."],["VirtualAddress","Represents a virtual address. On architectures that have extra requirements for canonical virtual addresses (e.g. x86_64 requiring correct sign-extension in high bits), these requirements are always enforced."]],"trait":[["FrameAllocator","`FrameAllocator` is used to interact with a physical memory manager in a platform-independent way. Methods on `FrameAllocator` take `&self` and so are expected to use interior-mutability through a type such as `Mutex` to ensure safe access. This allows structures to store a reference to the allocator, and deallocate memory when they’re dropped."],["FrameSize","This trait is implemented by a number of marker types, one for each size of frame and page. Different size types are defined depending on the target architecture."],["PageTable","A `PageTable` allows the manipulation of a set of page-tables."]],"type":[["Bytes",""],["Gibibytes",""],["Kibibytes",""],["Mebibytes",""]]});