#ifndef PAGING_H
#define PAGING_H

#define PAGING_PRESENT        (1 << 0)  // Present; must be 1 to reference a paging table
#define PAGING_RW             (1 << 1)  // Read/write; if 0, writes may not be allowed (see Section 4.6)
#define PAGING_USER           (1 << 2)  // User/supervisor; if 0, user-mode accesses are not allowed (see Section 4.6)
#define PAGING_PWT            (1 << 3)  // Page-level write-through; indirectly determines memory type (see Section 4.9.2)
#define PAGING_PCD            (1 << 4)  // Page-level cache disable; indirectly determines memory type (see Section 4.9.2)
#define PAGING_ACCESSED       (1 << 5)  // Accessed; indicates whether this entry has been used (see Section 4.8)
#define PAGING_R              (1 << 11) // For ordinary paging, ignored; for HLAT paging, restart (see Section 4.8)


#define PAGING_NX              (1 << 63) // Execute-disable (if 1, instruction fetches are not allowed; see Section 4.6); otherwise, reserved (must be 0)





void initialize_paging();

#endif