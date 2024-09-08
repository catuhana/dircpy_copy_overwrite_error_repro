fn main() {
    assert!(
        cfg!(any(feature = "dircpy_upstream", feature = "dircpy_fork")),
        "Enable one of `dircpy_upstream` or `dircpy_fork` features"
    );
    assert!(
        !cfg!(all(feature = "dircpy_upstream", feature = "dircpy_fork")),
        "Enable only one of `dircpy_upstream` or `dircpy_fork` features"
    )
}
