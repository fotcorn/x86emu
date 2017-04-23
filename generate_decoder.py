
instructions = ['add', 'or', 'adc', 'sbb', 'and', 'sub', 'xor', 'cmp']


i = 0


for instruction in instructions:
    print("""    0x{i0:02X} => {{
        let argument = self.decode_8bit_reg_8bit_immediate(decoder_flags);
        self.cpu.{instr}(self.machine_state, argument);
    }}
    0x{i1:02X} => {{
        let argument = self.decode_reg_reg(register_size, decoder_flags);
        self.cpu.{instr}(self.machine_state, argument);
    }}
    0x{i2:02X} => {{
        let argument = self.decode_8bit_reg_8bit_immediate(decoder_flags | REVERSED_REGISTER_DIRECTION);
        self.cpu.{instr}(self.machine_state, argument);
    }}
    0x{i3:02X} => {{
        let argument = self.decode_reg_reg(register_size, decoder_flags | REVERSED_REGISTER_DIRECTION);
        self.cpu.{instr}(self.machine_state, argument);
    }}
    0x{i4:02X} => {{
        let argument = self.decode_al_immediate();
        self.cpu.{instr}(self.machine_state, argument);
    }}
    0x{i5:02X} => {{
        let argument = self.decode_ax_immediate(register_size, decoder_flags);
        self.cpu.{instr}(self.machine_state, argument);
    }}""".format(i0=i, i1=i + 1, i2=i + 2, i3=i + 3, i4=i + 4, i5=i + 5, i6=i + 6, instr=instruction))
    i += 8
