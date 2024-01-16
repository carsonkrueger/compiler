i1: .INT    #0
i2: .INT    #8

        JMP MAIN
MAIN:   MOVI    R3, #1
        TRP     #1
        LDR     R4, i2
        STR     R3, R4
        TRP     #0