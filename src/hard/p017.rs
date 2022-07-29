/// Suppose we represent our file system by a string in the following manner:
///
/// The string `"dir\n\tsubdir1\n\tsubdir2\n\t\tfile.ext"` represents:
///
/// ```
/// dir
///     subdir1
///     subdir2
///         file.ext
/// ```
///
/// The directory `dir` contains an empty sub-directory `subdir1` and a sub-directory `subdir2` containing a file `file.ext`.
///
/// The string `"dir\n\tsubdir1\n\t\tfile1.ext\n\t\tsubsubdir1\n\tsubdir2\n\t\tsubsubdir2\n\t\t\tfile2.ext"` represents:
///
/// ```
/// dir
///     subdir1
///         file1.ext
///         subsubdir1
///     subdir2
///         subsubdir2
///             file2.ext
/// ```
///
/// The directory `dir` contains two sub-directories `subdir1` and `subdir2`. `subdir1` contains a file `file1.ext`
/// and an empty second-level sub-directory `subsubdir1`. `subdir2` contains a second-level sub-directory
/// `subsubdir2` containing a file `file2.ext`.
///
/// We are interested in finding the longest (number of characters) absolute path to a file within our file system.
/// For example, in the second example above, the longest absolute path is `"dir/subdir2/subsubdir2/file2.ext"`,
/// and its length is `32` (not including the double quotes).
///
/// Given a string representing the file system in the above format, return the length of the longest absolute path to
/// a file in the abstracted file system. If there is no file in the system, return 0.
///
/// Note:
/// The name of a file contains at least a period and an extension.
/// The name of a directory or sub-directory will not contain a period.
///
pub fn solve(file_system: &str) -> usize {
    let extract_level = |byte| byte != b'\t';
    let extract_element = |byte| byte == b'\n';

    let mut stack = Vec::with_capacity(5);
    let mut max_path = 0;
    let mut offset = 0;

    loop {
        let level = extract_substring(file_system, offset, extract_level).len();
        let element = extract_substring(file_system, offset + level, extract_element);
        offset += level + element.len() + 1;

        if stack.len() < level { panic!("missing folder name in filesystem"); }

        if is_file(element) {
            let path_len = stack.iter().sum::<usize>() + element.len();
            max_path = max_path.max(path_len);

        } else {
            if stack.len() > level { stack.drain(level..); }
            stack.push(element.len() + 1)
        }

        if offset >= file_system.len() { break; }
    }

    max_path
}

#[inline(always)]
fn extract_substring<'str>(string: &'str str, offset: usize, predicate: impl Fn(u8) -> bool) -> &'str str {
    let iter = string.as_bytes().iter().enumerate().skip(offset);

    for (idx, &byte) in iter {
        if predicate(byte) {
            return &string[offset..idx];
        }
    }

    &string[offset..]
}

#[inline(always)]
pub fn is_file(element: &str) -> bool {
    element.as_bytes().iter().any(|&byte| byte == b'.')
}
