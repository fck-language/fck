Built-in types
==============

This page details the different built in types within fck. For the different types of operators used by fck, please see the :doc:`operators </Operators>` page

``int`` type
------------
::

    int <identifier> ((::|:>) <value>)?

The ``int`` variable type inherits from the ``Number`` class, and holds a single integer value. It has a default value of 0. When a value is assigned to an ``int`` type variable or cast as an ``int`` using the ``at`` keyword, the value sometimes has to be preprocessed to form an integer if it is not already. The table below details the different preprocessing that occurs for different value types:

+---------------+------------------------------------------------------------+--------------------------+
| Value type    | Preprocessing                                              | Warning or error raised  |
+---------------+------------------------------------------------------------+--------------------------+
| ``int``       | None                                                       | None                     |
+---------------+------------------------------------------------------------+--------------------------+
| ``float``     | Value is rounded                                           | None                     |
+---------------+------------------------------------------------------------+--------------------------+
| ``list``      || **For a list with a single element in recursively:**      | :ref:`W014<warnings>`    |
|               || The single element is removed and processed depending on  |                          |
|               | its type                                                   |                          |
|               +------------------------------------------------------------+--------------------------+
|               || **For all other lists:**                                  | :ref:`E000<errors>`      |
|               || An error is raised                                        |                          |
+---------------+------------------------------------------------------------+--------------------------+
| ``str``       | fck will attempt to convert the string into a base 10      || Warning:                |
|               | integer, or float and round the value. If it is not        | :ref:`W016<warnings>`    |
|               | successful, an error is raised. If it is successful, a     || Error:                  |
|               | warning is raised.                                         | :ref:`E000<errors>`      |
+---------------+------------------------------------------------------------+--------------------------+

``float`` type
--------------
::

    float <identifier> ((::|:>) <value>)?

The ``float`` variable type inherits from the ``Number`` class, and holds a single floating point value. It has a default value of 0.0. When a value is assigned to a ``float`` type variable or cast as a ``float`` using the ``at`` keyword, the value sometimes has to be preprocessed to form a floating point value if it is not already. The table below details the different preprocessing that occurs for different value types:

+---------------+------------------------------------------------------------+--------------------------+
| Value type    | Preprocessing                                              | Warning or error raised  |
+---------------+------------------------------------------------------------+--------------------------+
| ``int``       | Converts integer value to floating point value             | None                     |
+---------------+------------------------------------------------------------+--------------------------+
| ``float``     | None                                                       | None                     |
+---------------+------------------------------------------------------------+--------------------------+
| ``list``      || **For a list with a single element in recursively:**      | :ref:`W014<warnings>`    |
|               || The single element is removed and processed depending on  |                          |
|               | its type                                                   |                          |
|               +------------------------------------------------------------+--------------------------+
|               || **For all other lists:**                                  | :ref:`E000<errors>`      |
|               || An error is raised                                        |                          |
+---------------+------------------------------------------------------------+--------------------------+
| ``str``       | fck will attempt to convert the string into a base 10      || Warning:                |
|               | float. If it is not                                        | :ref:`W016<warnings>`    |
|               | successful, an error is raised. If it is successful, a     || Error:                  |
|               | warning is raised                                          | :ref:`E000<errors>`      |
+---------------+------------------------------------------------------------+--------------------------+

.. _list documentation:

``list`` type
-------------
::

    list <identifier> ((::|:>) <value>)?

The ``list`` type inherits from the ``Value`` class and contains a list of elements, which can be empty. The default value is an empty list (``[]``). When a value is assigned to a ``list`` type variable or cast as a ``list`` using the ``as`` keyword, some preprocessing may be required depending on the type of value being assigned. This preprocessing is outlined in the table below:

+---------------+------------------------------------------------------------+--------------------------+
| Value type    | Preprocessing                                              | Warning or error raised  |
+---------------+------------------------------------------------------------+--------------------------+
| ``int``,      | Value is put into a list, and a warning is raised          | :ref:`W008<warnings>`    |
| ``float``, or |                                                            |                          |
| ``str``       |                                                            |                          |
+---------------+------------------------------------------------------------+--------------------------+
| ``list``      | None                                                       | None                     |
+---------------+------------------------------------------------------------+--------------------------+

A ``list`` type can be multi-dimensional.

Accessing elements in a list
^^^^^^^^^^^^^^^^^^^^^^^^^^^^
::

    <list to access>[<range>(,<range>)*]

To access a range of elements or single element in a ``list``, the
