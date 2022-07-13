
{% for k, o in cffi_unique_ptrs.items() -%}
{{ cffi_genrs_unique_ptr(k, **o) }}
{% endfor %}

{% for k, o in cffi_shared_ptrs.items() -%}
{{ cffi_genrs_shared_ptr(k, **o) }}
{% endfor %}
