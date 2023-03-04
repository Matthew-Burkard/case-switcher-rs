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
                    (mixed_sample, "avocadoBagelCoffeeDONUTEclairFoodGravyHoney"),
                    (lone_word_sample, "Honey"),
                    (lone_upper_word_sample, "ICING"),
                ]
            ),
        );

        // dot.case
        test_switcher_fn(
            to_dot,
            HashMap::from(
                [
                    (mixed_sample, "avocado.bagel.coffee.donut.eclair.food.gravy.honey"),
                    (lone_word_sample, "honey"),
                    (lone_upper_word_sample, "icing"),
                ]
            ),
        );

        // kebab-case
        test_switcher_fn(
            to_kebab,
            HashMap::from(
                [
                    (mixed_sample, "avocado-bagel-coffee-donut-eclair-food-gravy-honey"),
                    (lone_word_sample, "honey"),
                    (lone_upper_word_sample, "icing"),
                ]
            ),
        );

        // PascalCase
        test_switcher_fn(
            to_pascal,
            HashMap::from(
                [
                    (mixed_sample, "AvocadoBagelCoffeeDONUTEclairFoodGravyHoney"),
                    (lone_word_sample, "Honey"),
                    (lone_upper_word_sample, "ICING"),
                ]
            ),
        );

        // path/case
        test_switcher_fn(
            to_path,
            HashMap::from(
                [
                    (mixed_sample, "avocado/bagel/coffee/donut/eclair/food/gravy/honey"),
                    (lone_word_sample, "honey"),
                    (lone_upper_word_sample, "icing"),
                ]
            ),
        );

        // snake_case
        test_switcher_fn(
            to_snake,
            HashMap::from(
                [
                    (mixed_sample, "avocado_bagel_coffee_donut_eclair_food_gravy_honey"),
                    (lone_word_sample, "honey"),
                    (lone_upper_word_sample, "icing"),
                ]
            ),
        );

        // Title Case
        test_switcher_fn(
            to_title,
            HashMap::from(
                [
                    (mixed_sample, "Avocado Bagel Coffee DONUT Eclair Food Gravy Honey"),
                    (lone_word_sample, "Honey"),
                    (lone_upper_word_sample, "ICING"),
                ]
            ),
        );
    }

    #[test]
    fn readme_demo() {
        let sample = "avocado bagel-coffeeDONUTEclair_food.gravy";

        let result = to_camel(sample);
        assert_eq!(result, "avocadoBagelCoffeeDONUTEclairFoodGravy");

        let result = to_dot(sample);
        assert_eq!(result, "avocado.bagel.coffee.donut.eclair.food.gravy");

        let result = to_kebab(sample);
        assert_eq!(result, "avocado-bagel-coffee-donut-eclair-food-gravy");

        let result = to_pascal(sample);
        assert_eq!(result, "AvocadoBagelCoffeeDONUTEclairFoodGravy");

        let result = to_path(sample);
        assert_eq!(result, "avocado/bagel/coffee/donut/eclair/food/gravy");

        let result = to_snake(sample);
        assert_eq!(result, "avocado_bagel_coffee_donut_eclair_food_gravy");

        let result = to_title(sample);
        assert_eq!(result, "Avocado Bagel Coffee DONUT Eclair Food Gravy");
    }

    fn test_switcher_fn<F>(fun: F, samples: HashMap<&str, &str>)
        where F: Fn(&str) -> String
    {
        assert_eq!(fun(""), "");
        for (sample, expected) in samples {
            assert_eq!(fun(sample), String::from(expected));
        }
    }
}
