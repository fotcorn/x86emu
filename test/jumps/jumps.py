from __future__ import print_function


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


def jump(instruction, value1, value2, jump_instr_taken, jump_instr_not_taken):
    print_tab('mov     ${},%rax'.format(value1))
    print_tab('{:8s}${},%rax'.format(instruction, value2))
    print_tab('{} jump{}'.format(jump_instr_taken, jump.counter))
    print_tab('int3')
    print('jump{}:'.format(jump.counter))
    print_tab('{} fail'.format(jump_instr_not_taken))
    print()

    jump.counter += 1

jump.counter = 0


def main():
    start()

    jump('add', -5, 5, 'jz', 'jnz')
    jump('add', 5, 5, 'jnz', 'jz')

    end()

if __name__ == '__main__':
    main()
