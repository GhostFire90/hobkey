#include "limine_requests.h"
#include <limine.h>


static volatile struct limine_memmap_request memmap_req = {
    .id = LIMINE_MEMMAP_REQUEST,
    .revision = 0,
    .response = 0x0
};

static volatile struct limine_hhdm_request hhdm_req = {
    .id = LIMINE_HHDM_REQUEST,
    .revision = 0,
    .response = 0x0
};

static volatile struct limine_kernel_address_request kernel_addr_request = {
    .id = LIMINE_KERNEL_ADDRESS_REQUEST,
    .revision = 0,
    .response = 0x0
};

static volatile struct limine_framebuffer_request frame_buffer_req = {
    .id = LIMINE_FRAMEBUFFER_REQUEST,
    .revision = 0
};

static volatile struct limine_module_request initrd_req = {
    .id = LIMINE_MODULE_REQUEST,
    .revision = 0
};

static volatile struct limine_stack_size_request stack_size ={
    .id = LIMINE_STACK_SIZE_REQUEST,
    .revision = 0,
    .stack_size = 0x10000
};

static volatile struct limine_rsdp_request rsdp_req = {
    .id = LIMINE_RSDP_REQUEST,
    .revision = 0
};

static volatile struct limine_smp_request smp_req ={
    .id = LIMINE_SMP_REQUEST,
    .revision = 0
};

const struct limine_memmap_response *limine_memmap()
{
    return memmap_req.response;
}

const struct limine_hhdm_response *limine_hhdm()
{
    return hhdm_req.response;
}

const struct limine_kernel_address_response *limine_kernel_addr()
{
    return kernel_addr_request.response;
}

const struct limine_module_response *limine_modules()
{
    return initrd_req.response;
}

const struct limine_framebuffer_response *limine_framebuffer()
{
    return frame_buffer_req.response;
}

const struct limine_rsdp_response *limine_rsdp()
{
    return rsdp_req.response;
}
