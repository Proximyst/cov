project('sample', 'c')
add_global_arguments('-Db_coverage=true', language: 'c')
s = library('sample', ['sample.c', 'helpers.c'])

test('simple unit test', executable('simple_unit_test', 'sample_test.c', link_with: s))
