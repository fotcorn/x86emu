from __future__ import print_function
register_operations = ['o', 'no', 'b', 'ae', 'e', 'ne', 'be', 'a', 's', 'ns', 'p', 'np', 'l', 'ge', 'le', 'g']

i = 0x40

for ro in register_operations:
    print("""    0x{:x} => {{
        let (argument, ip_offset) = self.get_argument(register_size,
                                                      RegOrOpcode::Register,
                                                      ImmediateSize::None,
                                                      decoder_flags | REVERSED_REGISTER_DIRECTION);
        self.inc_rip(ip_offset);
        self.cpu.cmov{}(self.machine_state, argument);
    }},""".format(i, ro))
    i += 1

for ro in register_operations:
    print('fn cmov{}(&self, machine_state: &mut MachineState, arg: InstructionArguments);'.format(ro))
    print()

for ro in register_operations:
    print("""    fn cmov{0}(&self, _machine_state: &mut MachineState, arg: InstructionArguments) {{
        print_instruction("cmov{0}", &arg);
    }}
    """.format(ro))

for i, ro in enumerate(register_operations):
    print("""    fn cmov{0}(&self, machine_state: &mut MachineState, arg: InstructionArguments) {{
        print_instruction("cmov{0}", &arg);
        if {1}machine_state.get_flag(Flags::) {{
            self.mov_impl(machine_state, arg);
        }}
    }}
""".format(ro, '' if i % 2 == 0 else '!'))
