use std::fs;

fn open_file() -> String {
    fs::read_to_string("tests/snapshots/ls-R.log").expect("Unable to read file")
}

#[cfg(test)]
mod tests {
    use super::*;
    use insta;
    use scp_rs::scp::*;

    #[test]
    fn test_find_files() {
        let file = open_file();
        let files_only = find_files(&file);
        println!("{:?}", files_only);
        insta::assert_debug_snapshot!(files_only);
    }
}
