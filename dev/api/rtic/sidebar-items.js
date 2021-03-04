initSidebarItems({"attr":[["app","Attribute used to declare a RTIC application"]],"fn":[["pend","Sets the given `interrupt` as pending"]],"mod":[["mutex_prelude","Makes locks work on N-tuples, locks the mutexes from left-to-right in the tuple. These are used to reduce rightward drift in code and to help make intentions clearer."]],"struct":[["Exclusive","Newtype over `&'a mut T` that implements the `Mutex` trait"]],"trait":[["Monotonic","A monotonic clock / counter definition.Codegen extensions providedThe RTIC codegen will implement an infallible `Monotonic::now()` that simply gets the value from the `Clock::try_now()`, and an `Monotonic::zero()` which returns the zero time for use in `#[init]`."],["Mutex","Memory safe access to shared resources"]]});