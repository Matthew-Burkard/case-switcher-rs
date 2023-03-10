use regex::Regex;


/// Return a version of the string in `camelCase` format.
///
/// # Arguments
///
/// * `string` - The string to get a camelCase version of.
///
/// # Examples
///
/// ```
/// let result = case_switcher::to_camel("sample_string");
/// assert_eq!(result, "sampleString");
/// ```
pub fn to_camel(string: &str) -> String {
    let words = get_words(string);
    let mut first_word = String::new();
    if let Some(word) = words.first() {
        first_word = if word.chars().next().unwrap().is_uppercase() {
            word.to_owned()
        } else {
            word.to_lowercase()
        };
    }
    let remaining_words = words.iter().skip(1);
    let mut result = String::new();
    result.push_str(&first_word);
    result.push_str(&remaining_words.map(|s| capitalize(s)).collect::<String>());
    result
}


/// Return a version of the string in `dot.case` format.
///
/// # Arguments
///
/// * `string` - The string to get a dot.case version of.
///
/// # Examples
///
/// ```
/// let result = case_switcher::to_dot("sample_string");
/// assert_eq!(result, "sample.string");
/// ```
pub fn to_dot(string: &str) -> String {
    lower_join(string, ".")
}


/// Return a version of the string in `kebab-case` format.
///
/// # Arguments
///
/// * `string` - The string to get a kebab-case version of.
///
/// # Examples
///
/// ```
/// let result = case_switcher::to_kebab("sample_string");
/// assert_eq!(result, "sample-string");
/// ```
pub fn to_kebab(string: &str) -> String {
    lower_join(string, "-")
}


/// Return a version of the string in `PascalCase` format.
///
/// # Arguments
///
/// * `string` - The string to get a PascalCase version of.
///
/// # Examples
///
/// ```
/// let result = case_switcher::to_pascal("sample_string");
/// assert_eq!(result, "SampleString");
/// ```
pub fn to_pascal(string: &str) -> String {
    get_words(string)
        .iter()
        .map(|s| capitalize(s))
        .collect::<String>()
}


/// Return a version of the string in `path/case` format.
///
/// # Arguments
///
/// * `string` - The string to get a path/case version of.
///
/// # Examples
///
/// ```
/// let result = case_switcher::to_path("sample_string");
/// assert_eq!(result, "sample/string");
/// ```
pub fn to_path(string: &str) -> String {
    lower_join(string, "/")
}


/// Return a version of the string in `snake_case` format.
///
/// # Arguments
///
/// * `string` - The string to get a snake_case version of.
///
/// # Examples
///
/// ```
/// let result = case_switcher::to_snake("sampleString");
/// assert_eq!(result, "sample_string");
/// ```
pub fn to_snake(string: &str) -> String {
    lower_join(string, "_")
}


/// Return a version of the string in `Title Case` format.
///
/// # Arguments
///
/// * `string` - The string to get a Title Case version of.
///
/// # Examples
///
/// ```
/// let result = case_switcher::to_title("sample_string");
/// assert_eq!(result, "Sample String");
/// ```
pub fn to_title(string: &str) -> String {
    get_words(string)
        .iter()
        .map(|s| capitalize(s))
        .collect::<Vec<String>>()
        .join(" ")
}


/// Get all of the words in a string.
///
/// # Arguments
///
/// * `string` - The string to get words from.
///
/// # Examples
///
/// ```
/// let result = case_switcher::get_words("sample_string");
/// assert_eq!(result, vec!["sample", "string"]);
/// ```
pub fn get_words(string: &str) -> Vec<String> {
    // Split on word boundaries and underscores
    let re = Regex::new(r"(.*?)[\W_]").unwrap();
    let words = re.replace_all(string, "$1 $3");

    // Split on lower then upper: "oneTwo" -> ["one", "Two"]
    let re = Regex::new(r"([a-z])([A-Z])").unwrap();
    let words = re.replace_all(&words, "$1 $2");

    // Split on upper then upper + lower: "JSONWord" -> ["JSON", "Word"]
    let re = Regex::new(r"([A-Z])([A-Z])([a-z])").unwrap();
    let words = re.replace_all(&words, "$1 $2$3");

    // Split on number + letter: "TO1Cat23dog" -> ["TO1", "Cat23", "dog"]
    let re = Regex::new(r"(\d)([A-Za-z])").unwrap();
    let words = re.replace_all(&words, "$1 $2");
    words.split_whitespace().map(|s| s.to_string()).collect::<Vec<String>>()
}


/// Return a version of the string with the first letter capitalized.
///
/// # Arguments
///
/// * `string` - The string to get a capitalized version of.
///
/// # Examples
///
/// ```
/// let result = case_switcher::capitalize("sample_string");
/// assert_eq!(result, "Sample_string");
/// ```
pub fn capitalize(string: &str) -> String {
    let mut chars = string.chars();
    if let Some(first_char) = chars.next() {
        let capitalized = first_char.to_uppercase().collect::<String>();
        capitalized + chars.as_str()
    } else {
        String::new()
    }
}

fn lower_join(string: &str, join_string: &str) -> String {
    get_words(string)
        .into_iter()
        .map(|w| w.to_lowercase())
        .collect::<Vec<String>>()
        .join(join_string)
}
