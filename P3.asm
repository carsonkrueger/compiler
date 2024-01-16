size:   .INT  #10       ;4

arr:    .INT  #10       ;8
        .INT  #2
        .INT  #3
        .INT  #4
        .INT  #15
        .INT  #-6
        .INT  #7
        .INT  #8
        .INT  #9
        .INT  #10

i:      .INT  #0        ;48
sum:    .INT  #0        ;52
temp:   .INT            ;56
result: .INT            ;60

even:   .STR  " is even"        ;64
odd:    .STR  " is odd"         ;73
sumstr: .STR  "Sum is "         ;81
relend: .STR  " -- "            ;89
nl:     .BYT  '\n'              ;94
space:  .BYT  ' '               ;95
substr: .STR  " - "             ;96
eqstr:  .STR  " = "             ;100

DAGS:   .BYT  'D'               ;104
        .BYT  'A'
        .BYT  'G'
        .BYT  'S'

GADS:   .INT  #-99 ; 157 255 255 255    ;108

rel:    .BYT  ' '       ;112

; i = R15
; i offset = R14
; sum = R13
; result = R12

        JMP  MAIN                       ;113
MAIN:   LDA  R14, arr   ;load i offset  ;125

LOOP:   MOVI R10, #10                   ;137
        CMP  R10, R15  ;cmp i to size 10        ;149
        BRZ  R10, NEXT ;branch to NEXT at i == 10       ;161

        LDR  R0,  R14  ;load arr[i] into R0             ;173
        ADD  R13, R0   ;add arr[i] to sum               ;185

        LDR  R12, R14  ;begin arr[i] % 2                ;197
        DIVI R12, #2   ;div arr[i] by 2, will do int division   ;209
        MULI R12, #2   ;multiply division result by arr[i] to see if result comes back to full value    ;221
        CMP  R0,  R12  ;compare if they  are the same value, 0 if true, 1 if false (modulo)     ;233
        BRZ  R0,  P_EVEN        ;245
        BNZ  R0,  P_ODD         ;257

P_EVEN: LDR  R3,  R14           ;269
        TRP  #1        ;print arr[i]    ;281
        LDA  R3,  even
        TRP  #5        ;print even msg
        LDB  R3,  nl
        TRP  #3
        JMP  END
        
P_ODD:  LDR  R3,  R14
        TRP  #1        ;print arr[i]
        LDA  R3,  odd
        TRP  #5        ;print even msg
        LDB  R3,  nl
        TRP  #3
        JMP  END

END:    ADI  R14, #4   ;incrment i offset   
        ADI  R15, #1   ;increment i
        JMP  LOOP

NEXT:   LDA  R3,  sumstr
        TRP  #5
        MOV  R3,  R13
        TRP  #1
        LDB  R3,  nl
        TRP  #3

        LDR  R3,  DAGS ;copy DAGS into GADS
        STR  R3,  GADS
        LDR  R3,  GADS

        ;DAGS
        LDA  R4,  DAGS ;R4 contains address to byte 1
        LDA  R5,  DAGS ;R5 contains address to byte 3
        ADI  R5,  #2

        ;GADS
        LDA  R6,  GADS ;R6 contains address to byte 1
        LDA  R7,  GADS ;R7 contains address to byte 3
        ADI  R7,  #2

        LDB  R4,  R4
        LDB  R5,  R5
        
        STB  R4,  R7   ;set byte 1 to 'D'
        STB  R5,  R6   ;set byte 3 to 'G'

        LDB  R3,  nl
        TRP  #3

;R0 = i
;R4 = DAGS address
;R5 = GADS address

        MOVI R0,  #0
        LDA  R4,  DAGS
        LDA  R5,  GADS

LOOP_2: MOV  R1,  R0
        CMPI R1,  #4
        BRZ  R1,  NEXT_2

        MOVI R6,  #0
        MOVI R7,  #0
        LDB  R6,  R4 ;R6 = DAGS[i]
        LDB  R7,  R5 ;R7 = GADS[i]

        CMP  R6,  R7
        BLT  R6,  LT
        BGT  R6,  GT
        BRZ  R6,  EQ

        ;if DAGS[i] < GADS[i] then '<'
LT:     MOVI R8,  '<'
        STB  R8,  rel
        JMP  END_2

        ;else if DAGS[i] > GADS[i] then '>'
GT:     MOVI R8,  '>'
        STB  R8,  rel
        JMP  END_2
        
        ;if DAGS[i] = GADS[i] then '='
EQ:     MOVI R8,  '='
        STB  R8,  rel
        JMP  END_2

END_2:  LDB  R6,  R4 ;R6 = DAGS[i]
        LDB  R7,  R5 ;R7 = GADS[i]

        MOV  R3,  R6
        TRP  #3         ;print DAGS[i]
        LDB  R3,  space
        TRP  #3         ;print space
        LDB  R3,  rel
        TRP  #3         ;print rel
        LDB  R3,  space
        TRP  #3         ;print space
        MOV  R3,  R7
        TRP  #3         ;print GADS[i]
        LDA  R3,  relend
        TRP  #5         ;print --

        ADI  R0,  #1    ;increment i
        ADI  R4,  #1    ;increment DAGS address
        ADI  R5,  #1    ;increment GADS address
        JMP  LOOP_2

NEXT_2: LDB  R3,  nl
        TRP  #3
        TRP  #3

        LDR  R3,  DAGS
        TRP  #1

        LDA  R3,  substr
        TRP  #5

        LDR  R3,  GADS
        TRP  #1

        LDA  R3,  eqstr
        TRP  #5

        LDR  R3,  DAGS
        LDR  R4,  GADS
        SUB  R3,  R4
        TRP  #1

        JMP  EXIT

EXIT:   TRP  #0
