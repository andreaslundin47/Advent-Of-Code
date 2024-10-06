/*
   Looking at the code, I conclude that we seek a value of d such that

       d = a_init + 7 * 365, where a_init >= 0

   which must satisfy the following sequence when integer divison is
   applied between each step:

   d: even ==> output 0
       d = d / 2
   d: odd  ==> output 1
       d = d / 2
   d: even ==> output 0
       d = d / 2
   d: odd  ==> output 1

       ...

   d = 2: even ==> output 0
       d = d / 2
   d = 1: odd  ==> output 1

   After we reach 1, the inital value is restored, and the sequence repeates
   arbitrarily long.

    We solve by strting from the bottom at 1, and iterate until we have a large
    enough d, and thus a_init.
*/

fn main() {
    let mut d = 1;

    while d < 7 * 365 || d % 2 != 0 {
        d = 2 * d + 1 - (d % 2);
    }

    let a_init = d - 7 * 365;

    println!("a = {a_init}");
}
