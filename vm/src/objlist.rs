use super::pyobject::{PyObject, PyObjectKind, PyObjectRef, PyResult};
use super::vm::VirtualMachine;

fn get_pos(l: &Vec<PyObjectRef>, p: i32) -> usize {
    if p < 0 {
        l.len() - ((-p) as usize)
    } else {
        p as usize
    }
}

pub fn get_item(vm: &mut VirtualMachine, l: &Vec<PyObjectRef>, b: PyObjectRef) -> PyResult {
    match &(b.borrow()).kind {
        PyObjectKind::Integer { value } => {
            let pos_index = get_pos(l, *value);
            if pos_index < l.len() {
                let obj = l[pos_index].clone();
                Ok(obj)
            } else {
                Err(vm.new_exception("Index out of bounds!".to_string()))
            }
        }
        PyObjectKind::Slice { start, stop, step } => {
            let start = match start {
                &Some(start) => get_pos(l, start),
                &None => 0,
            };
            let stop = match stop {
                &Some(stop) => get_pos(l, stop),
                &None => l.len() as usize,
            };
            let step = match step {
                //Some(step) => step as usize,
                &None => 1 as usize,
                _ => unimplemented!("stepped slicing not supported for type {:?}", l),
            };
            // TODO: we could potentially avoid this copy and use slice
            let obj = PyObject::new(
                PyObjectKind::List {
                    elements: l[start..stop].to_vec(),
                },
                vm.get_type(),
            );
            Ok(obj)
        }
        _ => Err(vm.new_exception(format!(
            "TypeError: indexing type {:?} with index {:?} is not supported (yet?)",
            l, b
        ))),
    }
}

// set_item:
pub fn set_item(
    vm: &mut VirtualMachine,
    l: &mut Vec<PyObjectRef>,
    idx: PyObjectRef,
    obj: PyObjectRef,
) -> PyResult {
    match &(idx.borrow()).kind {
        PyObjectKind::Integer { value } => {
            let pos_index = get_pos(l, *value);
            l[pos_index] = obj;
            Ok(vm.get_none())
        }
        _ => panic!(
            "TypeError: indexing type {:?} with index {:?} is not supported (yet?)",
            l, idx
        ),
    }
}

pub fn append(vm: &mut VirtualMachine, l: PyObjectRef, other: PyObjectRef) -> PyResult {
    Ok(vm.get_none())
}

/* TODO:
pub fn make_type() -> PyObjectRef {

    // dict.insert("__set_item__".to_string(), set_item);
    dict.insert("append".to_string(), append)
}
*/
