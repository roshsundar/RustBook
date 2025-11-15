/* Summary:
Patterns come in 2 forms, refutable and irrefutable.

Irrefutable patterns match any possible value passed.
    i.e. let x = 5; 
         x matches anything.
Refutable patterns can fail to match for some value(s).
    i.e. if let Some(x) = value
         If value is None, Some(x) won't match.

Function params, let statements, and for loops only accept irrefutable patterns b/c
the program can't handle when the pattern doesn't match.

if let, while let, and let...else accept refutable and irrefutable patterns,
but are meant for refurable ones. 
*/

fn main() {
    // let Some(x) = Some(2); // ! err: refutable patterns in local binding, let binidings require an irrefutable pattern

    // The let...else construct allows a refutable pattern.
    let Some(x) = Some(2) else {
        return;
    };
}
