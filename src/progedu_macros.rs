#[macro_export]
macro_rules! nonterm {
    ($r:expr) => {
        NonTerminal::new($r.to_owned())
    };
}

#[macro_export]
macro_rules! term {
    ($r:expr) => {
        Terminal::new($r.to_owned())
    };
}

#[macro_export]
macro_rules! seq { ($($r: expr),* $(,)?) => { Sequence::<Box<dyn Node>>::new(vec![ $( Box::new($r), )* ]) } }

#[macro_export]
macro_rules! choice { ($($r: expr),* $(,)?) => { Choice::<Box<dyn Node>>::new(vec![ $( Box::new($r), )* ]) } }
// macro_rules! v_grid { ($($r: expr),* $(,)?) => { VerticalGrid::<Box<dyn Node>>::new(vec![ $( Box::new($r), )* ]) } }
#[macro_export]
macro_rules! h_grid { ($($r: expr),* $(,)?) => { HorizontalGrid::<Box<dyn Node>>::new(vec![ $( Box::new($r), )* ]) } }

#[macro_export]
macro_rules! v_grid_spaced { ($($r: expr),* $(,)?) => {
    VerticalGrid::<Box<dyn Node>>::new( {
    let vec: Vec<Box<dyn Node>> = vec![ $( Box::new($r), )* ];
    let mut result = Vec::new();
    let len = vec.len();
    for (i, node) in vec.into_iter().enumerate() {
        result.push(node);
        // if not final
        if i + 1 < len {
            result.push(Box::new(seq!()));
            result.push(Box::new(seq!()));
            result.push(Box::new(seq!()));
        }
    }
    result
})
} }

#[macro_export]
macro_rules! rpt {
    ($r:expr, $s:expr) => {
        Repeat::new($r, $s)
    };
    ($r:expr) => {
        rpt!($r, Empty)
    };
}
