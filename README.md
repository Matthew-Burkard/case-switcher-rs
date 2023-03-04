# Case Switcher

[![Crates.io](https://img.shields.io/crates/v/case-switcher.svg)](https://crates.io/crates/case-switcher)
[![Apache 2.0](https://img.shields.io/badge/license-MIT%2FApache-blue.svg)](https://gitlab.com/mburkard/case-switcher-rs/LICENSE.md)

This library provides functions to change the case convention of a string.

Supported cases:

- camelCase
- dot.case
- kebab-case
- PascalCase
- path/case
- snake_case
- Title Case

### Install

```shell
cargo add case-switcher
```

### Demo

```rust
use case_switcher as cs;

fn main() {
    let sample = "avocado bagel-coffeeDONUTEclair_food.gravy";

    let result = cs::to_camel(sample);
    assert_eq!(result, "avocadoBagelCoffeeDONUTEclairFoodGravy");

    let result = cs::to_dot(sample);
    assert_eq!(result, "avocado.bagel.coffee.donut.eclair.food.gravy");

    let result = cs::to_kebab(sample);
    assert_eq!(result, "avocado-bagel-coffee-donut-eclair-food-gravy");

    let result = cs::to_pascal(sample);
    assert_eq!(result, "AvocadoBagelCoffeeDONUTEclairFoodGravy");

    let result = cs::to_path(sample);
    assert_eq!(result, "avocado/bagel/coffee/donut/eclair/food/gravy");

    let result = cs::to_snake(sample);
    assert_eq!(result, "avocado_bagel_coffee_donut_eclair_food_gravy");

    let result = cs::to_title(sample);
    assert_eq!(result, "Avocado Bagel Coffee DONUT Eclair Food Gravy");
}
```

## Support The Developer

<a href="https://www.buymeacoffee.com/mburkard" target="_blank">
  <img src="https://cdn.buymeacoffee.com/buttons/v2/default-blue.png"
       width="217"
       height="60"
       alt="Buy Me A Coffee">
</a>
