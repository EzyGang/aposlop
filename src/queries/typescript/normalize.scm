[
  (function_declaration)
  (generator_function_declaration)
  (method_definition)
  (function_expression)
  (generator_function)
  (arrow_function)
  (class_static_block)
] @block

(public_field_definition
  value: (_) @block)

[
  (identifier)
  (private_property_identifier)
  (property_identifier)
  (shorthand_property_identifier)
  (shorthand_property_identifier_pattern)
  (statement_identifier)
  (type_identifier)
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
