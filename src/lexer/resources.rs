pub static INSTRUCTION_NAMES: [&'static str; 56] = [
    "mov",
	"movs",
	"mvn",
	"mvns",
	"srw",
	"srr",
	"ldr",
	"ldro",
	"ldri",
	"str",
	"stro",
	"stri",
	"add",
	"adds",
	"addc",
	"addcs",
	"sub",
	"subs",
	"subc",
	"subcs",
	"and",
	"ands",
	"or",
	"ors",
	"eor",
	"eors",
	"lsl",
	"lsls",
	"lsr",
	"lsrs",
	"asr",
	"asrs",
	"cls",
	"csls",
	"csr",
	"csrs",
	"cmn",
	"addcd",
	"cmp",
	"subcd",
	"andd",
	"ord",
	"eord",
	"lsld",
	"lsrd",
	"asrd",
	"csld",
	"csrd",
	"ba",
	"bal",
	"br",
	"brl",
	"ptr",
	"ptw",
	"ptsr",
	"svc"
];

pub static MACRO_NAMES: [&'static str; 2] = [
    "!mov",
	"!b"
];

pub static DIRECTIVE_NAMES: [&'static str; 6] = [
    ".res",
	".byte",
	".bytes",
	".arr",
	".label",
	".start"
];

pub static REGISTER_NAMES: [&'static str; 16] = [
    "r0",
	"r1",
	"r2",
	"r3",
	"r4",
	"r5",
	"r6",
	"r7",
	"r8",
	"r9",
	"r10",
	"r11",
	"r12",
	"r13",
	"r14",
	"r15"
];

pub static PORT_NAMES: [&'static str; 8] = [
    "p0",
	"p1",
	"p2",
	"p3",
	"p4",
	"p5",
	"p6",
	"p7"
];

pub static SYSTEM_REGISTER_NAMES: [&'static str; 6] = [
    "pc_b0",
	"pc_b1",
	"pdbr_b0",
	"pdbr_b1",
	"psr",
	"intr"
];

pub static CONDITION_CODE_NAMES: [&'static str; 19] = [
    "al",
	"eq",
	"zs",
	"mi",
	"vs",
	"su",
	"cc",
	"gu",
	"ss",
	"gs",
	"ne",
	"zc",
	"pl",
	"vc",
	"geu",
	"cs",
	"seu",
	"ges",
	"ses"
];