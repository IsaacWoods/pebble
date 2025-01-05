searchState.loadedDescShard("maitake_sync", 0, "maitake-sync\nAn error indicating that a <code>WaitCell</code>, <code>WaitQueue</code> or <code>Semaphore</code>…\nContains the error value\nAn asynchronous mutual exclusion lock for protecting …\nAn RAII implementation of a “scoped lock” of a <code>Mutex</code>. …\nContains the success value\nAn RAII implementation of a “scoped lock” of a <code>Mutex</code>. …\nOwned RAII structure used to release the shared read …\nOwned RAII structure used to release the exclusive write …\nAn asynchronous readers-writer lock.\nRAII structure used to release the shared read access of a …\nRAII structure used to release the exclusive write access …\nAn asynchronous counting semaphore.\nAn atomically registered <code>Waker</code>.\nA map of <code>Waker</code>s associated with keys, allowing tasks to be …\nA queue of waiting tasks which can be woken in first-in, …\nThe result of waiting on a <code>WaitQueue</code> or <code>Semaphore</code>.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nAn asynchronous mutual exclusion lock.\nLocks this <code>RwLock</code> with shared read access, returning an …\nAn asynchronous readers-writer lock.\nAn asynchronous counting semaphore.\nSynchronous spinning-based synchronization primitives.\nAttempts to acquire this <code>RwLock</code> for shared read access, …\nAttempts to acquire this <code>RwLock</code> for exclusive write …\nReusable utilities for synchronization primitives.\nAn atomically registered <code>Waker</code>, for waking a single task.\nA map of <code>Waker</code>s associated with keys, so that a task can …\nA queue of waiting tasks that can be woken in first-in, …\nLocks this <code>RwLock</code> with exclusive write access,returning an …\nA future returned by the <code>Mutex::lock</code> method.\nAn asynchronous mutual exclusion lock for protecting …\nAn RAII implementation of a “scoped lock” of a <code>Mutex</code>. …\nAn RAII implementation of a “scoped lock” of a <code>Mutex</code>. …\nReturns the argument unchanged.\nCalls <code>U::from(self)</code>.\nLocks this mutex.\nLocks this mutex, returning an owned RAII guard.\nReturns a new <code>Mutex</code> protecting the provided <code>data</code>.\nAttempts to lock the mutex without waiting, returning <code>None</code> …\nAttempts this mutex without waiting, returning an owned …\nOwned RAII structure used to release the shared read …\nOwned RAII structure used to release the exclusive write …\nAn asynchronous readers-writer lock.\nRAII structure used to release the shared read access of a …\nRAII structure used to release the exclusive write access …\nReturns a new <code>RwLock</code> protecting the provided <code>data</code>, in an …\nLocks this <code>RwLock</code> with shared read access, causing the …\nAttempts to acquire this <code>RwLock</code> for shared read access, …\nAttempts to acquire this <code>RwLock</code> for exclusive write …\nLocks this <code>RwLock</code> with exclusive write access, causing the …\nThe future returned by the <code>Semaphore::acquire</code> method.\nFuture returned from <code>Semaphore::acquire_owned()</code>.\nThe semaphore has been closed, so additional permits …\nThe semaphore does not currently have enough permits to …\nThe maximum number of permits a <code>Semaphore</code> may contain.\nAn owned RAII guard representing one or more permits …\nA RAII guard representing one or more permits acquired …\nAn asynchronous counting semaphore.\nErrors returned by <code>Semaphore::try_acquire</code>.\nAcquire <code>permits</code> permits from the <code>Semaphore</code>, waiting …\nAcquire <code>permits</code> permits from the <code>Semaphore</code>, waiting …\nAdd <code>permits</code> new permits to the semaphore.\nReturns the number of permits currently available in this …\nCloses the semaphore.\nForget this permit, dropping it <em>without</em> returning the …\nForget this permit, dropping it <em>without</em> returning the …\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nReturns a new <code>Semaphore</code> with <code>permits</code> permits available.\nReturns the count of semaphore permits owned by this <code>Permit</code>…\nReturns the count of semaphore permits owned by this …\nTry to acquire <code>permits</code> permits from the <code>Semaphore</code>, without …\nTry to acquire <code>permits</code> permits from the <code>Semaphore</code>, without …\nA spinlock-based mutual exclusion lock for protecting …\nAn RAII implementation of a “scoped lock” of a mutex. …\nForcibly unlock the mutex.\nReturns the argument unchanged.\nReturns the argument unchanged.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nAcquires a mutex, spinning until it is locked.\nReturns a new <code>Mutex</code> protecting the provided <code>data</code>.\nCells storing a value which must be initialized prior to …\nAttempts to acquire this lock without spinning\nA cell which may be initialized a single time after it is …\nA cell which will be lazily initialized by the provided …\nErrors returned by <code>InitOnce::try_init</code>.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nBorrow the contents of this <code>InitOnce</code> cell, or panic if it …\nBorrow the value, or initialize it if it has not yet been …\nReturns the value of the lazy cell, if it has already been …\nBorrow the value mutably, or initialize it if it has not …\nBorrow the contents of this <code>InitOnce</code> cell, or initialize …\nBorrow the contents of this <code>InitOnce</code> cell, <strong>without</strong> checking\nInitialize the cell to <code>value</code>, panicking if it has already …\nEnsure that the cell has been initialized.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nReturns the value that the caller attempted to initialize …\nReturns a new <code>Lazy</code> cell, initialized with the provided …\nBorrow the contents of this <code>InitOnce</code> cell, if it has been …\nInitialize the cell to <code>value</code>, returning an error if it has …\nReturns a new <code>InitOnce</code> in the uninitialized state.\nAn exponential backoff for spin loops.\nAligns the wrapped value to the size of a cache line.\nThe default maximum exponent (2^8).\nReturns the argument unchanged.\nReturns the argument unchanged.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nUnwraps the inner value and returns it.\nReturns a new exponential backoff with the maximum …\nPads <code>value</code> to the length of a cache line.\nBacks off in a spin loop.\nReturns a new exponential backoff with the provided max …\nThe <code>Waker</code> was not registered because another task was …\nThe <code>Waker</code> was not registered because the <code>WaitCell</code> has been …\nAn error indicating that a <code>WaitCell</code> was closed or busy …\nFuture returned from <code>WaitCell::subscribe()</code>.\nFuture returned from <code>WaitCell::wait()</code>.\nAn atomically registered <code>Waker</code>.\nClose the <code>WaitCell</code>.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nReturns a new <code>WaitCell</code>, with no <code>Waker</code> stored in it.\nPoll to wait on this <code>WaitCell</code>, consuming a stored wakeup or\nEagerly subscribe to notifications from this <code>WaitCell</code>.\nWait to be woken up by this cell.\nWake the <code>Waker</code> stored in this cell.\nThe received data has already been extracted\nThe <code>WaitMap</code> has already been closed.\nThe queue was already closed when the wake was attempted, …\nThe <code>WaitMap</code> already had an item matching the given key\nA future that ensures a <code>Wait</code> has been added to a <code>WaitMap</code>.\nContains the error value\nThe <code>Wait</code> was never added to the <code>WaitMap</code>\nNo task matching the given key was found in the queue.\nContains the success value\nFuture returned from <code>WaitMap::wait()</code>.\nErrors returned by <code>WaitMap::wait</code>, indicating a failed wake.\nA map of <code>Waker</code>s associated with keys, allowing tasks to be …\nFuture returned from <code>WaitMap::wait_owned()</code>.\nThe result of a call to <code>WaitMap::wait()</code>.\nThe result of an attempted <code>WaitMap::wake()</code> operation.\nThe task was successfully woken, and the data was provided.\nClose the queue, indicating that it may no longer be used.\nReturns a future that completes when the <code>Wait</code> item has been\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nReturns a new <code>WaitMap</code>.\nWait to be woken up by this queue.\nWait to be woken up by this queue, returning a future that…\nWake a certain task in the queue.\nFuture returned from <code>WaitQueue::wait()</code>.\nFuture returned from <code>WaitQueue::wait_owned()</code>.\nA queue of waiting tasks which can be woken in first-in, …\nClose the queue, indicating that it may no longer be used.\nReturns the argument unchanged.\nReturns the argument unchanged.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nReturns a new <code>WaitQueue</code>.\nReturns <code>true</code> if <code>self</code> and <code>other</code> are waiting on a …\nReturns <code>true</code> if <code>self</code> and <code>other</code> are waiting on a …\nEagerly subscribe this future to wakeups from …\nEagerly subscribe this future to wakeups from …\nWait to be woken up by this queue.\nWait to be woken up by this queue, returning a future that…\nReturns <code>true</code> if this <code>Wait</code> future is waiting for a …\nReturns <code>true</code> if this <code>WaitOwned</code> future is waiting for a …\nWake the next task in the queue.\nWake <em>all</em> tasks currently in the queue.")