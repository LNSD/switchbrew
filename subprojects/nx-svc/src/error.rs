//! Error codes for the SVC.
//!
//! This module contains the error codes for the SVC.

/// Identifies which module caused an error.
///
/// Note that error codes can propagate through a call chain, so this may not always
/// correspond to the module containing the API call that returned the error.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(u32)]
pub enum Module {
    /// SVC
    Kernel = 1,
    FS = 2,
    /// Used for Memory, Thread, Mutex, Nvidia, etc.
    OS = 3,
    HTCS = 4,
    NCM = 5,
    DD = 6,
    LR = 8,
    Loader = 9,
    CMIF = 10,
    HIPC = 11,
    TMA = 12,
    DMNT = 13,
    GDS = 14,
    PM = 15,
    NS = 16,
    BSDSockets = 17,
    HTC = 18,
    TSC = 19,
    NCMContent = 20,
    SM = 21,
    RO = 22,
    GC = 23,
    SDMMC = 24,
    OVLN = 25,
    SPL = 26,
    Socket = 27,
    HTCLOW = 29,
    DDSF = 30,
    HTCFS = 31,
    Async = 32,
    Util = 33,
    TIPC = 35,
    ANIF = 37,
    ETHC = 100,
    I2C = 101,
    GPIO = 102,
    UART = 103,
    CPAD = 104,
    Settings = 105,
    FTM = 106,
    WLAN = 107,
    XCD = 108,
    TMP451 = 109,
    NIFM = 110,
    HwOpus = 111,
    LSM6DS3 = 112,
    Bluetooth = 113,
    VI = 114,
    NFP = 115,
    Time = 116,
    FGM = 117,
    OE = 118,
    BH1730FVC = 119,
    PCIe = 120,
    Friends = 121,
    BCAT = 122,
    SSLSrv = 123,
    Account = 124,
    News = 125,
    Mii = 126,
    NFC = 127,
    AM = 128,
    PlayReport = 129,
    AHID = 130,
    Qlaunch = 132,
    PCV = 133,
    USBPD = 134,
    BPC = 135,
    PSM = 136,
    NIM = 137,
    PSC = 138,
    TC = 139,
    USB = 140,
    NSD = 141,
    PCTL = 142,
    BTM = 143,
    LA = 144,
    ETicket = 145,
    NGC = 146,
    ERPT = 147,
    APM = 148,
    CEC = 149,
    Profiler = 150,
    ErrorUpload = 151,
    LIDBE = 152,
    Audio = 153,
    NPNS = 154,
    NPNSHTTPSTREAM = 155,
    ARP = 157,
    SWKBD = 158,
    BOOT = 159,
    NetDiag = 160,
    NFCMifare = 161,
    UserlandAssert = 162,
    Fatal = 163,
    NIMShop = 164,
    SPSM = 165,
    BGTC = 167,
    UserlandCrash = 168,
    SASBUS = 169,
    PI = 170,
    AudioCtrl = 172,
    LBL = 173,
    JIT = 175,
    HDCP = 176,
    OMM = 177,
    PDM = 178,
    OLSC = 179,
    SREPO = 180,
    Dauth = 181,
    STDFU = 182,
    DBG = 183,
    DHCPS = 186,
    SPI = 187,
    AVM = 188,
    PWM = 189,
    RTC = 191,
    Regulator = 192,
    LED = 193,
    SIO = 195,
    PCM = 196,
    CLKRST = 197,
    POWCTL = 198,
    AudioOld = 201,
    HID = 202,
    LDN = 203,
    CS = 204,
    Irsensor = 205,
    Capture = 206,
    Manu = 208,
    ATK = 209,
    WEB = 210,
    LCS = 211,
    GRC = 212,
    Repair = 213,
    Album = 214,
    RID = 215,
    Migration = 216,
    MigrationLdcServ = 217,
    HIDBUS = 218,
    ENS = 219,
    WebSocket = 223,
    DCDMTP = 227,
    PGL = 228,
    Notification = 229,
    INS = 230,
    LP2P = 231,
    RCD = 232,
    LCM40607 = 233,
    PRC = 235,
    TMAHTC = 237,
    ECTX = 238,
    MNPP = 239,
    HSHL = 240,
    CAPMTP = 242,
    DP2HDMI = 244,
    Cradle = 245,
    SProfile = 246,
    NDRM = 250,
    TSPM = 499,
    DevMenu = 500,
    GeneralWebApplet = 800,
    WifiWebAuthApplet = 809,
    WhitelistedApplet = 810,
    ShopN = 811,
}

/// Error description types
pub type Description = u32;

/// Converts an error description type into the value used in the error code.
pub trait IntoDescription {
    /// Converts the error description value into a `u32` .
    fn into_value(self) -> Description;
}

// Treat `u32` as a valid description type
impl IntoDescription for u32 {
    fn into_value(self) -> Description {
        self
    }
}

/// Raw error code type
pub type ResultCode = u32;

/// Converts an error enum into the raw error code
// TODO: Seal this trait
pub trait ToRawResultCode {
    /// Converts the error enum into a raw error code
    fn to_rc(self) -> ResultCode;
}

impl ToRawResultCode for u32 {
    fn to_rc(self) -> ResultCode {
        self
    }
}

impl ToRawResultCode for (Module, Description) {
    fn to_rc(self) -> ResultCode {
        // The module is shifted left by 9 bits, and the error code is OR'd with it.
        ((self.0 as u32) << 9) | self.1
    }
}

/// Error codes for kernel operations
///
/// This is an enum of all the known error codes returned by the kernel.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(u32)]
pub enum KernelError {
    OutOfSessions = 7,
    InvalidArgument = 14,
    NotImplemented = 33,
    NoSynchronizationObject = 57,
    TerminationRequested = 59,
    InvalidSize = 101,
    InvalidAddress = 102,
    OutOfResource = 103,
    OutOfMemory = 104,
    OutOfHandles = 105,
    InvalidCurrentMemory = 106,
    InvalidNewMemoryPermission = 108,
    InvalidMemoryRegion = 110,
    InvalidPriority = 112,
    InvalidCoreId = 113,
    InvalidHandle = 114,
    InvalidPointer = 115,
    InvalidCombination = 116,
    TimedOut = 117,
    Cancelled = 118,
    OutOfRange = 119,
    InvalidEnumValue = 120,
    NotFound = 121,
    Busy = 122,
    SessionClosed = 123,
    InvalidState = 125,
    ReservedUsed = 126,
    PortClosed = 131,
    LimitReached = 132,
    ReceiveListBroken = 258,
    OutOfAddressSpace = 259,
    MessageTooLarge = 260,
    InvalidId = 519,
}

impl PartialEq<u32> for KernelError {
    /// Compares the error code with a raw description value.
    fn eq(&self, other: &u32) -> bool {
        *self as u32 == *other
    }
}

impl PartialEq<KernelError> for u32 {
    /// Compares the error code with a raw description value.
    fn eq(&self, other: &KernelError) -> bool {
        *self == *other as u32
    }
}

impl IntoDescription for KernelError {
    fn into_value(self) -> u32 {
        self as u32
    }
}

impl ToRawResultCode for KernelError {
    fn to_rc(self) -> ResultCode {
        (Module::Kernel, self.into_value()).to_rc()
    }
}
