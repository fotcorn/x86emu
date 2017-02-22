
vmlinuz:     file format pei-x86-64


Disassembly of section .text:

00000000000048a0 <.text+0x2a0>:
    48a0:	31 c0                	xor    %eax,%eax
    48a2:	8e d8                	mov    %eax,%ds
    48a4:	8e c0                	mov    %eax,%es
    48a6:	8e d0                	mov    %eax,%ss
    48a8:	8e e0                	mov    %eax,%fs
    48aa:	8e e8                	mov    %eax,%gs
    48ac:	48 8d 2d 4d fd ff ff 	lea    -0x2b3(%rip),%rbp        # 0x4600
    48b3:	8b 86 30 02 00 00    	mov    0x230(%rsi),%eax
    48b9:	ff c8                	dec    %eax
    48bb:	48 01 c5             	add    %rax,%rbp
    48be:	48 f7 d0             	not    %rax
    48c1:	48 21 c5             	and    %rax,%rbp
    48c4:	48 81 fd 00 00 00 01 	cmp    $0x1000000,%rbp
    48cb:	7d 07                	jge    0x48d4
    48cd:	48 c7 c5 00 00 00 01 	mov    $0x1000000,%rbp
    48d4:	48 8d 9d 00 40 fa 00 	lea    0xfa4000(%rbp),%rbx
    48db:	48 8d a3 c0 7d 6c 00 	lea    0x6c7dc0(%rbx),%rsp
    48e2:	6a 00                	pushq  $0x0
    48e4:	9d                   	popfq  
    48e5:	56                   	push   %rsi
    48e6:	48 8d 35 cb 3a 6b 00 	lea    0x6b3acb(%rip),%rsi        # 0x6b83b8
    48ed:	48 8d bb b8 3d 6b 00 	lea    0x6b3db8(%rbx),%rdi
    48f4:	48 c7 c1 c0 3d 6b 00 	mov    $0x6b3dc0,%rcx
    48fb:	48 c1 e9 03          	shr    $0x3,%rcx
    48ff:	fd                   	std    
    4900:	f3 48 a5             	rep movsq %ds:(%rsi),%es:(%rdi)
    4903:	fc                   	cld    
    4904:	5e                   	pop    %rsi
    4905:	48 8d 83 90 a4 6a 00 	lea    0x6aa490(%rbx),%rax
    490c:	ff e0                	jmpq   *%rax
	...
    498e:	00 00                	add    %al,(%rax)
    4990:	48 89 3d 6a 39 6b 00 	mov    %rdi,0x6b396a(%rip)        # 0x6b8301
    4997:	48 89 35 6b 39 6b 00 	mov    %rsi,0x6b396b(%rip)        # 0x6b8309
    499e:	48 8d 05 5c 39 6b 00 	lea    0x6b395c(%rip),%rax        # 0x6b8301
    49a5:	48 89 05 ec 38 6b 00 	mov    %rax,0x6b38ec(%rip)        # 0x6b8298
    49ac:	48 89 d6             	mov    %rdx,%rsi
    49af:	e9 ac fe ff ff       	jmpq   0x4860
    49b4:	1f                   	(bad)  
    49b5:	8b 08                	mov    (%rax),%ecx
    49b7:	00 00                	add    %al,(%rax)
    49b9:	00 00                	add    %al,(%rax)
    49bb:	00 02                	add    %al,(%rdx)
    49bd:	03 ec                	add    %esp,%ebp
    49bf:	fd                   	std    
    49c0:	7b 7c                	jnp    0x4a3e
    49c2:	1c 65                	sbb    $0x65,%al
    49c4:	dd f8                	fnop   
    49c6:	ff cf                	dec    %edi
    49c8:	26                   	es
    49c9:	59                   	pop    %rcx
    49ca:	9a                   	(bad)  
    49cb:	d2 94 09 d0 40 10 90 	rclb   %cl,-0x6fefbf30(%rcx,%rcx,1)
    49d2:	40 53                	rex push %rbx
    49d4:	6c                   	insb   (%dx),%es:(%rdi)
    49d5:	45 a4                	rex.RB movsb %ds:(%rsi),%es:(%rdi)
    49d7:	e1 a0                	loope  0x4979
    49d9:	8d a5 92 a5 89 cc    	lea    -0x33765a6e(%rbp),%esp
    49df:	c2 46 8b             	retq   $0x8b46
    49e2:	b4 80                	mov    $0x80,%ah
    49e4:	22 8a d6 22 1e d0    	and    -0x2fe1dd2a(%rdx),%cl
    49ea:	de b0 0b 55 a0 4d    	fidiv  0x4da0550b(%rax)
    49f0:	98                   	cwtl   
    49f1:	84 76 1c             	test   %dh,0x1c(%rsi)
    49f4:	16                   	(bad)  
    49f5:	2a 07                	sub    (%rdi),%al
    49f7:	45 d1 db             	rex.RB rcr %r11d
    49fa:	2a 2a                	sub    (%rdx),%ch
    49fc:	f5                   	cmc    
    49fd:	46 a5                	rex.RX movsl %ds:(%rsi),%es:(%rdi)
    49ff:	08 96 d2 42 9b 43    	or     %dl,0x439b42d2(%rsi)
    4a05:	c9                   	leaveq 
    4a06:	b6 1c                	mov    $0x1c,%dh
    4a08:	43 39 55 8a          	rex.XB cmp %edx,-0x76(%r13)
    4a0c:	10 2a                	adc    %ch,(%rdx)
    4a0e:	94                   	xchg   %eax,%esp
    4a0f:	09 e1                	or     %esp,%ecx
    4a11:	10 68 9b             	adc    %ch,-0x65(%rax)
    4a14:	9e                   	sahf   
    4a15:	a0 fb 7d bf af d9 4d 	movabs 0x29364dd9afbf7dfb,%al
    4a1c:	36 29 
    4a1e:	dc 9f af bf ef e3    	fcompl -0x1c104051(%rdi)
    4a24:	f7 fd                	idiv   %ebp
    4a26:	e3 f7                	jrcxz  0x4a1f
    4a28:	7b 3d                	jnp    0x4a67
    4a2a:	1f                   	(bad)  
    4a2b:	d0 ec                	shr    %ah
    4a2d:	ce                   	(bad)  
    4a2e:	ce                   	(bad)  
    4a2f:	e1 9a                	loope  0x49cb
    4a31:	6b 66 ae d3          	imul   $0xffffffd3,-0x52(%rsi),%esp
    4a35:	5c                   	pop    %rsp
    4a36:	73 cd                	jae    0x4a05
