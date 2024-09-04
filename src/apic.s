section .text
    global apic_test
    extern apic_test_C

    apic_test:
        cld
        call apic_test_C
        iret   
