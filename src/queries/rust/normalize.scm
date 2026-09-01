[
  (function_item)
  (closure_expression)
] @block

[
  (identifier)
  (field_identifier)
  (shorthand_field_identifier)
  (type_identifier)
] @anonymize.identifier

[
  (integer_literal)
  (float_literal)
  (string_literal)
  (raw_string_literal)
  (char_literal)
  (boolean_literal)
] @anonymize.literal

(line_comment) @ignore
(block_comment) @ignore
