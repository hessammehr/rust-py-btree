#![crate_type = "dylib"]

#[macro_use] extern crate cpython;

use cpython::{PyObject, PyResult};
use std::{cell, cmp, collections};
use std::ops::Deref;

py_module_initializer!(btree, initbtree, PyInit_btree, |py, m| {
    try!(m.add(py, "__doc__", "Rust BTreeSet for Python."));
    try!(m.add_class::<BTreeSet>(py));
    Ok(())
});


/// Newtype around PyObject that implements Ord using python value comparisons.
/// Python exceptions are converted into Rust panics.
struct OrdPyObject(PyObject);

impl PartialEq for OrdPyObject {
    fn eq(&self, _other: &Self) -> bool {
        false
    }
}
impl Eq for OrdPyObject {}
impl PartialOrd for OrdPyObject {
    fn partial_cmp(&self, _other: &Self) -> Option<cmp::Ordering> {
        None
    }
}
impl Ord for OrdPyObject {
    fn cmp(&self, _other: &Self) -> cmp::Ordering {
        unimplemented!()
    }
}

// impl <'source> FromPyObject<'source> for OrdPyObject {
//     fn extract(py: Python, obj: &'source PyObject) -> PyResult<OrdPyObject> {
//         try!(obj.cast_as::<OrdPyObject>(py))
//     }
// }

impl Deref for OrdPyObject {
    type Target = PyObject;

    fn deref(&self) -> &PyObject {
        &self.0
    }
}

py_class!(class BTreeSet |py| {
    data set: cell::RefCell<collections::BTreeSet<OrdPyObject>>;

    def __new__(_cls) -> PyResult<BTreeSet> {
        BTreeSet::create_instance(py,
            cell::RefCell::new(collections::BTreeSet::new()))
    }

    def __bool__(&self) -> PyResult<bool> {
        Ok(!self.set(py).borrow().is_empty())
    }

    def __len__(&self) -> PyResult<usize> {
        Ok(self.set(py).borrow().len())
    }

    def insert(&self, item: PyObject) -> PyResult<bool> {
        let p = OrdPyObject(item);
        Ok(self.set(py).borrow_mut().insert(p))
    }
});

