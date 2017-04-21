def main():
    print('.text')
    print('.global  _start')
    print('_start:')

    # value is positive, there is no difference between sign and zero extend
    # sign extend
    movsz('movsbw', 'r8b', 'bx', 'cx', 5, 5)
    movsz('movsbl', 'r8b', 'ebx', 'ecx', 5, 5)
    movsz('movsbq', 'r8b', 'rbx', 'rcx', 5, 5)

    movsz('movswl', 'ax', 'ebx', 'ecx', 5, 5)
    movsz('movswq', 'ax', 'rbx', 'rcx', 5, 5)

    # zero extend
    movsz('movzbw', 'r8b', 'bx', 'cx', 5, 5)
    movsz('movzbl', 'r8b', 'ebx', 'ecx', 5, 5)
    movsz('movzbq', 'r8b', 'rbx', 'rcx', 5, 5)

    movsz('movzwl', 'ax', 'ebx', 'ecx', 5, 5)
    movsz('movzwq', 'ax', 'rbx', 'rcx', 5, 5)


    # value is negative, sign extend will preserve value, zero extend will not
    # sign extend
    movsz('movsbw', 'r8b', 'bx', 'cx', -5, -5)
    movsz('movsbl', 'r8b', 'ebx', 'ecx', -5, -5)
    movsz('movsbq', 'r8b', 'rbx', 'rcx', -5, -5)

    movsz('movswl', 'ax', 'ebx', 'ecx', -5, -5)
    movsz('movswq', 'ax', 'rbx', 'rcx', -5, -5)

    # zero extend
    movsz('movzbw', 'r8b', 'bx', 'cx', -5, 2**8 - 5)
    movsz('movzbl', 'r8b', 'ebx', 'ecx', -5, 2**8 - 5)
    movsz('movzbq', 'r8b', 'rbx', 'rcx', -5, 2**8 - 5)

    movsz('movzwl', 'ax', 'ebx', 'ecx', -5, 2**16 - 5)
    movsz('movzwq', 'ax', 'rbx', 'rcx', -5, 2**16 - 5)

    print('    mov     $0,%rbx')
    print('    mov     $1,%rax')
    print('    int     $0x80')
    print('fail:')
    print('    int3')


def movsz(instruction, register_in, register_out, register_check, value_in, excepted_value):
    print("""
    mov     ${value_in}, %{register_in}
    {instruction}  %{register_in}, %{register_out}
    mov     ${excepted_value}, %{register_check}
    cmp     %{register_check}, %{register_out}
    jnz     fail
    """.format(instruction=instruction,
               register_in=register_in,
               register_out=register_out,
               register_check=register_check,
               value_in=value_in,
               excepted_value=excepted_value))

if __name__ == '__main__':
    main()
