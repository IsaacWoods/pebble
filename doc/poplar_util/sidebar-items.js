window.SIDEBAR_ITEMS = {"macro":[["assert_first_call","This macro should be called at the beginning of functions that create logic errors if they are called more than once. Most commonly this is used for initialization functions."],["impl_downcast","Adds downcasting support to traits that extend `downcast::Downcast` by defining forwarding methods to the corresponding implementations on `std::any::Any` in the standard library."],["unsafe_pinned","A pinned projection of a struct field."],["unsafe_unpinned","An unpinned projection of a struct field."]],"mod":[["bitmap","It’s useful to be able to model an integral type such as `u32` as being a series of bits, instead of a whole number. There are, of course, the usual bitwise operators for simple stuff, but the `Bitmap` trait provides more complex, specific operations that are useful for bitmaps."],["downcast","A copy of the `downcast-rs` library, but that has been made `no_std` compatible."],["math",""],["pin","This module includes some macros for more easily working with pinned types. It takes inspiration from the `pin-utils` crate, but extends it to provide custom visibility on the created methods."]],"struct":[["BinaryPrettyPrint","Values can be wrapped in this type when they’re printed to display them as easy-to-read binary numbers. `Display` is implemented to print the value in the form `00000000-00000000`, while `Debug` will print it in the form `00000000(8)-00000000(0)` (with offsets of each byte)."],["InitGuard","A guard for when you want to store some data in a static, and explicitly initialize it at some point. This is different from `spin::Once` in that you do not need to provide an initialization method every time you access the object (it also uses `MaybeUninit` instead of `Option`). This will only release shared references to the data inside, so if you want to mutate it from mutable threads, you’ll need to use a type like `Mutex` or `RwLock` within this."]]};