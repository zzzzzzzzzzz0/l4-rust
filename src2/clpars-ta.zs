加载lib/clpars4。

赋予1宽、前出以6、2。
赋予解以调用‘命令行加回调’、、
	-、、0、下代码
		赋予层【上】以如果存在层【上】那么算术‘层’ + 1否则0。
		赋予檐宽以算术‘层’ + ‘1宽’ + ‘前出’。
		循环【‘檐宽’】先=了\换行。
		赋予檐宽以算术‘檐宽’ + 2。
		循环【‘檐宽’】先\了换行。
	上代码、
	、、1、下代码
		别名砖以参数0。
		赋予半宽以算术‘层’ + ‘1宽’。
		赋予墙高以算术3 + ‘层’ / 3、0。
		循环【‘墙高’】【砌】先
			循环【‘半宽’】【第】先
				如果‘砌’大于1
				并且‘砌’小于‘墙高’
				并且算术‘第’+ 2小于‘半宽’
				并且算术‘第’ % 3等于1那么
					“ ”
				否则先
					‘砖’
				了
			了。
			换行
		了。
	上代码。
显示调用‘命令行解析’、‘解’、
	-、*、-、@、-、&、-、$、-、%、-、+、-、#、-、#、-、#。
