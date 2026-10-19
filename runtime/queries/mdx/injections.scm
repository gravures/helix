; Markdown

(fenced_code_block
  (code_fence_content) @injection.shebang @injection.content
  (#set! injection.include-unnamed-children))
  
(fenced_code_block
  (info_string
    (language) @injection.language)
  (code_fence_content) @injection.content (#set! injection.include-unnamed-children))

(document . (section . (thematic_break) (_) @injection.content (thematic_break)) (#set! injection.language "yaml") (#set! injection.include-unnamed-children))

((minus_metadata) @injection.content (#set! injection.language "yaml") (#set! injection.include-unnamed-children))

((plus_metadata) @injection.content (#set! injection.language "toml") (#set! injection.include-unnamed-children))

((markdown_inline) @injection.content (#set! injection.language "markdown.inline") (#set! injection.include-unnamed-children))

; JavaScript

; Parse the contents of tagged template literals using
; a language inferred from the tag.
(call_expression
  function: [
    (identifier) @injection.language
    (member_expression
      property: (property_identifier) @injection.language)
  ]
  arguments: (template_string (string_fragment) @injection.content)
  (#set! injection.combined)
  (#set! injection.include-children))


; Parse regex syntax within regex literals
((regex_pattern) @injection.content
 (#set! injection.language "regex"))

; Parse Ember/Glimmer/Handlebars/HTMLBars/etc. template literals
; e.g.: await render(hbs`<SomeComponent />`)
; (call_expression
;   function: ((identifier) @_name
;              (#eq? @_name "hbs"))
;   arguments: ((template_string) @glimmer
;               (#offset! @glimmer 0 1 0 -1)))
