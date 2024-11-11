//! # Workspace Member1
//!
//! Here's a diagram straight in the docstrings
//! ```mermaid
//! stateDiagram-v2
//!     [*] --> Still
//!     Still --> [*]
//!     Still --> Moving
//!     Moving --> Still
//!     Moving --> Crash
//!     Crash --> [*]
//! ```
//!
//! Here's the ARCHITECTURE.md document included into the workspace members root documentation
#![doc = include_str!("../../ARCHITECTURE.md")]
//!
//! See also [Workspace Member2](../workspace_member2/index.html)

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
