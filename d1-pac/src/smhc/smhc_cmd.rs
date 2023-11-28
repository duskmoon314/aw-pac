#[doc = "Register `smhc_cmd` reader"]
pub type R = crate::R<SMHC_CMD_SPEC>;
#[doc = "Register `smhc_cmd` writer"]
pub type W = crate::W<SMHC_CMD_SPEC>;
#[doc = "Field `cmd_idx` reader - CMD Index"]
pub type CMD_IDX_R = crate::FieldReader;
#[doc = "Field `cmd_idx` writer - CMD Index"]
pub type CMD_IDX_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `resp_rcv` reader - Response Receive"]
pub type RESP_RCV_R = crate::BitReader<RESP_RCV_A>;
#[doc = "Response Receive\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RESP_RCV_A {
    #[doc = "0: Command without response"]
    WITHOUT = 0,
    #[doc = "1: Command with response"]
    WITH = 1,
}
impl From<RESP_RCV_A> for bool {
    #[inline(always)]
    fn from(variant: RESP_RCV_A) -> Self {
        variant as u8 != 0
    }
}
impl RESP_RCV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RESP_RCV_A {
        match self.bits {
            false => RESP_RCV_A::WITHOUT,
            true => RESP_RCV_A::WITH,
        }
    }
    #[doc = "Command without response"]
    #[inline(always)]
    pub fn is_without(&self) -> bool {
        *self == RESP_RCV_A::WITHOUT
    }
    #[doc = "Command with response"]
    #[inline(always)]
    pub fn is_with(&self) -> bool {
        *self == RESP_RCV_A::WITH
    }
}
#[doc = "Field `resp_rcv` writer - Response Receive"]
pub type RESP_RCV_W<'a, REG> = crate::BitWriter<'a, REG, RESP_RCV_A>;
impl<'a, REG> RESP_RCV_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Command without response"]
    #[inline(always)]
    pub fn without(self) -> &'a mut crate::W<REG> {
        self.variant(RESP_RCV_A::WITHOUT)
    }
    #[doc = "Command with response"]
    #[inline(always)]
    pub fn with(self) -> &'a mut crate::W<REG> {
        self.variant(RESP_RCV_A::WITH)
    }
}
#[doc = "Field `long_resp` reader - Response Type"]
pub type LONG_RESP_R = crate::BitReader<LONG_RESP_A>;
#[doc = "Response Type\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LONG_RESP_A {
    #[doc = "0: Short Response (48 bits)"]
    SHORT = 0,
    #[doc = "1: Long Response (136 bits)"]
    LONG = 1,
}
impl From<LONG_RESP_A> for bool {
    #[inline(always)]
    fn from(variant: LONG_RESP_A) -> Self {
        variant as u8 != 0
    }
}
impl LONG_RESP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LONG_RESP_A {
        match self.bits {
            false => LONG_RESP_A::SHORT,
            true => LONG_RESP_A::LONG,
        }
    }
    #[doc = "Short Response (48 bits)"]
    #[inline(always)]
    pub fn is_short(&self) -> bool {
        *self == LONG_RESP_A::SHORT
    }
    #[doc = "Long Response (136 bits)"]
    #[inline(always)]
    pub fn is_long(&self) -> bool {
        *self == LONG_RESP_A::LONG
    }
}
#[doc = "Field `long_resp` writer - Response Type"]
pub type LONG_RESP_W<'a, REG> = crate::BitWriter<'a, REG, LONG_RESP_A>;
impl<'a, REG> LONG_RESP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Short Response (48 bits)"]
    #[inline(always)]
    pub fn short(self) -> &'a mut crate::W<REG> {
        self.variant(LONG_RESP_A::SHORT)
    }
    #[doc = "Long Response (136 bits)"]
    #[inline(always)]
    pub fn long(self) -> &'a mut crate::W<REG> {
        self.variant(LONG_RESP_A::LONG)
    }
}
#[doc = "Field `chk_resp_crc` reader - Check Response CRC"]
pub type CHK_RESP_CRC_R = crate::BitReader<CHK_RESP_CRC_A>;
#[doc = "Check Response CRC\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CHK_RESP_CRC_A {
    #[doc = "0: Do not check response CRC"]
    NOT_CHECK = 0,
    #[doc = "1: Check response CRC"]
    CHECK = 1,
}
impl From<CHK_RESP_CRC_A> for bool {
    #[inline(always)]
    fn from(variant: CHK_RESP_CRC_A) -> Self {
        variant as u8 != 0
    }
}
impl CHK_RESP_CRC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CHK_RESP_CRC_A {
        match self.bits {
            false => CHK_RESP_CRC_A::NOT_CHECK,
            true => CHK_RESP_CRC_A::CHECK,
        }
    }
    #[doc = "Do not check response CRC"]
    #[inline(always)]
    pub fn is_not_check(&self) -> bool {
        *self == CHK_RESP_CRC_A::NOT_CHECK
    }
    #[doc = "Check response CRC"]
    #[inline(always)]
    pub fn is_check(&self) -> bool {
        *self == CHK_RESP_CRC_A::CHECK
    }
}
#[doc = "Field `chk_resp_crc` writer - Check Response CRC"]
pub type CHK_RESP_CRC_W<'a, REG> = crate::BitWriter<'a, REG, CHK_RESP_CRC_A>;
impl<'a, REG> CHK_RESP_CRC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Do not check response CRC"]
    #[inline(always)]
    pub fn not_check(self) -> &'a mut crate::W<REG> {
        self.variant(CHK_RESP_CRC_A::NOT_CHECK)
    }
    #[doc = "Check response CRC"]
    #[inline(always)]
    pub fn check(self) -> &'a mut crate::W<REG> {
        self.variant(CHK_RESP_CRC_A::CHECK)
    }
}
#[doc = "Field `data_trans` reader - Data Transfer"]
pub type DATA_TRANS_R = crate::BitReader<DATA_TRANS_A>;
#[doc = "Data Transfer\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DATA_TRANS_A {
    #[doc = "0: Without data transfer"]
    WITHOUT = 0,
    #[doc = "1: With data transfer"]
    WITH = 1,
}
impl From<DATA_TRANS_A> for bool {
    #[inline(always)]
    fn from(variant: DATA_TRANS_A) -> Self {
        variant as u8 != 0
    }
}
impl DATA_TRANS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DATA_TRANS_A {
        match self.bits {
            false => DATA_TRANS_A::WITHOUT,
            true => DATA_TRANS_A::WITH,
        }
    }
    #[doc = "Without data transfer"]
    #[inline(always)]
    pub fn is_without(&self) -> bool {
        *self == DATA_TRANS_A::WITHOUT
    }
    #[doc = "With data transfer"]
    #[inline(always)]
    pub fn is_with(&self) -> bool {
        *self == DATA_TRANS_A::WITH
    }
}
#[doc = "Field `data_trans` writer - Data Transfer"]
pub type DATA_TRANS_W<'a, REG> = crate::BitWriter<'a, REG, DATA_TRANS_A>;
impl<'a, REG> DATA_TRANS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Without data transfer"]
    #[inline(always)]
    pub fn without(self) -> &'a mut crate::W<REG> {
        self.variant(DATA_TRANS_A::WITHOUT)
    }
    #[doc = "With data transfer"]
    #[inline(always)]
    pub fn with(self) -> &'a mut crate::W<REG> {
        self.variant(DATA_TRANS_A::WITH)
    }
}
#[doc = "Field `trans_dir` reader - Transfer Direction"]
pub type TRANS_DIR_R = crate::BitReader<TRANS_DIR_A>;
#[doc = "Transfer Direction\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TRANS_DIR_A {
    #[doc = "0: Read operation"]
    READ = 0,
    #[doc = "1: Write operation"]
    WRITE = 1,
}
impl From<TRANS_DIR_A> for bool {
    #[inline(always)]
    fn from(variant: TRANS_DIR_A) -> Self {
        variant as u8 != 0
    }
}
impl TRANS_DIR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TRANS_DIR_A {
        match self.bits {
            false => TRANS_DIR_A::READ,
            true => TRANS_DIR_A::WRITE,
        }
    }
    #[doc = "Read operation"]
    #[inline(always)]
    pub fn is_read(&self) -> bool {
        *self == TRANS_DIR_A::READ
    }
    #[doc = "Write operation"]
    #[inline(always)]
    pub fn is_write(&self) -> bool {
        *self == TRANS_DIR_A::WRITE
    }
}
#[doc = "Field `trans_dir` writer - Transfer Direction"]
pub type TRANS_DIR_W<'a, REG> = crate::BitWriter<'a, REG, TRANS_DIR_A>;
impl<'a, REG> TRANS_DIR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Read operation"]
    #[inline(always)]
    pub fn read(self) -> &'a mut crate::W<REG> {
        self.variant(TRANS_DIR_A::READ)
    }
    #[doc = "Write operation"]
    #[inline(always)]
    pub fn write(self) -> &'a mut crate::W<REG> {
        self.variant(TRANS_DIR_A::WRITE)
    }
}
#[doc = "Field `trans_mode` reader - Transfer Mode"]
pub type TRANS_MODE_R = crate::BitReader<TRANS_MODE_A>;
#[doc = "Transfer Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TRANS_MODE_A {
    #[doc = "0: Block data transfer command"]
    BLOCK = 0,
    #[doc = "1: Stream data transfer commmand"]
    STREAM = 1,
}
impl From<TRANS_MODE_A> for bool {
    #[inline(always)]
    fn from(variant: TRANS_MODE_A) -> Self {
        variant as u8 != 0
    }
}
impl TRANS_MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TRANS_MODE_A {
        match self.bits {
            false => TRANS_MODE_A::BLOCK,
            true => TRANS_MODE_A::STREAM,
        }
    }
    #[doc = "Block data transfer command"]
    #[inline(always)]
    pub fn is_block(&self) -> bool {
        *self == TRANS_MODE_A::BLOCK
    }
    #[doc = "Stream data transfer commmand"]
    #[inline(always)]
    pub fn is_stream(&self) -> bool {
        *self == TRANS_MODE_A::STREAM
    }
}
#[doc = "Field `trans_mode` writer - Transfer Mode"]
pub type TRANS_MODE_W<'a, REG> = crate::BitWriter<'a, REG, TRANS_MODE_A>;
impl<'a, REG> TRANS_MODE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Block data transfer command"]
    #[inline(always)]
    pub fn block(self) -> &'a mut crate::W<REG> {
        self.variant(TRANS_MODE_A::BLOCK)
    }
    #[doc = "Stream data transfer commmand"]
    #[inline(always)]
    pub fn stream(self) -> &'a mut crate::W<REG> {
        self.variant(TRANS_MODE_A::STREAM)
    }
}
#[doc = "Field `stop_cmd_flag` reader - Send Stop CMD Automatically (CMD12)"]
pub type STOP_CMD_FLAG_R = crate::BitReader<STOP_CMD_FLAG_A>;
#[doc = "Send Stop CMD Automatically (CMD12)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum STOP_CMD_FLAG_A {
    #[doc = "0: Do not send stop command at the end of the data transfer"]
    NO_STOP = 0,
    #[doc = "1: Send stop command automatically at the end of the data transfer"]
    AUTO_STOP = 1,
}
impl From<STOP_CMD_FLAG_A> for bool {
    #[inline(always)]
    fn from(variant: STOP_CMD_FLAG_A) -> Self {
        variant as u8 != 0
    }
}
impl STOP_CMD_FLAG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> STOP_CMD_FLAG_A {
        match self.bits {
            false => STOP_CMD_FLAG_A::NO_STOP,
            true => STOP_CMD_FLAG_A::AUTO_STOP,
        }
    }
    #[doc = "Do not send stop command at the end of the data transfer"]
    #[inline(always)]
    pub fn is_no_stop(&self) -> bool {
        *self == STOP_CMD_FLAG_A::NO_STOP
    }
    #[doc = "Send stop command automatically at the end of the data transfer"]
    #[inline(always)]
    pub fn is_auto_stop(&self) -> bool {
        *self == STOP_CMD_FLAG_A::AUTO_STOP
    }
}
#[doc = "Field `stop_cmd_flag` writer - Send Stop CMD Automatically (CMD12)"]
pub type STOP_CMD_FLAG_W<'a, REG> = crate::BitWriter<'a, REG, STOP_CMD_FLAG_A>;
impl<'a, REG> STOP_CMD_FLAG_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Do not send stop command at the end of the data transfer"]
    #[inline(always)]
    pub fn no_stop(self) -> &'a mut crate::W<REG> {
        self.variant(STOP_CMD_FLAG_A::NO_STOP)
    }
    #[doc = "Send stop command automatically at the end of the data transfer"]
    #[inline(always)]
    pub fn auto_stop(self) -> &'a mut crate::W<REG> {
        self.variant(STOP_CMD_FLAG_A::AUTO_STOP)
    }
}
#[doc = "Field `wait_pre_over` reader - Wait for Data Transfer Over"]
pub type WAIT_PRE_OVER_R = crate::BitReader<WAIT_PRE_OVER_A>;
#[doc = "Wait for Data Transfer Over\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WAIT_PRE_OVER_A {
    #[doc = "0: Send command at once, does not care about data transferring"]
    AT_ONCE = 0,
    #[doc = "1: Wait for data transfer completion before sending the current command"]
    WAIT = 1,
}
impl From<WAIT_PRE_OVER_A> for bool {
    #[inline(always)]
    fn from(variant: WAIT_PRE_OVER_A) -> Self {
        variant as u8 != 0
    }
}
impl WAIT_PRE_OVER_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> WAIT_PRE_OVER_A {
        match self.bits {
            false => WAIT_PRE_OVER_A::AT_ONCE,
            true => WAIT_PRE_OVER_A::WAIT,
        }
    }
    #[doc = "Send command at once, does not care about data transferring"]
    #[inline(always)]
    pub fn is_at_once(&self) -> bool {
        *self == WAIT_PRE_OVER_A::AT_ONCE
    }
    #[doc = "Wait for data transfer completion before sending the current command"]
    #[inline(always)]
    pub fn is_wait(&self) -> bool {
        *self == WAIT_PRE_OVER_A::WAIT
    }
}
#[doc = "Field `wait_pre_over` writer - Wait for Data Transfer Over"]
pub type WAIT_PRE_OVER_W<'a, REG> = crate::BitWriter<'a, REG, WAIT_PRE_OVER_A>;
impl<'a, REG> WAIT_PRE_OVER_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Send command at once, does not care about data transferring"]
    #[inline(always)]
    pub fn at_once(self) -> &'a mut crate::W<REG> {
        self.variant(WAIT_PRE_OVER_A::AT_ONCE)
    }
    #[doc = "Wait for data transfer completion before sending the current command"]
    #[inline(always)]
    pub fn wait(self) -> &'a mut crate::W<REG> {
        self.variant(WAIT_PRE_OVER_A::WAIT)
    }
}
#[doc = "Field `stop_abt_cmd` reader - Stop Abort Command"]
pub type STOP_ABT_CMD_R = crate::BitReader<STOP_ABT_CMD_A>;
#[doc = "Stop Abort Command\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum STOP_ABT_CMD_A {
    #[doc = "0: Normal command sending"]
    NORMAL = 0,
    #[doc = "1: Send Stop or Abort command to stop the current data transfer in progress"]
    STOP = 1,
}
impl From<STOP_ABT_CMD_A> for bool {
    #[inline(always)]
    fn from(variant: STOP_ABT_CMD_A) -> Self {
        variant as u8 != 0
    }
}
impl STOP_ABT_CMD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> STOP_ABT_CMD_A {
        match self.bits {
            false => STOP_ABT_CMD_A::NORMAL,
            true => STOP_ABT_CMD_A::STOP,
        }
    }
    #[doc = "Normal command sending"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == STOP_ABT_CMD_A::NORMAL
    }
    #[doc = "Send Stop or Abort command to stop the current data transfer in progress"]
    #[inline(always)]
    pub fn is_stop(&self) -> bool {
        *self == STOP_ABT_CMD_A::STOP
    }
}
#[doc = "Field `stop_abt_cmd` writer - Stop Abort Command"]
pub type STOP_ABT_CMD_W<'a, REG> = crate::BitWriter<'a, REG, STOP_ABT_CMD_A>;
impl<'a, REG> STOP_ABT_CMD_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal command sending"]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(STOP_ABT_CMD_A::NORMAL)
    }
    #[doc = "Send Stop or Abort command to stop the current data transfer in progress"]
    #[inline(always)]
    pub fn stop(self) -> &'a mut crate::W<REG> {
        self.variant(STOP_ABT_CMD_A::STOP)
    }
}
#[doc = "Field `send_init_seq` reader - Send Initialization"]
pub type SEND_INIT_SEQ_R = crate::BitReader<SEND_INIT_SEQ_A>;
#[doc = "Send Initialization\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SEND_INIT_SEQ_A {
    #[doc = "0: Normal command sending"]
    NORMAL = 0,
    #[doc = "1: Send initialization sequence before sending this command"]
    INIT_CMD = 1,
}
impl From<SEND_INIT_SEQ_A> for bool {
    #[inline(always)]
    fn from(variant: SEND_INIT_SEQ_A) -> Self {
        variant as u8 != 0
    }
}
impl SEND_INIT_SEQ_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SEND_INIT_SEQ_A {
        match self.bits {
            false => SEND_INIT_SEQ_A::NORMAL,
            true => SEND_INIT_SEQ_A::INIT_CMD,
        }
    }
    #[doc = "Normal command sending"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == SEND_INIT_SEQ_A::NORMAL
    }
    #[doc = "Send initialization sequence before sending this command"]
    #[inline(always)]
    pub fn is_init_cmd(&self) -> bool {
        *self == SEND_INIT_SEQ_A::INIT_CMD
    }
}
#[doc = "Field `send_init_seq` writer - Send Initialization"]
pub type SEND_INIT_SEQ_W<'a, REG> = crate::BitWriter<'a, REG, SEND_INIT_SEQ_A>;
impl<'a, REG> SEND_INIT_SEQ_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal command sending"]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(SEND_INIT_SEQ_A::NORMAL)
    }
    #[doc = "Send initialization sequence before sending this command"]
    #[inline(always)]
    pub fn init_cmd(self) -> &'a mut crate::W<REG> {
        self.variant(SEND_INIT_SEQ_A::INIT_CMD)
    }
}
#[doc = "Field `prg_clk` reader - Change Clock"]
pub type PRG_CLK_R = crate::BitReader<PRG_CLK_A>;
#[doc = "Change Clock\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PRG_CLK_A {
    #[doc = "0: Normal command"]
    NORMAL = 0,
    #[doc = "1: Change Card Clock"]
    CHANGE = 1,
}
impl From<PRG_CLK_A> for bool {
    #[inline(always)]
    fn from(variant: PRG_CLK_A) -> Self {
        variant as u8 != 0
    }
}
impl PRG_CLK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PRG_CLK_A {
        match self.bits {
            false => PRG_CLK_A::NORMAL,
            true => PRG_CLK_A::CHANGE,
        }
    }
    #[doc = "Normal command"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == PRG_CLK_A::NORMAL
    }
    #[doc = "Change Card Clock"]
    #[inline(always)]
    pub fn is_change(&self) -> bool {
        *self == PRG_CLK_A::CHANGE
    }
}
#[doc = "Field `prg_clk` writer - Change Clock"]
pub type PRG_CLK_W<'a, REG> = crate::BitWriter<'a, REG, PRG_CLK_A>;
impl<'a, REG> PRG_CLK_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal command"]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(PRG_CLK_A::NORMAL)
    }
    #[doc = "Change Card Clock"]
    #[inline(always)]
    pub fn change(self) -> &'a mut crate::W<REG> {
        self.variant(PRG_CLK_A::CHANGE)
    }
}
#[doc = "Field `boot_mod` reader - Boot Mode"]
pub type BOOT_MOD_R = crate::FieldReader<BOOT_MOD_A>;
#[doc = "Boot Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum BOOT_MOD_A {
    #[doc = "0: Normal command"]
    NORMAL = 0,
    #[doc = "1: Mandatory Boot operation"]
    MANDATORY_BOOT = 1,
    #[doc = "2: Alternate Boot operation"]
    ALTERNATE_BOOT = 2,
}
impl From<BOOT_MOD_A> for u8 {
    #[inline(always)]
    fn from(variant: BOOT_MOD_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for BOOT_MOD_A {
    type Ux = u8;
}
impl BOOT_MOD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<BOOT_MOD_A> {
        match self.bits {
            0 => Some(BOOT_MOD_A::NORMAL),
            1 => Some(BOOT_MOD_A::MANDATORY_BOOT),
            2 => Some(BOOT_MOD_A::ALTERNATE_BOOT),
            _ => None,
        }
    }
    #[doc = "Normal command"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == BOOT_MOD_A::NORMAL
    }
    #[doc = "Mandatory Boot operation"]
    #[inline(always)]
    pub fn is_mandatory_boot(&self) -> bool {
        *self == BOOT_MOD_A::MANDATORY_BOOT
    }
    #[doc = "Alternate Boot operation"]
    #[inline(always)]
    pub fn is_alternate_boot(&self) -> bool {
        *self == BOOT_MOD_A::ALTERNATE_BOOT
    }
}
#[doc = "Field `boot_mod` writer - Boot Mode"]
pub type BOOT_MOD_W<'a, REG> = crate::FieldWriter<'a, REG, 2, BOOT_MOD_A>;
impl<'a, REG> BOOT_MOD_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Normal command"]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(BOOT_MOD_A::NORMAL)
    }
    #[doc = "Mandatory Boot operation"]
    #[inline(always)]
    pub fn mandatory_boot(self) -> &'a mut crate::W<REG> {
        self.variant(BOOT_MOD_A::MANDATORY_BOOT)
    }
    #[doc = "Alternate Boot operation"]
    #[inline(always)]
    pub fn alternate_boot(self) -> &'a mut crate::W<REG> {
        self.variant(BOOT_MOD_A::ALTERNATE_BOOT)
    }
}
#[doc = "Field `exp_boot_ack` reader - Expect Boot Acknowledge"]
pub type EXP_BOOT_ACK_R = crate::BitReader;
#[doc = "Field `exp_boot_ack` writer - Expect Boot Acknowledge"]
pub type EXP_BOOT_ACK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `boot_abt` reader - Boot Abort"]
pub type BOOT_ABT_R = crate::BitReader;
#[doc = "Field `boot_abt` writer - Boot Abort"]
pub type BOOT_ABT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `vol_sw` reader - Voltage Switch"]
pub type VOL_SW_R = crate::BitReader<VOL_SW_A>;
#[doc = "Voltage Switch\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VOL_SW_A {
    #[doc = "0: Normal command"]
    NORMAL = 0,
    #[doc = "1: Voltage switch command, set for CMD11 only"]
    VOLTAGE_SWITCH = 1,
}
impl From<VOL_SW_A> for bool {
    #[inline(always)]
    fn from(variant: VOL_SW_A) -> Self {
        variant as u8 != 0
    }
}
impl VOL_SW_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> VOL_SW_A {
        match self.bits {
            false => VOL_SW_A::NORMAL,
            true => VOL_SW_A::VOLTAGE_SWITCH,
        }
    }
    #[doc = "Normal command"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == VOL_SW_A::NORMAL
    }
    #[doc = "Voltage switch command, set for CMD11 only"]
    #[inline(always)]
    pub fn is_voltage_switch(&self) -> bool {
        *self == VOL_SW_A::VOLTAGE_SWITCH
    }
}
#[doc = "Field `vol_sw` writer - Voltage Switch"]
pub type VOL_SW_W<'a, REG> = crate::BitWriter<'a, REG, VOL_SW_A>;
impl<'a, REG> VOL_SW_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal command"]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(VOL_SW_A::NORMAL)
    }
    #[doc = "Voltage switch command, set for CMD11 only"]
    #[inline(always)]
    pub fn voltage_switch(self) -> &'a mut crate::W<REG> {
        self.variant(VOL_SW_A::VOLTAGE_SWITCH)
    }
}
#[doc = "Field `cmd_load` reader - Start Command"]
pub type CMD_LOAD_R = crate::BitReader;
#[doc = "Field `cmd_load` writer - Start Command"]
pub type CMD_LOAD_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:5 - CMD Index"]
    #[inline(always)]
    pub fn cmd_idx(&self) -> CMD_IDX_R {
        CMD_IDX_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 6 - Response Receive"]
    #[inline(always)]
    pub fn resp_rcv(&self) -> RESP_RCV_R {
        RESP_RCV_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Response Type"]
    #[inline(always)]
    pub fn long_resp(&self) -> LONG_RESP_R {
        LONG_RESP_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Check Response CRC"]
    #[inline(always)]
    pub fn chk_resp_crc(&self) -> CHK_RESP_CRC_R {
        CHK_RESP_CRC_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Data Transfer"]
    #[inline(always)]
    pub fn data_trans(&self) -> DATA_TRANS_R {
        DATA_TRANS_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Transfer Direction"]
    #[inline(always)]
    pub fn trans_dir(&self) -> TRANS_DIR_R {
        TRANS_DIR_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Transfer Mode"]
    #[inline(always)]
    pub fn trans_mode(&self) -> TRANS_MODE_R {
        TRANS_MODE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Send Stop CMD Automatically (CMD12)"]
    #[inline(always)]
    pub fn stop_cmd_flag(&self) -> STOP_CMD_FLAG_R {
        STOP_CMD_FLAG_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Wait for Data Transfer Over"]
    #[inline(always)]
    pub fn wait_pre_over(&self) -> WAIT_PRE_OVER_R {
        WAIT_PRE_OVER_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Stop Abort Command"]
    #[inline(always)]
    pub fn stop_abt_cmd(&self) -> STOP_ABT_CMD_R {
        STOP_ABT_CMD_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Send Initialization"]
    #[inline(always)]
    pub fn send_init_seq(&self) -> SEND_INIT_SEQ_R {
        SEND_INIT_SEQ_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 21 - Change Clock"]
    #[inline(always)]
    pub fn prg_clk(&self) -> PRG_CLK_R {
        PRG_CLK_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bits 24:25 - Boot Mode"]
    #[inline(always)]
    pub fn boot_mod(&self) -> BOOT_MOD_R {
        BOOT_MOD_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bit 26 - Expect Boot Acknowledge"]
    #[inline(always)]
    pub fn exp_boot_ack(&self) -> EXP_BOOT_ACK_R {
        EXP_BOOT_ACK_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Boot Abort"]
    #[inline(always)]
    pub fn boot_abt(&self) -> BOOT_ABT_R {
        BOOT_ABT_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Voltage Switch"]
    #[inline(always)]
    pub fn vol_sw(&self) -> VOL_SW_R {
        VOL_SW_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 31 - Start Command"]
    #[inline(always)]
    pub fn cmd_load(&self) -> CMD_LOAD_R {
        CMD_LOAD_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - CMD Index"]
    #[inline(always)]
    #[must_use]
    pub fn cmd_idx(&mut self) -> CMD_IDX_W<SMHC_CMD_SPEC> {
        CMD_IDX_W::new(self, 0)
    }
    #[doc = "Bit 6 - Response Receive"]
    #[inline(always)]
    #[must_use]
    pub fn resp_rcv(&mut self) -> RESP_RCV_W<SMHC_CMD_SPEC> {
        RESP_RCV_W::new(self, 6)
    }
    #[doc = "Bit 7 - Response Type"]
    #[inline(always)]
    #[must_use]
    pub fn long_resp(&mut self) -> LONG_RESP_W<SMHC_CMD_SPEC> {
        LONG_RESP_W::new(self, 7)
    }
    #[doc = "Bit 8 - Check Response CRC"]
    #[inline(always)]
    #[must_use]
    pub fn chk_resp_crc(&mut self) -> CHK_RESP_CRC_W<SMHC_CMD_SPEC> {
        CHK_RESP_CRC_W::new(self, 8)
    }
    #[doc = "Bit 9 - Data Transfer"]
    #[inline(always)]
    #[must_use]
    pub fn data_trans(&mut self) -> DATA_TRANS_W<SMHC_CMD_SPEC> {
        DATA_TRANS_W::new(self, 9)
    }
    #[doc = "Bit 10 - Transfer Direction"]
    #[inline(always)]
    #[must_use]
    pub fn trans_dir(&mut self) -> TRANS_DIR_W<SMHC_CMD_SPEC> {
        TRANS_DIR_W::new(self, 10)
    }
    #[doc = "Bit 11 - Transfer Mode"]
    #[inline(always)]
    #[must_use]
    pub fn trans_mode(&mut self) -> TRANS_MODE_W<SMHC_CMD_SPEC> {
        TRANS_MODE_W::new(self, 11)
    }
    #[doc = "Bit 12 - Send Stop CMD Automatically (CMD12)"]
    #[inline(always)]
    #[must_use]
    pub fn stop_cmd_flag(&mut self) -> STOP_CMD_FLAG_W<SMHC_CMD_SPEC> {
        STOP_CMD_FLAG_W::new(self, 12)
    }
    #[doc = "Bit 13 - Wait for Data Transfer Over"]
    #[inline(always)]
    #[must_use]
    pub fn wait_pre_over(&mut self) -> WAIT_PRE_OVER_W<SMHC_CMD_SPEC> {
        WAIT_PRE_OVER_W::new(self, 13)
    }
    #[doc = "Bit 14 - Stop Abort Command"]
    #[inline(always)]
    #[must_use]
    pub fn stop_abt_cmd(&mut self) -> STOP_ABT_CMD_W<SMHC_CMD_SPEC> {
        STOP_ABT_CMD_W::new(self, 14)
    }
    #[doc = "Bit 15 - Send Initialization"]
    #[inline(always)]
    #[must_use]
    pub fn send_init_seq(&mut self) -> SEND_INIT_SEQ_W<SMHC_CMD_SPEC> {
        SEND_INIT_SEQ_W::new(self, 15)
    }
    #[doc = "Bit 21 - Change Clock"]
    #[inline(always)]
    #[must_use]
    pub fn prg_clk(&mut self) -> PRG_CLK_W<SMHC_CMD_SPEC> {
        PRG_CLK_W::new(self, 21)
    }
    #[doc = "Bits 24:25 - Boot Mode"]
    #[inline(always)]
    #[must_use]
    pub fn boot_mod(&mut self) -> BOOT_MOD_W<SMHC_CMD_SPEC> {
        BOOT_MOD_W::new(self, 24)
    }
    #[doc = "Bit 26 - Expect Boot Acknowledge"]
    #[inline(always)]
    #[must_use]
    pub fn exp_boot_ack(&mut self) -> EXP_BOOT_ACK_W<SMHC_CMD_SPEC> {
        EXP_BOOT_ACK_W::new(self, 26)
    }
    #[doc = "Bit 27 - Boot Abort"]
    #[inline(always)]
    #[must_use]
    pub fn boot_abt(&mut self) -> BOOT_ABT_W<SMHC_CMD_SPEC> {
        BOOT_ABT_W::new(self, 27)
    }
    #[doc = "Bit 28 - Voltage Switch"]
    #[inline(always)]
    #[must_use]
    pub fn vol_sw(&mut self) -> VOL_SW_W<SMHC_CMD_SPEC> {
        VOL_SW_W::new(self, 28)
    }
    #[doc = "Bit 31 - Start Command"]
    #[inline(always)]
    #[must_use]
    pub fn cmd_load(&mut self) -> CMD_LOAD_W<SMHC_CMD_SPEC> {
        CMD_LOAD_W::new(self, 31)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Command Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`smhc_cmd::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`smhc_cmd::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SMHC_CMD_SPEC;
impl crate::RegisterSpec for SMHC_CMD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`smhc_cmd::R`](R) reader structure"]
impl crate::Readable for SMHC_CMD_SPEC {}
#[doc = "`write(|w| ..)` method takes [`smhc_cmd::W`](W) writer structure"]
impl crate::Writable for SMHC_CMD_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets smhc_cmd to value 0"]
impl crate::Resettable for SMHC_CMD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
