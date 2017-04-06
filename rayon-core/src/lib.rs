//!
//! [Under construction](https://github.com/nikomatsakis/rayon/issues/231)
//!
//! ## Restricting multiple versions
//!
//! In order to ensure proper coordination between threadpools, and especially
//! to make sure there's only one global threadpool, `rayon-core` is actively
//! restricted from building multiple versions of itself into a single target.
//! You may see a build error like this in violation:
//!
//! ```text
//! error: native library `rayon-core` is being linked to by more
//! than one package, and can only be linked to by one package
//! ```
//!
//! While we strive to keep `rayon-core` semver-compatible, it's still
//! possible to arrive at this situation if different crates have overly
//! restrictive tilde or inequality requirements for `rayon-core`.  The
//! conflicting requirements will need to be resolved before the build will
//! succeed.

#![allow(non_camel_case_types)] // I prefer to use ALL_CAPS for type parameters
#![cfg_attr(test, feature(conservative_impl_trait))]

// If you're not compiling the unstable code, it often happens that
// there is stuff that is considered "dead code" and so forth. So
// disable warnings in that scenario.
#![cfg_attr(not(feature = "unstable"), allow(warnings))]

extern crate deque;
#[macro_use]
extern crate lazy_static;
#[cfg(feature = "unstable")]
extern crate futures;
extern crate libc;
extern crate num_cpus;
extern crate rand;

#[macro_use]
mod log;

mod configuration;
mod latch;
mod join;
mod job;
mod registry;
#[cfg(feature = "unstable")]
mod future;
mod scope;
mod sleep;
#[cfg(feature = "unstable")]
mod spawn_async;
mod test;
mod thread_pool;
mod unwind;
mod util;

pub use configuration::Configuration;
pub use configuration::ExitHandler;
pub use configuration::PanicHandler;
pub use configuration::StartHandler;
pub use configuration::dump_stats;
pub use configuration::initialize;
pub use thread_pool::ThreadPool;
pub use join::join;
pub use scope::{scope, Scope};
#[cfg(feature = "unstable")]
pub use spawn_async::spawn_async;
#[cfg(feature = "unstable")]
pub use spawn_async::spawn_future_async;
#[cfg(feature = "unstable")]
pub use future::RayonFuture;

/// Returns the number of threads in the current registry. If this
/// code is executing within the Rayon thread-pool, then this will be
/// the number of threads for the current thread-pool. Otherwise, it
/// will be the number of threads for the global thread-pool.
///
/// This can be useful when trying to judge how many times to split
/// parallel work (the parallel iterator traits use this value
/// internally for this purpose).
pub fn current_num_threads() -> usize {
    ::registry::Registry::current_num_threads()
}
