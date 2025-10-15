flisp language specification
============================

Routines
--------

At the top level there must be a *routine directory*, containing exactly one
expression that is used as the return value.


Expressions
-----------

Expressions may be one of the following:

 * **File with a literal**:
   For files with a literal as the name, the name is used as the expression's
   value. The following literals are recognised:
    - **Integers** (see section *Values* below for the format).
    - **Booleans** (see section *Values* below for the format).
    - **Strings**: Any sequence of characters enclosed between single (`'`) or
      double (`"`) quotes. Escape characters are not supported.

   File names not matching any of these will be treated as an identifier unless
   they do not match in one of the following ways (which triggers a syntax
   error):
    - Integer literal followed by invalid characters for integer values.
    - Strings without a closing quote character.
    - Strings with characters after the closing quote character.

 * **File with a non-reserved identifier**:
    - **Regular file**: The file's content is used as the expression's value.
    - **Directory**: The directory is processed as a routine, and its return
      value is used as the expression's value.
    - **Symbolic link to a file**: The referenced file's content is used as the
      expression's value.
    - **Symbolic link to a directory**: The referenced directory is processed as
      a routine, and its return value is used as the expression's value.
    - **Symbolic link to a literal**: The referenced path is used as a literal
      value.

   Note that with the exception of expressions defined inside a `with` (see
   below), non-reserved file names usually do not matter.

 * **File with a reserved identifier**:
   The file, directory or symbolic link is processed as a *language builtins*,
   and its return value is used as the expression's value. Language builtins
   (and thus reserved identifier names) are listed and described in the section
   *Builtings* below.


Values
------

### Scalar

A scalar is a singular value. Recognised types for scalars are:

 * **Integer**: A numeric value with an optional sign (`+` or `-`) in front,
   represented in either octal (`[-+]?0[0-7]*`), decimal (`[-+][1-9][0-9]*`) or
   hexadecimal (`[-+]?0x[0-9a-fA-F]+`).
 * **Boolean**: Either `true` or `false`. See below for handling non-boolean
   values in boolean contexts (e.g. for checking a condition).
 * **String**: Any other sequence of characters.

Where the type is relevant, it is inferred from the value based on this pattern.

Integer and boolean values may also be used as literals (see *Expressions*
below), whereas strings must be enclosed in single (`'`) or double (`"`) quotes.

In boolean contexts (e.g. the condition for an `if` expression), integers are
treated as false if their value is 0 and true otherwise, and strings are treated
as false if their length is 0 and true otherwise.

### List

A list is an ordered sequences of scalar or list values.

Since we cannot represent lists in a filesystem, to build a new list from
scratch, the following structure is required (example of list with three
elements):

    /list
     + head
     + /tail
        + head
        + /tail
           + head

Each `head` expression here is a **[TODO]**


Contexts and runtime memory
---------------------------

Runtime data (the *context*) is managed in a temporary directory, chosen by the
interpreter. It contains all variables (files or file symlinks with non-reserved
identifiers) known to the given expression.

Context data is manipulated as follows:

 * Whenever a `with` expression is evaluated, a new context is created, all
   variables of the current context are symlinked into the newly created
   context, and any new variables defined with `with` are added to that context.

 * Whenever any other expression is evaluated, the current context directory
   itself is symlinked to the expression's context.

Since the runtime directory is not known when creating a flisp program, one can
refer to them with a (dangling) symlink from `run:somevar`, which the
interpreter will resolve to `somevar` in the current context.


Builtins
--------

The following builtins are recognised:

### `with` (directory)

Creates a new context with some variables defined, and evaluates a
sub-expression.

The structure of a `with` directory is as follows:

 * One or more named expressions where the name is made available to the
   sub-expression (`do`, see below) and its children. No order of evaluation is
   guaranteed; if a specific order is desired, `with` expressions must be
   nested.

 * Exactly one expression named `do`, whose return value is used as the value of
   the `with` expression.

Example:

    /with
     + a
     + b -> run:foobar
     + /do
        + /add
           + a
           + b
           + some_expr -> ../../../barbaz

### `if` (directory)

Evaluates one of two expressions based on a condition expression.

The directory contains exactly three expressions named as follows:

 * `cond`: Expression that evaluates to either true or false.
 * `then`: Expression that is evaluated if `cond` returns true.
 * `else`: Expression that is evaluated if `cond` returns false.

Example:

    /if
     + /cond
        + /with

### `match` (directory)

Compares an expression against one or more patterns, and evaluates the
expression for the matching pattern.

The expression to be compoared against the patterns is expected to be defined in
the current context as `expr`.

The directory contains any number of expressions that will be evaluated in no
particular order, and whose values will be compared against the expression. The
matching expression's **[TODO]**

 * `expr`

### `list` (directory)

*TODO*

### `concat` (directory)

*TODO*


Tail recursion
--------------

Given the language's mostly static and functional nature, recursive routines are
likely a common pattern.

To avoid nesting paths too deply, an interpreter shall resolve a symlink's
source before changing to that path (instead of just following the symlink) in
the following cases:

 * For `do` in a `with` expression.
 * For `then` and `else` in an `if` expression.
 * For any matched case in a `match` expression.
 * For any simple routine call.

This does not apply to regular files.

Furthermore, the runtime shall be adapted as follows upon performing a tail
recursion:

 * Interpreters that invoke subprocesses for evaluating sub-expressions shall
   replace themselves (i.e. `exec`) instead of launching a subprocess.
 * The current context (see *Contexts and runtime memory* above) shall not be
   symlinked into a new context, but instead the current context shall be
   reused.
