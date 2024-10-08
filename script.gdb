define igr
i r eip edi esi edx ecx ebx esp ebp eflags
end

define nibt
ni
bt
end

define nidisas
ni
x/16i $eip
end

define niir
ni
igr
end

set disassembly-flavor intel
target remote localhost:1234
b _start
c
c
