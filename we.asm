;  FIRST FOUR BYTES: Initial Program Counter 
;      The address/offset of the first instruction:  6 (0x00000006)

;  First Static/Global Variable in the Data Segment
W:       .BYT 'W'    ; offset 4 (first pass create symbol table  { <label_name>: offset })
e:       .BYT 'e'    ; offset 5

;  First Instruction in the Code Segment 
; (No Static/Global Variables Allowed After the First Instruction)
         JMP  MAIN   ; offset 6   => 0x00000001, 0x00000012, 0x00000000 ;First Instruction
MAIN:    LDB  R3, W  ; offset 18  => 0x0000000C, 0x00000003, 0x00000004
         TRP  #3     ; offset 30
         LDB  R3, e  ; offset 42
         TRP  #3     ; offset 54
         TRP  #0     ; offset 66
                     ; total size = 66 + 12 = 78 bytes
;{
;  "W": 4,
;  "e": 5,
;  "code_segment_begins": 6 ;       flag for the beginning of the code segment
;  "MAIN": 18 (0x12)
;  "code_segment_ends": 78 (0x4E) ; flag for the end of the code segment
;}