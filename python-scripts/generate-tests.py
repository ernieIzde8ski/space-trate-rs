#!/usr/bin/env python3
"""
Spits out tests to dump into lib.rs for each file in the schema-examples directory.
"""


import re
from pathlib import Path

root = Path(__file__).parent.parent
schema_dir = root / "schema-examples"
target_file = root / "src" / "tests" / "schema.rs"


start = r"""use crate::api::schemas;

macro_rules! json_deserialize_test {
    ($name:ident, $struct_type:ty, $file_path:expr) => {
        #[test]
        fn $name() {
            let string = std::fs::read_to_string($file_path).unwrap();
            serde_json::from_str::<$struct_type>(&string).unwrap();
        }
    };
}"""


def title_to_snake(s: str, /) -> str:
    """TitleCase -> snake_case"""
    # delete non-word characters -> prefix capital letters with underscores
    # -> lowercase -> remove leading underscore (if exists)
    s = re.sub(r"\W+", "", s)
    s = re.sub(r"([A-Z]+)", r"_\1", s).lower().lstrip("_")
    return s.lstrip("_")


def main():
    # iterate through files and match by their names
    # it should output a bunch of lines looking like the following:
    #   json_deserialize_test!(test_contract_schema,schemas::Contract,"schema-examples/Contract.json");

    file = open(target_file, "w+", encoding="utf-8")
    file.write(start)
    file.write("\n\n")
    for path in schema_dir.glob("*.json"):
        title_name = path.name.removesuffix(".json")
        snake_name = title_to_snake(title_name)
        print(
            "json_deserialize_test!(test_",
            snake_name,
            "_schema, schemas::",
            title_name,
            ', "schema-examples/',
            path.name,
            '");',
            sep="",
            file=file,
        )


if __name__ == "__main__":
    main()
