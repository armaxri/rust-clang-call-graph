@echo off

call cd tests\playground\external_resources

call git.exe clone https://github.com/google/googletest.git
call cd googletest
call git.exe checkout v1.15.2
