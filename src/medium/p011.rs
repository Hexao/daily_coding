/// Implement an autocomplete system. That is, given a query string `s`
/// and a set of all possible query strings, return all strings in the
/// set that have s as a prefix.
///
/// For example, given the query string `de` and the set of strings
/// `[dog, deer, deal]`, return `[deer, deal]`.
///
/// Hint: Try preprocessing the dictionary into a more efficient data
/// structure to speed up queries.
pub fn find_in<'word>(dico: &[&'word str], mask: &str) -> Vec<&'word str> {
    dico.iter()
        .filter(|word| word.starts_with(mask))
        .map(|word| *word)
        .collect()
}
