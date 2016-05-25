#![feature(const_fn)]
#![feature(asm)]
#![no_std]

#[macro_use]
extern crate vga;

extern {
    fn load_idt();

    fn isr0();
    fn isr1();
    fn isr2();
    fn isr3();
    fn isr4();
    fn isr5();
    fn isr6();
    fn isr7();
    fn isr8();
    fn isr9();
    fn isr10();
    fn isr11();
    fn isr12();
    fn isr13();
    fn isr14();
    fn isr15();
    fn isr16();
    fn isr17();
    fn isr18();
    fn isr19();
    fn isr20();
    fn isr21();
    fn isr22();
    fn isr23();
    fn isr24();
    fn isr25();
    fn isr26();
    fn isr27();
    fn isr28();
    fn isr29();
    fn isr30();
    fn isr31();
    fn isr32();
    fn isr33();
    fn isr34();
    fn isr35();
    fn isr36();
    fn isr37();
    fn isr38();
    fn isr39();
    fn isr40();
    fn isr41();
    fn isr42();
    fn isr43();
    fn isr44();
    fn isr45();
    fn isr46();
    fn isr47();
    fn isr48();
    fn isr49();
    fn isr50();
    fn isr51();
    fn isr52();
    fn isr53();
    fn isr54();
    fn isr55();
    fn isr56();
    fn isr57();
    fn isr58();
    fn isr59();
    fn isr60();
    fn isr61();
    fn isr62();
    fn isr63();
    fn isr64();
    fn isr65();
    fn isr66();
    fn isr67();
    fn isr68();
    fn isr69();
    fn isr70();
    fn isr71();
    fn isr72();
    fn isr73();
    fn isr74();
    fn isr75();
    fn isr76();
    fn isr77();
    fn isr78();
    fn isr79();
    fn isr80();
    fn isr81();
    fn isr82();
    fn isr83();
    fn isr84();
    fn isr85();
    fn isr86();
    fn isr87();
    fn isr88();
    fn isr89();
    fn isr90();
    fn isr91();
    fn isr92();
    fn isr93();
    fn isr94();
    fn isr95();
    fn isr96();
    fn isr97();
    fn isr98();
    fn isr99();
    fn isr100();
    fn isr101();
    fn isr102();
    fn isr103();
    fn isr104();
    fn isr105();
    fn isr106();
    fn isr107();
    fn isr108();
    fn isr109();
    fn isr110();
    fn isr111();
    fn isr112();
    fn isr113();
    fn isr114();
    fn isr115();
    fn isr116();
    fn isr117();
    fn isr118();
    fn isr119();
    fn isr120();
    fn isr121();
    fn isr122();
    fn isr123();
    fn isr124();
    fn isr125();
    fn isr126();
    fn isr127();
    fn isr128();
    fn isr129();
    fn isr130();
    fn isr131();
    fn isr132();
    fn isr133();
    fn isr134();
    fn isr135();
    fn isr136();
    fn isr137();
    fn isr138();
    fn isr139();
    fn isr140();
    fn isr141();
    fn isr142();
    fn isr143();
    fn isr144();
    fn isr145();
    fn isr146();
    fn isr147();
    fn isr148();
    fn isr149();
    fn isr150();
    fn isr151();
    fn isr152();
    fn isr153();
    fn isr154();
    fn isr155();
    fn isr156();
    fn isr157();
    fn isr158();
    fn isr159();
    fn isr160();
    fn isr161();
    fn isr162();
    fn isr163();
    fn isr164();
    fn isr165();
    fn isr166();
    fn isr167();
    fn isr168();
    fn isr169();
    fn isr170();
    fn isr171();
    fn isr172();
    fn isr173();
    fn isr174();
    fn isr175();
    fn isr176();
    fn isr177();
    fn isr178();
    fn isr179();
    fn isr180();
    fn isr181();
    fn isr182();
    fn isr183();
    fn isr184();
    fn isr185();
    fn isr186();
    fn isr187();
    fn isr188();
    fn isr189();
    fn isr190();
    fn isr191();
    fn isr192();
    fn isr193();
    fn isr194();
    fn isr195();
    fn isr196();
    fn isr197();
    fn isr198();
    fn isr199();
    fn isr200();
    fn isr201();
    fn isr202();
    fn isr203();
    fn isr204();
    fn isr205();
    fn isr206();
    fn isr207();
    fn isr208();
    fn isr209();
    fn isr210();
    fn isr211();
    fn isr212();
    fn isr213();
    fn isr214();
    fn isr215();
    fn isr216();
    fn isr217();
    fn isr218();
    fn isr219();
    fn isr220();
    fn isr221();
    fn isr222();
    fn isr223();
    fn isr224();
    fn isr225();
    fn isr226();
    fn isr227();
    fn isr228();
    fn isr229();
    fn isr230();
    fn isr231();
    fn isr232();
    fn isr233();
    fn isr234();
    fn isr235();
    fn isr236();
    fn isr237();
    fn isr238();
    fn isr239();
    fn isr240();
    fn isr241();
    fn isr242();
    fn isr243();
    fn isr244();
    fn isr245();
    fn isr246();
    fn isr247();
    fn isr248();
    fn isr249();
    fn isr250();
    fn isr251();
    fn isr252();
    fn isr253();
    fn isr254();
    fn isr255();
}

