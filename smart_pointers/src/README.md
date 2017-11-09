Types:
------

- Box
- Rc and Arc
- Cell and Refcell

Cell vs RefCell
---------------

- Cell can only be used for Copy types
- Cell does not do any runtime checking to enforce borrow-checking rules,
  whereas RefCell does.
- Cell implements value semantics \ RefCell implements reference semantics
