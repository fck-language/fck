.. role:: inftag

Error messages
==============

Overview
--------

One of the more playful features of fck is the customisable error messages. Upon running either the shell, or interpreting or compiling a file, fck will parse a raw text file that contains the custom warning and error messages to return to the end user. The general style of a warning or error is:

.. code-block:: text

    *********************************************************************
    [Warning or Error]:
    <Description of warning or error>: <Custom message related to warning
    or error, wrapped to a given length>
    <Traceback>
    **********************************************************************

The warning or error is wrapped to a predetermined length, specified in the ``.fck`` file, and given the default value of 70.

Error message file
------------------

.. code-block:: text

    <Error or warning identifier>=
        (["<Custom message>"(,"<Custom message>")*])|("<Single custom message>")


The general format of each line of the error message file can either be a single message surrounded by inverted commas (``'``) or quote marks (``"``), or it could be a list of messages with the same format as a single message, separated by commas.

Error and warning identifiers
-----------------------------

All the warnings and error message identifiers are listed below, along with an explanation of what each one is used for:

.. _warnings:

Warnings
^^^^^^^^

* ``DivideByZero`` (``W001``)
    Raised when dividing by zero. Returns an infinity
* ``ModByZero`` (``W002``)
    Raised when calculating ``a % 0``. Returns 0
* ``ValueMultString`` (``W003``)
    Raised when multiplying a value by a string. Included due to bad practice. Returns string multiplied by the value
* ``StringMultFloat`` (``W004``)
    Raised when multiplying a string by a float value. Rounds the float value to an int
* ``InfinityDivValue`` (``W005``)
    Raised when an infinity is divided by a value. Returns an infinity where the saved value is divided by the given value
* ``ValueDivInfinity`` (``W006``)
    Raised when a value is divided by an infinity. Returns 0
* ``InfinityDivInfinity`` (``W007``)
    Raised when an infinity is divided by another infinity. Returns the saved value of the first infinity divided by the saved value of the second infinity
* ``ListFromValue`` (``W008``)
    Raised when a value is assigned to a list variable. Converts the value into a list containing the value
* ``ListIndexOutOfRange`` (``W009``)
    Raised when the given range extends outside of the range of the given list. Alters the range to fit inside the list. See the :ref:`list documentation` for more information
* ``ListIndexFloat`` (``W010``)
    Raised when the given range for a list contains a float. The float value is rounded
* ``SilentCaseReset`` (``W011``)
    Raised when a new option for a ``silent<case>`` variable has the same expression as an already specified option. Replaces the old option statement with the new option statement
* ``IterateStepLoop`` (``W012``)
    Raised when the given step value of an iterate loop would result in the loop never reaching the second value. The step value is multiplied by -1
* ``IterateStepZero`` (``W013``)
    Raised when the given step value of an iterate loop evaluates to 0. Step value is ignores and the default value is used.
* ``ValueFromList`` (``W014``)
    Raised when a list is assigned to either an ``int``, ``float``, ``sfloat``, or ``str``. Value in the list is used instead of the list. **Only** raised if the list recursively has 1 element (true for ``[1]`` and ``[[1]]``, but not ``[[1], 2]``) and the type of the single element can be used with the variable type.
* ``StringFromValue`` (``W015``)
    Raised when a value is assigned to a string variable. Value is converted into a string
* ``ValueFromString`` (``W016``)
    Raised when a string is assigned to either an ``int``, ``float``, or ``sfloat``. String is converted into a value

.. _errors:

Errors
^^^^^^