#[derive(Copy,Clone,Debug)]
#[repr(packed,C)]
struct IdtEntry {
    base_low: u16,
    selector: u16,
    zero: u8,
    flags: u8,
    base_mid: u16,
    base_high: u32,
    reserved: u32,
}

#[derive(Debug)]
#[repr(packed,C)]
pub struct IdtPointer {
    limit: u16,
    base: u64,
}

#[repr(packed,C)]
struct Idt {
    entries: [IdtEntry; 256],
}

impl Idt {
    const fn new() -> Idt {
        Idt {
            entries: [IdtEntry {
                base_low: 0,
                selector: 0,
                zero: 0,
                flags: 0,
                base_mid: 0,
                base_high: 0,
                reserved: 0,
            }; 256],
        }
    }

    fn set_isr(&mut self, num: u8, base: u64) {
        let base_low = (base - 0x100000) as u16;

        let new_isr = IdtEntry {
            base_low: base_low,
            selector: 0x8,
            zero: 0,
            flags: 0x8e,
            base_mid: 0x10,
            base_high: 0,
            reserved: 0,
        };

        self.entries[num as usize] = new_isr;
    }
}

static mut IDT: Idt = Idt::new();

#[no_mangle]
pub static mut IDT_POINTER: IdtPointer = IdtPointer { limit: 0, base: 0 };

