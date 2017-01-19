400526:	55                   	push   %rbp
400527:	48 89 e5             	mov    %rsp,%rbp
40052a:	48 83 ec 10          	sub    $0x10,%rsp
40052e:	c7 45 fc 05 00 00 00 	movl   $0x5,-0x4(%rbp)
400535:	83 45 fc 0f          	addl   $0xf,-0x4(%rbp)
400539:	8b 45 fc             	mov    -0x4(%rbp),%eax
40053c:	89 c6                	mov    %eax,%esi
40053e:	bf 04 06 40 00       	mov    $0x400604,%edi
400543:	b8 00 00 00 00       	mov    $0x0,%eax
400548:	e8 b3 fe ff ff       	callq  400400 <printf@plt>
40054d:	8b 45 fc             	mov    -0x4(%rbp),%eax
400550:	8d 50 03             	lea    0x3(%rax),%edx
400553:	85 c0                	test   %eax,%eax
400555:	0f 48 c2             	cmovs  %edx,%eax
400558:	c1 f8 02             	sar    $0x2,%eax
40055b:	89 45 fc             	mov    %eax,-0x4(%rbp)
40055e:	8b 45 fc             	mov    -0x4(%rbp),%eax
400561:	89 c6                	mov    %eax,%esi
400563:	bf 04 06 40 00       	mov    $0x400604,%edi
400568:	b8 00 00 00 00       	mov    $0x0,%eax
40056d:	e8 8e fe ff ff       	callq  400400 <printf@plt>
400572:	b8 00 00 00 00       	mov    $0x0,%eax
400577:	c9                   	leaveq
400578:	c3                   	retq
400579:	0f 1f 80 00 00 00 00 	nopl   0x0(%rax)
