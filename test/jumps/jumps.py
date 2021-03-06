from __future__ import print_function
import ctypes

def print_tab(string):
    print('    {}'.format(string))


def start():
    print('.text')
    print('.global _start')
    print('_start:')
    print()


def end():
    print_tab('mov     $0,%rbx')
    print_tab('mov     $1,%rax')
    print_tab('int     $0x80')
    print('fail:')
    print_tab('int3')


def jump(instruction, value1, value2, jump_instr_taken, jump_instr_not_taken,
        register1='al', register2='bl'):
    print_tab('mov     ${},%{}'.format(value1, register1))
    print_tab('mov     ${},%{}'.format(value2, register2))
    print_tab('{:8s}%{},%{}'.format(instruction, register1, register2))
    print_tab('{} jump{}'.format(jump_instr_taken, jump.counter))
    print_tab('int3')
    print('jump{}:'.format(jump.counter))
    print_tab('{} fail'.format(jump_instr_not_taken))
    print()

    jump.counter += 1

jump.counter = 0


def main():
    start()
    # zero
    for i in range(0, 256):
        for j in range(0, 256):
            if j - i == 0: 
                jump('cmp', i, j, 'jz', 'jnz')
            else:
                jump('cmp', i, j, 'jnz', 'jz')

    # sign
    for i in range(-128, 128):
        for j in range(-128, 128):
            ret = ctypes.c_byte(j - i)  # truncate value to one byte
            if ret.value < 0:
                jump('cmp', i, j, 'js', 'jns')
            else:
                jump('cmp', i, j, 'jns', 'js')

    # overflow aka signed carry
    for i in range(-128, 128):
        for j in range(-128, 128):
            res = i + j
            if res > 127 or res < -128:
                jump('add', i, j, 'jo', 'jno')
            else:
                jump('add', i, j, 'jno', 'jo')
            res = j - i
            if res > 127 or res < -128:
                jump('sub', i, j, 'jo', 'jno')
            else:
                jump('sub', i, j, 'jno', 'jo')

    ## below
    # unsigned/carry
    for i in range(0, 256):
        for j in range(0, 256):
            if i + j > 255:
                jump('add', i, j, 'jc', 'jnc')
            else:
                jump('add', i, j, 'jnc', 'jc')
            if j - i < 0:
                jump('sub', i, j, 'jc', 'jnc')
            else:
                jump('sub', i, j, 'jnc', 'jc')
    # signed
    jump('cmp', 5, 4, 'jl', 'jge')
    jump('cmp', 4, 5, 'jge', 'jl')
    jump('cmp', 5, 5, 'jge', 'jl')
    jump('cmp', -4, -5, 'jl', 'jge')
    jump('cmp', -5, -4, 'jge', 'jl')
    jump('cmp', -5, -5, 'jge', 'jl')

    ## below or equal
    # unsigned
    jump('cmp', 5, 4, 'jbe', 'ja')
    jump('cmp', 5, 5, 'jbe', 'ja')
    jump('cmp', 4, 5, 'ja', 'jbe')
    # signed
    jump('cmp', 5, 4, 'jle', 'jg')
    jump('cmp', 5, 5, 'jle', 'jg')
    jump('cmp', 4, 5, 'jg', 'jle')
    jump('cmp', -4, -5, 'jle', 'jg')
    jump('cmp', -5, -5, 'jle', 'jg')
    jump('cmp', -5, -4, 'jg', 'jle')

    # parity, even count of ones
    jump('add', 0, 0, 'jp', 'jnp')
    jump('add', 0, 1, 'jnp', 'jp')
    jump('add', 0, 2, 'jnp', 'jp')
    jump('add', 0, 3, 'jp', 'jnp')
    jump('add', 0, 4, 'jnp', 'jp')
    jump('add', 0, 5, 'jp', 'jnp')
    jump('add', 0, 6, 'jp', 'jnp')
    jump('add', 0, 7, 'jnp', 'jp')
    # parity only cares about the least significant byte
    jump('add', 0, 0 + 256, 'jp', 'jnp', register1='rax', register2='rbx')
    jump('add', 0, 0 + 512, 'jp', 'jnp', register1='rax', register2='rbx')
    jump('add', 0, 0 + 768, 'jp', 'jnp',register1='rax', register2='rbx')
    end()

if __name__ == '__main__':
    main()
