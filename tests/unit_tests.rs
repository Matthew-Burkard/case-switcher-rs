#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use case_switcher::{to_camel, to_dot, capitalize, get_words};

    #[test]
    fn test_get_words() {
        let words = get_words("PascalCase_snake_case_threeCamelCase-kebab-caseJSONWords.dot.case");
        assert_eq!(words[0], "Pascal");
        assert_eq!(words[1], "Case");
        assert_eq!(words[2], "snake");
        assert_eq!(words[3], "case");
        assert_eq!(words[4], "three");
        assert_eq!(words[5], "Camel");
        assert_eq!(words[6], "Case");
        assert_eq!(words[7], "kebab");
        assert_eq!(words[8], "case");
        assert_eq!(words[9], "JSON");
        assert_eq!(words[10], "Words");
        assert_eq!(words[11], "dot");
        assert_eq!(words[12], "case");

        let words = get_words("JSON1Jelly23Kebab");
        assert_eq!(words[0], "JSON1");
        assert_eq!(words[1], "Jelly23");
        assert_eq!(words[2], "Kebab");
    }

    #[test]
    fn test_capitalize() {
        assert_eq!(capitalize("coffee"), "Coffee");
        assert_eq!(capitalize("Coffee"), "Coffee");
        assert_eq!(capitalize("COFFEE"), "COFFEE");
    }

    #[test]
    fn test_switchers() {
        let mixed_sample = "avocado bagel-coffeeDONUTEclair_food.gravy/honey";
        let lone_word_sample = "Honey";
        let lone_upper_word_sample = "ICING";

        // camelCase
        test_switcher_fn(
            to_camel,
            HashMap::from(
                [
                    (mixed_sample, String::from("avocadoBagelCoffeeDONUTEclairFoodGravyHoney")),
                    (lone_word_sample, String::from("Honey")),
                    (lone_upper_word_sample, String::from("ICING")),
                ]
            ),
        );

        // dot.case
        test_switcher_fn(
            to_dot,
            HashMap::from(
                [
                    (mixed_sample, String::from("avocado.bagel.coffee.donut.eclair.food.gravy.honey")),
                    (lone_word_sample, String::from("honey")),
                    (lone_upper_word_sample, String::from("icing")),
                ]
            ),
        );
    }

    fn test_switcher_fn<F>(fun: F, samples: HashMap<&str, String>)
        where F: Fn(&str) -> String
    {
        assert_eq!(fun(""), "");
        for (sample, expected) in &samples {
            assert_eq!(&fun(sample), expected);
        }
    }
}
