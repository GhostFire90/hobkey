use crate::limine_req::FB_REQ;
use crate::memory::pmm::PMM;


#[no_mangle]
pub extern "C" fn kmain() -> !{

    let _before = PMM::get_avaiable_memory();    
    let mut _p = PMM::pop_page().unwrap();
    PMM::push_page(_p);
    _p = PMM::pop_page().unwrap();
    let _after = PMM::get_avaiable_memory();
    


    let fbr = FB_REQ.get_response().unwrap();
    let fb = fbr.framebuffers().next().unwrap();
    
    let buf_len : usize =((fb.bpp() as u64 / 8) *fb.width()*fb.height()).try_into().unwrap();    
    
    unsafe {
        fb.addr().write_bytes(0xff, buf_len);
    }
    loop {
        
    }
}

