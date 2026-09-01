(function_item) @block

(identifier) @anonymize.identifier

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
