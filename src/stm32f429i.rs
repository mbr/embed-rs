use super::InterruptHandler;

#[repr(C)]
pub struct VectorTable {
    pub msp: &'static (),
    pub reset: Option<InterruptHandler>,
    pub nmi: Option<InterruptHandler>,
    pub hard_fault: Option<InterruptHandler>,
    pub mem_manage: Option<InterruptHandler>,
    pub bus_fault: Option<InterruptHandler>,
    pub usage_fault: Option<InterruptHandler>,
    pub _reserved7: Option<InterruptHandler>,
    pub _reserved8: Option<InterruptHandler>,
    pub _reserved9: Option<InterruptHandler>,
    pub _reserved10: Option<InterruptHandler>,
    pub svc: Option<InterruptHandler>,
    pub debug_monitor: Option<InterruptHandler>,
    pub _reserved13: Option<InterruptHandler>,
    pub pend_sv: Option<InterruptHandler>,
    pub sys_tick: Option<InterruptHandler>,
    pub interrupt0: Option<InterruptHandler>,
    pub interrupt1: Option<InterruptHandler>,
    pub interrupt2: Option<InterruptHandler>,
    pub interrupt3: Option<InterruptHandler>,
    pub interrupt4: Option<InterruptHandler>,
    pub interrupt5: Option<InterruptHandler>,
    pub interrupt6: Option<InterruptHandler>,
    pub interrupt7: Option<InterruptHandler>,
    pub interrupt8: Option<InterruptHandler>,
    pub interrupt9: Option<InterruptHandler>,
}

pub const VECTOR_TABLE: VectorTable = VectorTable {
    msp: &(),
    reset: None,
    nmi: None,
    hard_fault: None,
    mem_manage: None,
    bus_fault: None,
    usage_fault: None,
    _reserved7: None,
    _reserved8: None,
    _reserved9: None,
    _reserved10: None,
    svc: None,
    debug_monitor: None,
    _reserved13: None,
    pend_sv: None,
    sys_tick: None,
    interrupt0: None,
    interrupt1: None,
    interrupt2: None,
    interrupt3: None,
    interrupt4: None,
    interrupt5: None,
    interrupt6: None,
    interrupt7: None,
    interrupt8: None,
    interrupt9: None,
};

pub const INITIAL_CPU_FREQ: usize = 8_000_000;
