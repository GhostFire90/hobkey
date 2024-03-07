#include "limine_requests.h"
#include <limine.h>


struct limine_memmap_request memmap_req = {
    .id = LIMINE_MEMMAP_REQUEST,
    .revision = 0
};

static struct limine_hhdm_request hhdm_req = {
    .id = LIMINE_HHDM_REQUEST,
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