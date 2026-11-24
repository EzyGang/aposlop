[
  (module
    (function_definition) @block)
  (block
    (function_definition) @block)
  (module
    (class_definition) @block)
  (block
    (class_definition) @block)
  (decorated_definition
    definition: (function_definition)) @block
  (decorated_definition
    definition: (class_definition)) @block
  (lambda) @block
]

[
  (function_definition
    body: (block) @sequence)
  (if_statement
    consequence: (block) @sequence)
  (elif_clause
    consequence: (block) @sequence)
  (else_clause
    body: (block) @sequence)
  (for_statement
    body: (block) @sequence)
  (while_statement
    body: (block) @sequence)
  (try_statement
    body: (block) @sequence)
  (except_clause
    (block) @sequence)
  (finally_clause
    (block) @sequence)
  (with_statement
    body: (block) @sequence)
  (case_clause
    consequence: (block) @sequence)
]

(identifier) @anonymize.identifier

[
  (integer)
  (float)
  (string)
  (concatenated_string)
  (true)
  (false)
  (none)
  (ellipsis)
] @anonymize.literal

(comment) @ignore
