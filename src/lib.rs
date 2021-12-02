use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::path::Path;
use std::str::FromStr;

pub fn stream_ints<I, T>(input: I) -> impl Iterator<Item = T>
where
    I: Read,
    T: FromStr,
{
    BufReader::new(input)
        .lines()
        .filter_map(Result::ok)
        .map(|line| T::from_str(&line))
        .filter_map(Result::ok)
}

pub fn stream_items_from_file<P: AsRef<Path>, T: FromStr>(
    path: P,
) -> std::io::Result<impl Iterator<Item = T>> {
    Ok(stream_ints(File::open(path)?))
}

pub mod test_helpers {
    use std::{fmt::Display, fs::File, io::Write, path::Path};
    use tempfile::{tempdir, TempDir};

    pub fn create_line_file<T: Display, I: Iterator<Item = T>>(
        inp: I,
        dir: Option<TempDir>,
    ) -> (TempDir, impl AsRef<Path>) {
        let dir = dir.unwrap_or_else(|| tempdir().expect("Failed to create tempdir"));
        let filepath = dir.path().join("tempinput.txt");
        let mut file = File::create(&filepath).expect("Could not create file");
        inp.for_each(|item| writeln!(file, "{}", item).expect("Could not write to file"));
        (dir, filepath)
    }
}
