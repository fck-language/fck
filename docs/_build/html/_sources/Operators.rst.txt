.. role:: warning

Operators
=========

Assignment operators
--------------------

fck uses two main assignment operators, both of which do the same thing, but one return the assigned value.

- ``::`` is the simplest assignment operator ans assigns the given value to the given variable
- ``:>`` is similar to the assignment operator ``::``, but returns the assigned value

Both operators can be used inline with other code. For example: ::

    print(int a :: 1)
    print(int b :> 3)

when run would return: ::

    None
    3

Numerical operators
-------------------

+---------+-----------------------------------------+------------+--------------------------------+
|Operator | Description                             |Example     |Result                          |
+---------+-----------------------------------------+------------+--------------------------------+
|``+``    | Add two values together                 | ``a + b``  | :math:`a + b`                  |
+---------+-----------------------------------------+------------+--------------------------------+
|``-``    | Subtract one value from another         | ``a - b``  | :math:`a - b`                  |
+---------+-----------------------------------------+------------+--------------------------------+
|``*``    | Multiply two values by one another      | ``a * b``  | :math:`a \times b`             |
+---------+-----------------------------------------+------------+--------------------------------+
|``**``   | Raise one value to the power of the     | ``a ** b`` | :math:`a^b`                    |
|         | other [#]_                              |            |                                |
+---------+-----------------------------------------+------------+--------------------------------+
|``\``    | Divide one value by another             | ``a / b``  | :math:`a \div b`               |
+---------+-----------------------------------------+------------+--------------------------------+
|``\\``   | Divide one value by another and floor   | ``a // b`` |:math:`\lfloor a \div b\rfloor` |
|         | the result                              |            |                                |
+---------+-----------------------------------------+------------+--------------------------------+
|``%``    | Remainder of the division of two values | ``a % b``  |                                |
+---------+-----------------------------------------+------------+--------------------------------+

.. [#] fck defines :math:`0^0` as 1

fck has been designed to follow the rules of maths in the order of operations, and so ``1 + 2 * 3`` will return ``7``, and not ``9`` as would be the case for ``(1 + 2) * 3``.

Division by zero
^^^^^^^^^^^^^^^^

When dividing, for floor dividing, by zero, fck will return infinity. :warning:`This will raise a warning.` However, fck will attempt to preserve as much information as possible, and so the value of infinity returned will contain a the value that was divided by zero. This value is then used in other numerical operations.

Similarly, ``a % 0`` is defined as 0.

Wrapped operators
-----------------

All of the numerical operators can be wrapped inside one of the two assignment operators (``::`` and ``:>``) to perform an operation on a variable and reassign the variable.

For example: ::

    >>> int a :: 12
    >>> a :> a + 3
    15

Can be simplified to: ::

    >>> int a :: 12
    >>> a :+> 3
    15

The given value is always considered as the second value, and so ``a :/: b`` would assign ``a`` to ``a / b``.
