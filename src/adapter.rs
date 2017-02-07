use std::error::Error;

use ndarray::NDArray;
use ruru::{AnyObject, Array, Class, Fixnum, Object, VM};


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
        match size {
            Ok(size) => {
                itself.get_data(&*NDARRAY_WRAPPER)
                    .arange(size.to_i64() as usize);
            },
            Err(ref error) => { VM::raise(error.to_exception(), error.description()); },
        }
        itself
    }

    fn ndarray_reshape(shape: Array) -> RubyNDArray {
        match shape {
            Ok(shape) => {
                let mut shp = Vec::with_capacity(shape.length());

                for item in shape.into_iter() {
                    match item.try_convert_to::<Fixnum>() {
                        Ok(i) => {
                            shp.push(i.to_i64() as usize);
                        },
                        Err(ref error) => { VM::raise(error.to_exception(), error.description()); },
                    }
                }

                itself.get_data(&*NDARRAY_WRAPPER)
                    .reshape(&*shp);
            },
            Err(ref error) => { VM::raise(error.to_exception(), error.description()); },
        }
        itself
    }

    fn ndarray_length() -> Fixnum {
        Fixnum::new(
            itself.get_data(&*NDARRAY_WRAPPER).len() as i64
        )
    }
    fn ndarray_rank() -> Fixnum {
        Fixnum::new(
            itself.get_data(&*NDARRAY_WRAPPER).rank() as i64
        )
    }
);

#[no_mangle]
pub extern fn init_ndarray() {
    Class::new("RubyNDArray", None).define(|itself| {
        itself.def_self("new", ndarray_new);
        itself.def("arange", ndarray_arange);
        itself.def("reshape", ndarray_reshape);
        itself.def("length", ndarray_length);
        itself.def("rank", ndarray_rank);
        itself.def("ndim", ndarray_rank);
    });
}
