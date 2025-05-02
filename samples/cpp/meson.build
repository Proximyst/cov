project('sample', 'cpp')
add_global_arguments('-Db_coverage=true', language: 'cpp')
s = library('sample', 'sample.cpp')

test('simple unit test', executable('simple_unit_test', 'sample_test.cpp', link_with: s))
