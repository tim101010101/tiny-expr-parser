pub(crate) const ERROR: usize = 0;
pub(crate) const START: usize = 1;
pub(crate) const OPERATOR: usize = 2;
pub(crate) const ZERO: usize = 3;
pub(crate) const NUM: usize = 4;

/// a closure function will return
/// which used to determine whether the giving state is a termination state
///
/// # Example
/// ```rust
/// let is_terminator = get_terminator_judgument();
/// assert!(is_terminator(OPERATOR));
/// assert!(is_terminator(NUM));
/// ```
pub(crate) fn get_terminator_judgement() -> impl Fn(usize) -> bool {
    /// all the termination state
    const END_STATE: [usize; 3] = [OPERATOR, ZERO, NUM];
    move |state: usize| END_STATE.contains(&state)
}

/// returns a closure function for state transition as a helper
///
/// # Example
/// ```rust
/// let transition = get_transition();
///
/// let mut state = START;
/// state = transition('+', state);
/// assert_eq!(OPERATOR, state);
/// ```
pub(crate) fn get_transition() -> impl Fn(char, usize) -> usize {
    /// hardcode state transfer table
    ///
    /// |              | op  | ws  | 0   | 1-9 |
    /// |--------------|-----|-----|-----|-----|
    /// | ERROR        | E   | E   | E   | E   |
    /// | START        | 2   | 1   | 3   | 4   |
    /// | OPERATOR     | 2   | 1   | 3   | 4   |
    /// | ZERO         | 2   | 1   | E   | E   |
    /// | NUM          | 2   | 1   | 4   | 4   |
    ///
    const STATE_TABLE: [(usize, usize, usize, usize); 5] = [
        (0, 0, 0, 0), // ERROR
        (2, 1, 3, 4), // START
        (2, 1, 3, 4), // OPERATOR
        (2, 1, 0, 0), // ZERO
        (2, 1, 4, 4), // NUM
    ];
    let is_op = |c: char| match c {
        '-' | '+' | '*' | '/' | '(' | ')' => true,
        _ => false,
    };
    let is_whitespace = |c: char| match c {
        ' ' => true,
        _ => false,
    };
    let is_zero = |c: char| match c {
        '0' => true,
        _ => false,
    };
    let is_one_to_nine = |c: char| match c {
        '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => true,
        _ => false,
    };
    move |c: char, state: usize| {
        if is_op(c) {
            STATE_TABLE[state].0
        } else if is_whitespace(c) {
            STATE_TABLE[state].1
        } else if is_zero(c) {
            STATE_TABLE[state].2
        } else if is_one_to_nine(c) {
            STATE_TABLE[state].3
        } else {
            ERROR
        }
    }
}
