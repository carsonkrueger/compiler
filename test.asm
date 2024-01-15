i1: .INT    #0
i2: .INT    #1

        JMP MAIN
MAIN:   MOVI    R3, #1
        TRP     #1
        STR     R3, i2
        TRP     #0