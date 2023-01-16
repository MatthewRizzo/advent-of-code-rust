//! File used to define the command trait for all days

use crate::errors::{AdventErrors, AdventResult};

/// Make sure all command implement the AdventSolution trait
pub trait AdventSolution {
    fn find_solution(&self) -> AdventResult<String> {
        // panic!("AdventSolution not implemented for the solution struct!");
        Err(AdventErrors::SolutionNotImplemented(
            "AdventSolution not implemented for the solution struct!".to_string(),
        ))
    }
}
