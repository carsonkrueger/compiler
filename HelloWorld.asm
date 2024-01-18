A:      .STR  "Hello "          ;4-10
B:      .STR  "Carson Krueger"  ;11-25
C:      .STR  "I can print ;"   ;26-39
D:      .STR  "1 + 3 = "         ;40-48
E:      .STR  "2 * 3 = "         ;49-57
F:      .STR  "1 - 5 = "         ;58-66
G:      .STR  "5 / 2 = "         ;67-76
one:    .INT  #1                ;80
two:    .INT  #2                ;84
three:  .INT  #3                ;88
five:   .INT  #5                ;92
H:      .STR  "Goodbye"         ;100
nl:     .BYT  '\n'              ;101

        JMP  MAIN
MAIN:   LDA  R3, A
        TRP  #5     ;print A

        LDA  R3, B
        TRP  #5     ; print B

        LDB  R3, nl
        TRP  #3

        LDA  R3, D
        TRP  #5     ; print D
        LDR  R3, one
        LDR  R4, three
        ADD  R3, R4
        TRP  #1     ; print D result

        LDB  R3, nl
        TRP  #3

        LDA  R3, E
        TRP  #5     ; print E
        LDR  R3, two
        LDR  R4, three
        MUL  R3, R4
        TRP  #1     ; print E result

        LDB  R3, nl
        TRP  #3

        LDA  R3, F
        TRP  #5     ; print F
        LDR  R3, one
        LDR  R4, five
        SUB  R3, R4
        TRP  #1     ; print F result

        LDB  R3, nl
        TRP  #3

        LDA  R3, G
        TRP  #5     ; print G
        LDR  R3, five
        LDR  R4, two
        DIV  R3, R4
        TRP  #1     ; print G result

        LDB  R3, nl
        TRP  #3

        LDA  R3, C
        TRP  #5     ; print C
        
        LDB  R3, nl
        TRP  #3

        LDA  R3, H
        TRP  #5     ; print H

        TRP  #0