


#[derive(Debug,Copy,Clone)]
pub struct DifferentSizeErr;


pub fn swap_slice<T>(a:&mut [T],b:&mut [T])->Result<(),DifferentSizeErr>{
    if a.len() == b.len(){
        for (a,b) in a.iter_mut().zip(b.iter_mut()){
            core::mem::swap(a,b);
        }
        Ok(())
    }else{
        Err(DifferentSizeErr)
    }
}


pub fn duplicate_empty_slice<T>(arr: &mut [T]) -> &mut [T] {
    assert!(arr.is_empty());
    unsafe { core::slice::from_raw_parts_mut(arr.as_mut_ptr(), 0) }
}


pub fn are_adjacent<'a, T1, T2>(first: &'a [T1], second: &'a [T2]) -> bool {
    let fl = first.len();
    first[fl..].as_ptr() == second.as_ptr() as *const T1
}

pub fn slice_join_mut<'a, T>(first: &'a mut [T], second: &'a mut [T]) -> &'a mut [T] {
    let fl = first.len();
    if first[fl..].as_mut_ptr() == second.as_mut_ptr() {
        unsafe { ::core::slice::from_raw_parts_mut(first.as_mut_ptr(), fl + second.len()) }
    } else {
        panic!("Slices not adjacent");
    }
}

pub fn slice_join_bytes_mut<'a, T>(first: &'a mut [T], second: &'a mut [u8]) -> &'a mut [u8] {
    let fl = first.len();
    if first[fl..].as_mut_ptr() as *mut u8 == second.as_mut_ptr() {
        unsafe {
            ::core::slice::from_raw_parts_mut(
                first.as_mut_ptr() as *mut u8,
                fl * core::mem::size_of::<T>() + second.len(),
            )
        }
    } else {
        panic!("Slices not adjacent");
    }
}

pub fn bytes_join_slice_mut<'a, T>(first: &'a mut [u8], second: &'a mut [T]) -> &'a mut [u8] {
    let fl = first.len();
    if first[fl..].as_mut_ptr() == second.as_mut_ptr() as *mut u8 {
        unsafe {
            ::core::slice::from_raw_parts_mut(
                first.as_mut_ptr() as *mut u8,
                fl + second.len() * core::mem::size_of::<T>(),
            )
        }
    } else {
        panic!("Slices not adjacent");
    }
}
