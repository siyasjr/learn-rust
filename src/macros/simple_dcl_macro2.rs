macro_rules! django {
    ($($arg:expr),*) => {
        
        $(println!("{}", $arg);)*
    };
}

fn main() {
    django!("ssdcsfsdfvfswfswfwsfwsfdw", "gsdfsdfs", "dssfwsewaedsw",2554);
}
