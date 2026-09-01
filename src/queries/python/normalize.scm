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
