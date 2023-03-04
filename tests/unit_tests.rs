#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use case_switcher::{
        capitalize, get_words, to_camel, to_dot, to_kebab, to_pascal, to_path, to_snake, to_title
    };

    #[test]
    fn test_get_words() {
        let words = get_words("PascalCase_snake_case_threeCamelCase-kebab-caseJSONWords.dot.case");
        assert_eq!(
            words,
            vec![
                "Pascal",
                "Case",
                "snake",
                "case",
                "three",
                "Camel",
                "Case",
                "kebab",
                "case",
                "JSON",
                "Words",
                "dot",
                "case",
            ]
        );
        assert_eq!(get_words("JSON1Jelly23Kebab"), vec!["JSON1", "Jelly23", "Kebab"]);
    }

    #[test]
    fn test_capitalize() {
        assert_eq!(capitalize("coffee"), "Coffee");
        assert_eq!(capitalize("Coffee"), "Coffee");
        assert_eq!(capitalize("COFFEE"), "COFFEE");
        assert_eq!(capitalize(""), "");
        let result = capitalize("sample_string");
        assert_eq!(result, "Sample_string");
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

        // kebab-case
        test_switcher_fn(
            to_kebab,
            HashMap::from(
                [
                    (mixed_sample, String::from("avocado-bagel-coffee-donut-eclair-food-gravy-honey")),
                    (lone_word_sample, String::from("honey")),
                    (lone_upper_word_sample, String::from("icing")),
                ]
            ),
        );

        // PascalCase
        test_switcher_fn(
            to_pascal,
            HashMap::from(
                [
                    (mixed_sample, String::from("AvocadoBagelCoffeeDONUTEclairFoodGravyHoney")),
                    (lone_word_sample, String::from("Honey")),
                    (lone_upper_word_sample, String::from("ICING")),
                ]
            ),
        );

        // path/case
        test_switcher_fn(
            to_path,
            HashMap::from(
                [
                    (mixed_sample, String::from("avocado/bagel/coffee/donut/eclair/food/gravy/honey")),
                    (lone_word_sample, String::from("honey")),
                    (lone_upper_word_sample, String::from("icing")),
                ]
            ),
        );

        // snake_case
        test_switcher_fn(
            to_snake,
            HashMap::from(
                [
                    (mixed_sample, String::from("avocado_bagel_coffee_donut_eclair_food_gravy_honey")),
                    (lone_word_sample, String::from("honey")),
                    (lone_upper_word_sample, String::from("icing")),
                ]
            ),
        );

        // Title Case
        test_switcher_fn(
            to_title,
            HashMap::from(
                [
                    (mixed_sample, String::from("Avocado Bagel Coffee DONUT Eclair Food Gravy Honey")),
                    (lone_word_sample, String::from("Honey")),
                    (lone_upper_word_sample, String::from("ICING")),
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
