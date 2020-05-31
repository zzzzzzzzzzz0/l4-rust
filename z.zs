#!/usr/lib/zhscript2/l --。
赋予调试【顶】以。
定义提示、内容以下代码
	显示“# ‘内容’”换行。
上代码。
定义运行、命令、饶以下代码
	提示‘命令’。
	如果‘调试’那么提示未‘参数0’，退出。
	赋予错以执行‘命令’。
	提示‘错’。
	如果‘错’并且不‘饶’那么结束10。
上代码。
定义行解以下代码
	调用‘命令行解析’、‘参数栈’。
上代码。

加载lib/clpars4。
调用‘命令行加回调’、
	r|r1|r2|r0、下原样
		运行“cargo run ”分支‘参数0’先
			r1：src2/dl.zs。
			r2：src2/gjk.zs。
			r0：“-- ---- -zhscript-help”。
		了“ ‘参数’”上原样、a、、
	br、下代码
		运行“rust.zs2 ‘参数0’”
	上代码、0、、
	mvr、下代码
		赋予源/、标/以./target/release/、/zzzzzzzzzzz4/usr/lib/zhscript2-rust/。
		运行“mv ‘源/’l ‘标/’l”。
		运行“mv ‘源/’libl4.so ‘标/’l4.so”。
	上代码、0、、
	meld、下代码
		运行“meld /zzzzzzzzzzz4/usr/lib/zhscript2-rust/lib src2/lib”
	上代码、0、、
	-h2、“赋予调试【顶】以1。”、0、、
	#、、h、。
行解‘参数栈’。