pub fn install() {
    unsafe {
        IDT.set_isr(0, isr0 as u64);
        IDT.set_isr(1, isr1 as u64);
        IDT.set_isr(2, isr2 as u64);
        IDT.set_isr(3, isr3 as u64);
        IDT.set_isr(4, isr4 as u64);
        IDT.set_isr(5, isr5 as u64);
        IDT.set_isr(6, isr6 as u64);
        IDT.set_isr(7, isr7 as u64);
        IDT.set_isr(8, isr8 as u64);
        IDT.set_isr(9, isr9 as u64);
        IDT.set_isr(10, isr10 as u64);
        IDT.set_isr(11, isr11 as u64);
        IDT.set_isr(12, isr12 as u64);
        IDT.set_isr(13, isr13 as u64);
        IDT.set_isr(14, isr14 as u64);
        IDT.set_isr(15, isr15 as u64);
        IDT.set_isr(16, isr16 as u64);
        IDT.set_isr(17, isr17 as u64);
        IDT.set_isr(18, isr18 as u64);
        IDT.set_isr(19, isr19 as u64);
        IDT.set_isr(20, isr20 as u64);
        IDT.set_isr(21, isr21 as u64);
        IDT.set_isr(22, isr22 as u64);
        IDT.set_isr(23, isr23 as u64);
        IDT.set_isr(24, isr24 as u64);
        IDT.set_isr(25, isr25 as u64);
        IDT.set_isr(26, isr26 as u64);
        IDT.set_isr(27, isr27 as u64);
        IDT.set_isr(28, isr28 as u64);
        IDT.set_isr(29, isr29 as u64);
        IDT.set_isr(30, isr30 as u64);
        IDT.set_isr(31, isr31 as u64);
        IDT.set_isr(32, isr32 as u64);
        IDT.set_isr(33, isr33 as u64);
        IDT.set_isr(34, isr34 as u64);
        IDT.set_isr(35, isr35 as u64);
        IDT.set_isr(36, isr36 as u64);
        IDT.set_isr(37, isr37 as u64);
        IDT.set_isr(38, isr38 as u64);
        IDT.set_isr(39, isr39 as u64);
        IDT.set_isr(40, isr40 as u64);
        IDT.set_isr(41, isr41 as u64);
        IDT.set_isr(42, isr42 as u64);
        IDT.set_isr(43, isr43 as u64);
        IDT.set_isr(44, isr44 as u64);
        IDT.set_isr(45, isr45 as u64);
        IDT.set_isr(46, isr46 as u64);
        IDT.set_isr(47, isr47 as u64);
        IDT.set_isr(48, isr48 as u64);
        IDT.set_isr(49, isr49 as u64);
        IDT.set_isr(50, isr50 as u64);
        IDT.set_isr(51, isr51 as u64);
        IDT.set_isr(52, isr52 as u64);
        IDT.set_isr(53, isr53 as u64);
        IDT.set_isr(54, isr54 as u64);
        IDT.set_isr(55, isr55 as u64);
        IDT.set_isr(56, isr56 as u64);
        IDT.set_isr(57, isr57 as u64);
        IDT.set_isr(58, isr58 as u64);
        IDT.set_isr(59, isr59 as u64);
        IDT.set_isr(60, isr60 as u64);
        IDT.set_isr(61, isr61 as u64);
        IDT.set_isr(62, isr62 as u64);
        IDT.set_isr(63, isr63 as u64);
        IDT.set_isr(64, isr64 as u64);
        IDT.set_isr(65, isr65 as u64);
        IDT.set_isr(66, isr66 as u64);
        IDT.set_isr(67, isr67 as u64);
        IDT.set_isr(68, isr68 as u64);
        IDT.set_isr(69, isr69 as u64);
        IDT.set_isr(70, isr70 as u64);
        IDT.set_isr(71, isr71 as u64);
        IDT.set_isr(72, isr72 as u64);
        IDT.set_isr(73, isr73 as u64);
        IDT.set_isr(74, isr74 as u64);
        IDT.set_isr(75, isr75 as u64);
        IDT.set_isr(76, isr76 as u64);
        IDT.set_isr(77, isr77 as u64);
        IDT.set_isr(78, isr78 as u64);
        IDT.set_isr(79, isr79 as u64);
        IDT.set_isr(80, isr80 as u64);
        IDT.set_isr(81, isr81 as u64);
        IDT.set_isr(82, isr82 as u64);
        IDT.set_isr(83, isr83 as u64);
        IDT.set_isr(84, isr84 as u64);
        IDT.set_isr(85, isr85 as u64);
        IDT.set_isr(86, isr86 as u64);
        IDT.set_isr(87, isr87 as u64);
        IDT.set_isr(88, isr88 as u64);
        IDT.set_isr(89, isr89 as u64);
        IDT.set_isr(90, isr90 as u64);
        IDT.set_isr(91, isr91 as u64);
        IDT.set_isr(92, isr92 as u64);
        IDT.set_isr(93, isr93 as u64);
        IDT.set_isr(94, isr94 as u64);
        IDT.set_isr(95, isr95 as u64);
        IDT.set_isr(96, isr96 as u64);
        IDT.set_isr(97, isr97 as u64);
        IDT.set_isr(98, isr98 as u64);
        IDT.set_isr(99, isr99 as u64);
        IDT.set_isr(100, isr100 as u64);
        IDT.set_isr(101, isr101 as u64);
        IDT.set_isr(102, isr102 as u64);
        IDT.set_isr(103, isr103 as u64);
        IDT.set_isr(104, isr104 as u64);
        IDT.set_isr(105, isr105 as u64);
        IDT.set_isr(106, isr106 as u64);
        IDT.set_isr(107, isr107 as u64);
        IDT.set_isr(108, isr108 as u64);
        IDT.set_isr(109, isr109 as u64);
        IDT.set_isr(110, isr110 as u64);
        IDT.set_isr(111, isr111 as u64);
        IDT.set_isr(112, isr112 as u64);
        IDT.set_isr(113, isr113 as u64);
        IDT.set_isr(114, isr114 as u64);
        IDT.set_isr(115, isr115 as u64);
        IDT.set_isr(116, isr116 as u64);
        IDT.set_isr(117, isr117 as u64);
        IDT.set_isr(118, isr118 as u64);
        IDT.set_isr(119, isr119 as u64);
        IDT.set_isr(120, isr120 as u64);
        IDT.set_isr(121, isr121 as u64);
        IDT.set_isr(122, isr122 as u64);
        IDT.set_isr(123, isr123 as u64);
        IDT.set_isr(124, isr124 as u64);
        IDT.set_isr(125, isr125 as u64);
        IDT.set_isr(126, isr126 as u64);
        IDT.set_isr(127, isr127 as u64);
        IDT.set_isr(128, isr128 as u64);
        IDT.set_isr(129, isr129 as u64);
        IDT.set_isr(130, isr130 as u64);
        IDT.set_isr(131, isr131 as u64);
        IDT.set_isr(132, isr132 as u64);
        IDT.set_isr(133, isr133 as u64);
        IDT.set_isr(134, isr134 as u64);
        IDT.set_isr(135, isr135 as u64);
        IDT.set_isr(136, isr136 as u64);
        IDT.set_isr(137, isr137 as u64);
        IDT.set_isr(138, isr138 as u64);
        IDT.set_isr(139, isr139 as u64);
        IDT.set_isr(140, isr140 as u64);
        IDT.set_isr(141, isr141 as u64);
        IDT.set_isr(142, isr142 as u64);
        IDT.set_isr(143, isr143 as u64);
        IDT.set_isr(144, isr144 as u64);
        IDT.set_isr(145, isr145 as u64);
        IDT.set_isr(146, isr146 as u64);
        IDT.set_isr(147, isr147 as u64);
        IDT.set_isr(148, isr148 as u64);
        IDT.set_isr(149, isr149 as u64);
        IDT.set_isr(150, isr150 as u64);
        IDT.set_isr(151, isr151 as u64);
        IDT.set_isr(152, isr152 as u64);
        IDT.set_isr(153, isr153 as u64);
        IDT.set_isr(154, isr154 as u64);
        IDT.set_isr(155, isr155 as u64);
        IDT.set_isr(156, isr156 as u64);
        IDT.set_isr(157, isr157 as u64);
        IDT.set_isr(158, isr158 as u64);
        IDT.set_isr(159, isr159 as u64);
        IDT.set_isr(160, isr160 as u64);
        IDT.set_isr(161, isr161 as u64);
        IDT.set_isr(162, isr162 as u64);
        IDT.set_isr(163, isr163 as u64);
        IDT.set_isr(164, isr164 as u64);
        IDT.set_isr(165, isr165 as u64);
        IDT.set_isr(166, isr166 as u64);
        IDT.set_isr(167, isr167 as u64);
        IDT.set_isr(168, isr168 as u64);
        IDT.set_isr(169, isr169 as u64);
        IDT.set_isr(170, isr170 as u64);
        IDT.set_isr(171, isr171 as u64);
        IDT.set_isr(172, isr172 as u64);
        IDT.set_isr(173, isr173 as u64);
        IDT.set_isr(174, isr174 as u64);
        IDT.set_isr(175, isr175 as u64);
        IDT.set_isr(176, isr176 as u64);
        IDT.set_isr(177, isr177 as u64);
        IDT.set_isr(178, isr178 as u64);
        IDT.set_isr(179, isr179 as u64);
        IDT.set_isr(180, isr180 as u64);
        IDT.set_isr(181, isr181 as u64);
        IDT.set_isr(182, isr182 as u64);
        IDT.set_isr(183, isr183 as u64);
        IDT.set_isr(184, isr184 as u64);
        IDT.set_isr(185, isr185 as u64);
        IDT.set_isr(186, isr186 as u64);
        IDT.set_isr(187, isr187 as u64);
        IDT.set_isr(188, isr188 as u64);
        IDT.set_isr(189, isr189 as u64);
        IDT.set_isr(190, isr190 as u64);
        IDT.set_isr(191, isr191 as u64);
        IDT.set_isr(192, isr192 as u64);
        IDT.set_isr(193, isr193 as u64);
        IDT.set_isr(194, isr194 as u64);
        IDT.set_isr(195, isr195 as u64);
        IDT.set_isr(196, isr196 as u64);
        IDT.set_isr(197, isr197 as u64);
        IDT.set_isr(198, isr198 as u64);
        IDT.set_isr(199, isr199 as u64);
        IDT.set_isr(200, isr200 as u64);
        IDT.set_isr(201, isr201 as u64);
        IDT.set_isr(202, isr202 as u64);
        IDT.set_isr(203, isr203 as u64);
        IDT.set_isr(204, isr204 as u64);
        IDT.set_isr(205, isr205 as u64);
        IDT.set_isr(206, isr206 as u64);
        IDT.set_isr(207, isr207 as u64);
        IDT.set_isr(208, isr208 as u64);
        IDT.set_isr(209, isr209 as u64);
        IDT.set_isr(210, isr210 as u64);
        IDT.set_isr(211, isr211 as u64);
        IDT.set_isr(212, isr212 as u64);
        IDT.set_isr(213, isr213 as u64);
        IDT.set_isr(214, isr214 as u64);
        IDT.set_isr(215, isr215 as u64);
        IDT.set_isr(216, isr216 as u64);
        IDT.set_isr(217, isr217 as u64);
        IDT.set_isr(218, isr218 as u64);
        IDT.set_isr(219, isr219 as u64);
        IDT.set_isr(220, isr220 as u64);
        IDT.set_isr(221, isr221 as u64);
        IDT.set_isr(222, isr222 as u64);
        IDT.set_isr(223, isr223 as u64);
        IDT.set_isr(224, isr224 as u64);
        IDT.set_isr(225, isr225 as u64);
        IDT.set_isr(226, isr226 as u64);
        IDT.set_isr(227, isr227 as u64);
        IDT.set_isr(228, isr228 as u64);
        IDT.set_isr(229, isr229 as u64);
        IDT.set_isr(230, isr230 as u64);
        IDT.set_isr(231, isr231 as u64);
        IDT.set_isr(232, isr232 as u64);
        IDT.set_isr(233, isr233 as u64);
        IDT.set_isr(234, isr234 as u64);
        IDT.set_isr(235, isr235 as u64);
        IDT.set_isr(236, isr236 as u64);
        IDT.set_isr(237, isr237 as u64);
        IDT.set_isr(238, isr238 as u64);
        IDT.set_isr(239, isr239 as u64);
        IDT.set_isr(240, isr240 as u64);
        IDT.set_isr(241, isr241 as u64);
        IDT.set_isr(242, isr242 as u64);
        IDT.set_isr(243, isr243 as u64);
        IDT.set_isr(244, isr244 as u64);
        IDT.set_isr(245, isr245 as u64);
        IDT.set_isr(246, isr246 as u64);
        IDT.set_isr(247, isr247 as u64);
        IDT.set_isr(248, isr248 as u64);
        IDT.set_isr(249, isr249 as u64);
        IDT.set_isr(250, isr250 as u64);
        IDT.set_isr(251, isr251 as u64);
        IDT.set_isr(252, isr252 as u64);
        IDT.set_isr(253, isr253 as u64);
        IDT.set_isr(254, isr254 as u64);
        IDT.set_isr(255, isr255 as u64);

        IDT_POINTER.limit = 4096;
        IDT_POINTER.base = &IDT as *const _ as u64;

        load_idt();
    }
}

