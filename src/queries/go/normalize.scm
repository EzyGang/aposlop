[
  (function_declaration)
  (method_declaration)
  (func_literal)
] @block

[
  (identifier)
  (field_identifier)
  (label_name)
  (package_identifier)
  (type_identifier)
] @anonymize.identifier

[
  (int_literal)
  (float_literal)
  (imaginary_literal)
  (rune_literal)
  (raw_string_literal)
  (interpreted_string_literal)
  (nil)
  (true)
  (false)
] @anonymize.literal

(comment) @ignore
