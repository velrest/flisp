# flisp
I swear im not mad.

Last executed statement of execution block `run` or function is return value

This is as of now only a spec and no interpreter is implemented.

Builtins:
- `match any [case]` define a case tree
- `case any` match given value to first file
- `lib` place to define functions, function names dont need index
- `run` Just run the contained code
- `any` Any value for matching
- `arg[index]` access the function argument at index `num`
- `set [variable_name] any -> any` set a variable in the current `run` context
- `set_global [variable_name] any -> any` set a variable in the current `start` context
- `read_io -> text` read input from user
- `write_io -> nothing` write ouput
- `*` `-` `+` `div` all `-> number` operators `div` since `/` is not allowed in file names
- `slash -> "/"`  the representation for `/` to match input to
- `call -> any` call a function defined in lib with arguments
- `nothing` pretty obvious
- `concat -> text` combine text

## Memory management
Variables are symlinked to `mem/your_var_name`.
Current idea is to implement an interpreter which will write the actual value of the variable into the symlinked file.
So therefore, every data you can put in a file, we support.

Static variables are created in `static/your_var_name` with the static value as file content.
