use std::error::Error;

use ndarray::NDArray;
use ruru::{AnyObject, Class, Fixnum, Object, VM};


wrappable_struct!(NDArray<usize>, NDArrayWrapper, NDARRAY_WRAPPER);


class!(RubyNDArray);


methods!(
    RubyNDArray,
    itself,

    fn ndarray_new() -> AnyObject {
        let ndarray = NDArray::<usize>::new();
        Class::from_existing("RubyNDArray").wrap_data(ndarray, &*NDARRAY_WRAPPER)
    }

    fn ndarray_arange(size: Fixnum) -> RubyNDArray {
        if let Err(ref error) = size {
            VM::raise(error.to_exception(), error.description());
        }

        itself.get_data(&*NDARRAY_WRAPPER)
            .arange(size.unwrap().to_i64() as usize);

        itself
    }
);

#[no_mangle]
pub extern fn init_ndarray() {
    Class::new("RubyNDArray", None).define(|itself| {
        itself.def_self("new", ndarray_new);
        itself.def("arange", ndarray_arange);
    });
}
