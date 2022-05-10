        .text                   # this file contains instruction code
.globl myfunc                   # myfunc is the name of a function
        .type   myfunc, @function
myfunc:                         # the start of the function
        pushl   %ebp            # function preamble:
        movl    %esp, %ebp      #  the 1st three instrs set up the stack
        subl    $16, %esp

        # A programmer adds specific IA32 instructions
        # here that allocate stack space for any local variables
        # and then implements code using parameters and locals to
        # perform the functionality of the myfunc function
        #
        # the return value should be stored in %eax before returning

        leave    # function return code
        ret