pub unsafe fn enable() {
    asm!("sti" :::: "volatile");
}

#[no_mangle]
pub extern "C" fn interrupt_handler(interrupt_number: isize, error_code: isize) {
    match interrupt_number {
        32 => {}, // timer
        _ => panic!("interrupt {} with error code 0x{:x}", interrupt_number, error_code),
    }
    unsafe{
        send_eoi(interrupt_number);
        enable();
    };
}

#[no_mangle]
pub extern fn pagefault_handler(address: usize, error_code: isize) {
    panic!("pagefault at 0x{:x} with error code {}", address, error_code)
}

#[no_mangle]
pub extern fn general_protection_fault_handler(address: usize, error_code: isize) {
    panic!("general protection fault at 0x{:x} with error code {}", address, error_code)
}

#[no_mangle]
pub extern fn keyboard_handler(interrupt_number: isize, key_code: usize) {
    assert!(interrupt_number == 33);
    kprintln!("Key code!: {}", key_code);
    unsafe{
        send_eoi(interrupt_number);
        enable();
    }
}

unsafe fn send_eoi(interrupt_number: isize) {
    match interrupt_number {
        i if i >= 40 => {
            asm!("outb %al, %dx" :: "{dx}"(0x20), "{al}"(0x20) :: "volatile");
            asm!("outb %al, %dx" :: "{dx}"(0xA0), "{al}"(0x20) :: "volatile");
        },
        32...40 => asm!("outb %al, %dx" :: "{dx}"(0x20), "{al}"(0x20) :: "volatile"),
        _ => {},
    }
}
