#!/usr/bin/env python3
"""
Spits out tests to dump into lib.rs for each file in the schema-examples directory.
"""


import re
from pathlib import Path

schema_dir = Path(__file__).with_name("schema-examples")


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
    for file in schema_dir.glob("*.json"):
        title_name = file.name.removesuffix(".json")
        snake_name = title_to_snake(title_name)
        print(
            "json_deserialize_test!(test_",
            snake_name,
            "_schema, schemas::",
            title_name,
            ', "schema-examples/',
            file.name,
            '");',
            sep="",
        )

if __name__ == "__main__": main()