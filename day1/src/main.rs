//! Main file to produce local binary
mod solution;

fn main() {
    let res = solution::solve_problem_1a();
    match res {
        Err(err) => {
            println!("Error solving the problem: {:?}", err)
        }
        Ok(res) => {
            println!("{}", res);
        }
    };
}
