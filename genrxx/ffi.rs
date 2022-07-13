
{% for k, o in cffi_unique_ptrs.items() -%}
{{ cffi_genrs_unique_ptr(k, **o) }}
{% endfor %}
