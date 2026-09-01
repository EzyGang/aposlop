[
  (if_statement)
  (for_statement)
  (for_in_statement)
  (while_statement)
  (do_statement)
  (switch_case)
  (catch_clause)
  (ternary_expression)
  (assignment_pattern)
  (object_assignment_pattern)
  (optional_chain)
] @complexity

(binary_expression
  operator: ["&&" "||" "??"] @complexity)

(augmented_assignment_expression
  operator: ["&&=" "||=" "??="] @complexity)

(required_parameter
  value: (_) @complexity)

(optional_parameter
  value: (_) @complexity)
