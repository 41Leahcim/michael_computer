# Michael computer
The Michael computer is a simple virtual computer made of gates and circuits. All components are based on nand-gates. The usage of loops, if-else, and match has been limited and reduced as much as possible. This makes it easier to translate the code to hardware.

# Instructions and binary codes
code | instruction
-|-
0000 00RT | Load constant
0000 01RT | Load memory
0000 10RF | store memory
0000 11RR | Not
0001 RTRF | Move between registers
0010 RTRF | Nand
0011 RTRF | And
0100 RTRF | Nor
0101 RTRF | Or
0110 RTRF | Xnor
0111 RTRF | Xor
1000 RTRF | Add
1001 RTRF | Add with overflow
1010 RTRF | Sub
1011 RTRF | Sub with overflow

RT = register from  
RF = register to  
RR = register to and from
