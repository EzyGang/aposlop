[
  (function_declaration)
  (generator_function_declaration)
  (method_definition)
  (function_expression)
  (generator_function)
  (arrow_function)
] @block

[
  (identifier)
  (property_identifier)
  (shorthand_property_identifier)
  (shorthand_property_identifier_pattern)
] @anonymize.identifier

[
  (number)
  (string)
  (template_string)
  (regex)
  (true)
  (false)
  (null)
] @anonymize.literal

(comment) @ignore
