error_chain! {
    foreign_links {
        IoError(::std::io::Error);
        TermError(::term::Error);
    }
}
