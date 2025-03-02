
.text
    .globl set_cr3
    .globl invalidate_page


    invalidate_page:
        invlpg [rdi]
        ret