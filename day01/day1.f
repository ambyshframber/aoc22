: sorttop2 
    2DUP < IF SWAP THEN
;
: sorttop4
    sorttop2 \ cd
    -ROT sorttop2 ROT \ bc
    2SWAP sorttop2 2SWAP \ ab
    sorttop2 \ cd
    -ROT sorttop2 ROT \ bc
    sorttop2
;

8 ALLOT
: line ( -- addr len )
    LITERAL DUP ( base ptr )
    BEGIN KEY DUP '\n' <> WHILE
        ( base ptr char )
        OVER C! 1+
    REPEAT
    DROP OVER -
;

: elf
    0
    BEGIN line DUP WHILE \ get a line, if len is 0, break
        NUMBER DROP +
    REPEAT
    2DROP
;
: elves
    0 0 0 \ so sort doesnt underflow
    BEGIN elf WHILE \ exit when you get the false 0-calorie elf
        sorttop4 DROP
    REPEAT
    DROP
;

elves
