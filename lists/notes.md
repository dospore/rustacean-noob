Some little notes that I liked when reading through this
- enum null pointer optimisations (with 2 elements in an enum one of the elements tags can be the null pointer since the other elements tag is not that). This can save on space
- Option::take() can be used like mem::replace(x, None)
- Option::map() replaces matching and doing something like `match option { None => None, Some(x) => Some(y) }`
