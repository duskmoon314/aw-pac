#[doc = "Register `smhc_ntsr` reader"]
pub type R = crate::R<SMHC_NTSR_SPEC>;
#[doc = "Register `smhc_ntsr` writer"]
pub type W = crate::W<SMHC_NTSR_SPEC>;
#[doc = "Field `hs400_new_sample_en` reader - "]
pub type HS400_NEW_SAMPLE_EN_R = crate::BitReader<HS400_NEW_SAMPLE_EN_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HS400_NEW_SAMPLE_EN_A {
    #[doc = "0: Disable hs400 new sample method"]
    DISABLE = 0,
    #[doc = "1: Enable hs400 new sample method"]
    ENABLE = 1,
}
impl From<HS400_NEW_SAMPLE_EN_A> for bool {
    #[inline(always)]
    fn from(variant: HS400_NEW_SAMPLE_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl HS400_NEW_SAMPLE_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HS400_NEW_SAMPLE_EN_A {
        match self.bits {
            false => HS400_NEW_SAMPLE_EN_A::DISABLE,
            true => HS400_NEW_SAMPLE_EN_A::ENABLE,
        }
    }
    #[doc = "Disable hs400 new sample method"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == HS400_NEW_SAMPLE_EN_A::DISABLE
    }
    #[doc = "Enable hs400 new sample method"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == HS400_NEW_SAMPLE_EN_A::ENABLE
    }
}
#[doc = "Field `hs400_new_sample_en` writer - "]
pub type HS400_NEW_SAMPLE_EN_W<'a, REG> = crate::BitWriter<'a, REG, HS400_NEW_SAMPLE_EN_A>;
impl<'a, REG> HS400_NEW_SAMPLE_EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable hs400 new sample method"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(HS400_NEW_SAMPLE_EN_A::DISABLE)
    }
    #[doc = "Enable hs400 new sample method"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(HS400_NEW_SAMPLE_EN_A::ENABLE)
    }
}
#[doc = "Field `cmd_sample_timing_phase` reader - "]
pub type CMD_SAMPLE_TIMING_PHASE_R = crate::FieldReader<CMD_SAMPLE_TIMING_PHASE_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CMD_SAMPLE_TIMING_PHASE_A {
    #[doc = "0: Sample timing phase offset 90"]
    O90 = 0,
    #[doc = "1: Sample timing phase offset 180"]
    O180 = 1,
    #[doc = "2: Sample timing phase offset 270"]
    O270 = 2,
    #[doc = "3: Ignore"]
    O0 = 3,
}
impl From<CMD_SAMPLE_TIMING_PHASE_A> for u8 {
    #[inline(always)]
    fn from(variant: CMD_SAMPLE_TIMING_PHASE_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CMD_SAMPLE_TIMING_PHASE_A {
    type Ux = u8;
}
impl CMD_SAMPLE_TIMING_PHASE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CMD_SAMPLE_TIMING_PHASE_A {
        match self.bits {
            0 => CMD_SAMPLE_TIMING_PHASE_A::O90,
            1 => CMD_SAMPLE_TIMING_PHASE_A::O180,
            2 => CMD_SAMPLE_TIMING_PHASE_A::O270,
            3 => CMD_SAMPLE_TIMING_PHASE_A::O0,
            _ => unreachable!(),
        }
    }
    #[doc = "Sample timing phase offset 90"]
    #[inline(always)]
    pub fn is_o90(&self) -> bool {
        *self == CMD_SAMPLE_TIMING_PHASE_A::O90
    }
    #[doc = "Sample timing phase offset 180"]
    #[inline(always)]
    pub fn is_o180(&self) -> bool {
        *self == CMD_SAMPLE_TIMING_PHASE_A::O180
    }
    #[doc = "Sample timing phase offset 270"]
    #[inline(always)]
    pub fn is_o270(&self) -> bool {
        *self == CMD_SAMPLE_TIMING_PHASE_A::O270
    }
    #[doc = "Ignore"]
    #[inline(always)]
    pub fn is_o0(&self) -> bool {
        *self == CMD_SAMPLE_TIMING_PHASE_A::O0
    }
}
#[doc = "Field `cmd_sample_timing_phase` writer - "]
pub type CMD_SAMPLE_TIMING_PHASE_W<'a, REG> =
    crate::FieldWriterSafe<'a, REG, 2, CMD_SAMPLE_TIMING_PHASE_A>;
impl<'a, REG> CMD_SAMPLE_TIMING_PHASE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Sample timing phase offset 90"]
    #[inline(always)]
    pub fn o90(self) -> &'a mut crate::W<REG> {
        self.variant(CMD_SAMPLE_TIMING_PHASE_A::O90)
    }
    #[doc = "Sample timing phase offset 180"]
    #[inline(always)]
    pub fn o180(self) -> &'a mut crate::W<REG> {
        self.variant(CMD_SAMPLE_TIMING_PHASE_A::O180)
    }
    #[doc = "Sample timing phase offset 270"]
    #[inline(always)]
    pub fn o270(self) -> &'a mut crate::W<REG> {
        self.variant(CMD_SAMPLE_TIMING_PHASE_A::O270)
    }
    #[doc = "Ignore"]
    #[inline(always)]
    pub fn o0(self) -> &'a mut crate::W<REG> {
        self.variant(CMD_SAMPLE_TIMING_PHASE_A::O0)
    }
}
#[doc = "Field `dat_sample_timing_phase` reader - "]
pub type DAT_SAMPLE_TIMING_PHASE_R = crate::FieldReader<DAT_SAMPLE_TIMING_PHASE_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DAT_SAMPLE_TIMING_PHASE_A {
    #[doc = "0: Sample timing phase offset 90"]
    O90 = 0,
    #[doc = "1: Sample timing phase offset 180"]
    O180 = 1,
    #[doc = "2: Sample timing phase offset 270"]
    O270 = 2,
    #[doc = "3: Sample timing phase offset 0 (only for SD2 hs400 mode)"]
    O0 = 3,
}
impl From<DAT_SAMPLE_TIMING_PHASE_A> for u8 {
    #[inline(always)]
    fn from(variant: DAT_SAMPLE_TIMING_PHASE_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DAT_SAMPLE_TIMING_PHASE_A {
    type Ux = u8;
}
impl DAT_SAMPLE_TIMING_PHASE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DAT_SAMPLE_TIMING_PHASE_A {
        match self.bits {
            0 => DAT_SAMPLE_TIMING_PHASE_A::O90,
            1 => DAT_SAMPLE_TIMING_PHASE_A::O180,
            2 => DAT_SAMPLE_TIMING_PHASE_A::O270,
            3 => DAT_SAMPLE_TIMING_PHASE_A::O0,
            _ => unreachable!(),
        }
    }
    #[doc = "Sample timing phase offset 90"]
    #[inline(always)]
    pub fn is_o90(&self) -> bool {
        *self == DAT_SAMPLE_TIMING_PHASE_A::O90
    }
    #[doc = "Sample timing phase offset 180"]
    #[inline(always)]
    pub fn is_o180(&self) -> bool {
        *self == DAT_SAMPLE_TIMING_PHASE_A::O180
    }
    #[doc = "Sample timing phase offset 270"]
    #[inline(always)]
    pub fn is_o270(&self) -> bool {
        *self == DAT_SAMPLE_TIMING_PHASE_A::O270
    }
    #[doc = "Sample timing phase offset 0 (only for SD2 hs400 mode)"]
    #[inline(always)]
    pub fn is_o0(&self) -> bool {
        *self == DAT_SAMPLE_TIMING_PHASE_A::O0
    }
}
#[doc = "Field `dat_sample_timing_phase` writer - "]
pub type DAT_SAMPLE_TIMING_PHASE_W<'a, REG> =
    crate::FieldWriterSafe<'a, REG, 2, DAT_SAMPLE_TIMING_PHASE_A>;
impl<'a, REG> DAT_SAMPLE_TIMING_PHASE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Sample timing phase offset 90"]
    #[inline(always)]
    pub fn o90(self) -> &'a mut crate::W<REG> {
        self.variant(DAT_SAMPLE_TIMING_PHASE_A::O90)
    }
    #[doc = "Sample timing phase offset 180"]
    #[inline(always)]
    pub fn o180(self) -> &'a mut crate::W<REG> {
        self.variant(DAT_SAMPLE_TIMING_PHASE_A::O180)
    }
    #[doc = "Sample timing phase offset 270"]
    #[inline(always)]
    pub fn o270(self) -> &'a mut crate::W<REG> {
        self.variant(DAT_SAMPLE_TIMING_PHASE_A::O270)
    }
    #[doc = "Sample timing phase offset 0 (only for SD2 hs400 mode)"]
    #[inline(always)]
    pub fn o0(self) -> &'a mut crate::W<REG> {
        self.variant(DAT_SAMPLE_TIMING_PHASE_A::O0)
    }
}
#[doc = "Field `cmd_send_rx_phase_clr` reader - Clear command rx phase before sending the command"]
pub type CMD_SEND_RX_PHASE_CLR_R = crate::BitReader<CMD_SEND_RX_PHASE_CLR_A>;
#[doc = "Clear command rx phase before sending the command\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMD_SEND_RX_PHASE_CLR_A {
    #[doc = "0: Disabled"]
    DISABLED = 0,
    #[doc = "1: Enabled"]
    ENABLED = 1,
}
impl From<CMD_SEND_RX_PHASE_CLR_A> for bool {
    #[inline(always)]
    fn from(variant: CMD_SEND_RX_PHASE_CLR_A) -> Self {
        variant as u8 != 0
    }
}
impl CMD_SEND_RX_PHASE_CLR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CMD_SEND_RX_PHASE_CLR_A {
        match self.bits {
            false => CMD_SEND_RX_PHASE_CLR_A::DISABLED,
            true => CMD_SEND_RX_PHASE_CLR_A::ENABLED,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CMD_SEND_RX_PHASE_CLR_A::DISABLED
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CMD_SEND_RX_PHASE_CLR_A::ENABLED
    }
}
#[doc = "Field `cmd_send_rx_phase_clr` writer - Clear command rx phase before sending the command"]
pub type CMD_SEND_RX_PHASE_CLR_W<'a, REG> = crate::BitWriter<'a, REG, CMD_SEND_RX_PHASE_CLR_A>;
impl<'a, REG> CMD_SEND_RX_PHASE_CLR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(CMD_SEND_RX_PHASE_CLR_A::DISABLED)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(CMD_SEND_RX_PHASE_CLR_A::ENABLED)
    }
}
#[doc = "Field `dat_recv_rx_phase_clr` reader - Clear the input phase of data lines before receiving the data"]
pub type DAT_RECV_RX_PHASE_CLR_R = crate::BitReader<DAT_RECV_RX_PHASE_CLR_A>;
#[doc = "Clear the input phase of data lines before receiving the data\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DAT_RECV_RX_PHASE_CLR_A {
    #[doc = "0: Disabled"]
    DISABLED = 0,
    #[doc = "1: Enabled"]
    ENABLED = 1,
}
impl From<DAT_RECV_RX_PHASE_CLR_A> for bool {
    #[inline(always)]
    fn from(variant: DAT_RECV_RX_PHASE_CLR_A) -> Self {
        variant as u8 != 0
    }
}
impl DAT_RECV_RX_PHASE_CLR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DAT_RECV_RX_PHASE_CLR_A {
        match self.bits {
            false => DAT_RECV_RX_PHASE_CLR_A::DISABLED,
            true => DAT_RECV_RX_PHASE_CLR_A::ENABLED,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DAT_RECV_RX_PHASE_CLR_A::DISABLED
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DAT_RECV_RX_PHASE_CLR_A::ENABLED
    }
}
#[doc = "Field `dat_recv_rx_phase_clr` writer - Clear the input phase of data lines before receiving the data"]
pub type DAT_RECV_RX_PHASE_CLR_W<'a, REG> = crate::BitWriter<'a, REG, DAT_RECV_RX_PHASE_CLR_A>;
impl<'a, REG> DAT_RECV_RX_PHASE_CLR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(DAT_RECV_RX_PHASE_CLR_A::DISABLED)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(DAT_RECV_RX_PHASE_CLR_A::ENABLED)
    }
}
#[doc = "Field `dat_trans_rx_phase_clr` reader - Clear the input phase of data lines before transferring the data"]
pub type DAT_TRANS_RX_PHASE_CLR_R = crate::BitReader<DAT_TRANS_RX_PHASE_CLR_A>;
#[doc = "Clear the input phase of data lines before transferring the data\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DAT_TRANS_RX_PHASE_CLR_A {
    #[doc = "0: Disabled"]
    DISABLED = 0,
    #[doc = "1: Enabled"]
    ENABLED = 1,
}
impl From<DAT_TRANS_RX_PHASE_CLR_A> for bool {
    #[inline(always)]
    fn from(variant: DAT_TRANS_RX_PHASE_CLR_A) -> Self {
        variant as u8 != 0
    }
}
impl DAT_TRANS_RX_PHASE_CLR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DAT_TRANS_RX_PHASE_CLR_A {
        match self.bits {
            false => DAT_TRANS_RX_PHASE_CLR_A::DISABLED,
            true => DAT_TRANS_RX_PHASE_CLR_A::ENABLED,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DAT_TRANS_RX_PHASE_CLR_A::DISABLED
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DAT_TRANS_RX_PHASE_CLR_A::ENABLED
    }
}
#[doc = "Field `dat_trans_rx_phase_clr` writer - Clear the input phase of data lines before transferring the data"]
pub type DAT_TRANS_RX_PHASE_CLR_W<'a, REG> = crate::BitWriter<'a, REG, DAT_TRANS_RX_PHASE_CLR_A>;
impl<'a, REG> DAT_TRANS_RX_PHASE_CLR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(DAT_TRANS_RX_PHASE_CLR_A::DISABLED)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(DAT_TRANS_RX_PHASE_CLR_A::ENABLED)
    }
}
#[doc = "Field `dat_crc_status_rx_phase_clr` reader - Clear the input phase of data lines before receiving the CRC status"]
pub type DAT_CRC_STATUS_RX_PHASE_CLR_R = crate::BitReader<DAT_CRC_STATUS_RX_PHASE_CLR_A>;
#[doc = "Clear the input phase of data lines before receiving the CRC status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DAT_CRC_STATUS_RX_PHASE_CLR_A {
    #[doc = "0: Disabled"]
    DISABLED = 0,
    #[doc = "1: Enabled"]
    ENABLED = 1,
}
impl From<DAT_CRC_STATUS_RX_PHASE_CLR_A> for bool {
    #[inline(always)]
    fn from(variant: DAT_CRC_STATUS_RX_PHASE_CLR_A) -> Self {
        variant as u8 != 0
    }
}
impl DAT_CRC_STATUS_RX_PHASE_CLR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DAT_CRC_STATUS_RX_PHASE_CLR_A {
        match self.bits {
            false => DAT_CRC_STATUS_RX_PHASE_CLR_A::DISABLED,
            true => DAT_CRC_STATUS_RX_PHASE_CLR_A::ENABLED,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DAT_CRC_STATUS_RX_PHASE_CLR_A::DISABLED
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DAT_CRC_STATUS_RX_PHASE_CLR_A::ENABLED
    }
}
#[doc = "Field `dat_crc_status_rx_phase_clr` writer - Clear the input phase of data lines before receiving the CRC status"]
pub type DAT_CRC_STATUS_RX_PHASE_CLR_W<'a, REG> =
    crate::BitWriter<'a, REG, DAT_CRC_STATUS_RX_PHASE_CLR_A>;
impl<'a, REG> DAT_CRC_STATUS_RX_PHASE_CLR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(DAT_CRC_STATUS_RX_PHASE_CLR_A::DISABLED)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(DAT_CRC_STATUS_RX_PHASE_CLR_A::ENABLED)
    }
}
#[doc = "Field `cmd_dat_rx_phase_clr` reader - Clear the input phase of command lines and data lines during the update clock operation"]
pub type CMD_DAT_RX_PHASE_CLR_R = crate::BitReader<CMD_DAT_RX_PHASE_CLR_A>;
#[doc = "Clear the input phase of command lines and data lines during the update clock operation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMD_DAT_RX_PHASE_CLR_A {
    #[doc = "0: Disabled"]
    DISABLED = 0,
    #[doc = "1: Enabled"]
    ENABLED = 1,
}
impl From<CMD_DAT_RX_PHASE_CLR_A> for bool {
    #[inline(always)]
    fn from(variant: CMD_DAT_RX_PHASE_CLR_A) -> Self {
        variant as u8 != 0
    }
}
impl CMD_DAT_RX_PHASE_CLR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CMD_DAT_RX_PHASE_CLR_A {
        match self.bits {
            false => CMD_DAT_RX_PHASE_CLR_A::DISABLED,
            true => CMD_DAT_RX_PHASE_CLR_A::ENABLED,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CMD_DAT_RX_PHASE_CLR_A::DISABLED
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CMD_DAT_RX_PHASE_CLR_A::ENABLED
    }
}
#[doc = "Field `cmd_dat_rx_phase_clr` writer - Clear the input phase of command lines and data lines during the update clock operation"]
pub type CMD_DAT_RX_PHASE_CLR_W<'a, REG> = crate::BitWriter<'a, REG, CMD_DAT_RX_PHASE_CLR_A>;
impl<'a, REG> CMD_DAT_RX_PHASE_CLR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(CMD_DAT_RX_PHASE_CLR_A::DISABLED)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(CMD_DAT_RX_PHASE_CLR_A::ENABLED)
    }
}
#[doc = "Field `mode_select` reader - "]
pub type MODE_SELECT_R = crate::BitReader<MODE_SELECT_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MODE_SELECT_A {
    #[doc = "0: Old mode of Sample/Output Timing"]
    OLD_MODE = 0,
    #[doc = "1: New mode of Sample/Output Timing"]
    NEW_MODE = 1,
}
impl From<MODE_SELECT_A> for bool {
    #[inline(always)]
    fn from(variant: MODE_SELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl MODE_SELECT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MODE_SELECT_A {
        match self.bits {
            false => MODE_SELECT_A::OLD_MODE,
            true => MODE_SELECT_A::NEW_MODE,
        }
    }
    #[doc = "Old mode of Sample/Output Timing"]
    #[inline(always)]
    pub fn is_old_mode(&self) -> bool {
        *self == MODE_SELECT_A::OLD_MODE
    }
    #[doc = "New mode of Sample/Output Timing"]
    #[inline(always)]
    pub fn is_new_mode(&self) -> bool {
        *self == MODE_SELECT_A::NEW_MODE
    }
}
#[doc = "Field `mode_select` writer - "]
pub type MODE_SELECT_W<'a, REG> = crate::BitWriter<'a, REG, MODE_SELECT_A>;
impl<'a, REG> MODE_SELECT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Old mode of Sample/Output Timing"]
    #[inline(always)]
    pub fn old_mode(self) -> &'a mut crate::W<REG> {
        self.variant(MODE_SELECT_A::OLD_MODE)
    }
    #[doc = "New mode of Sample/Output Timing"]
    #[inline(always)]
    pub fn new_mode(self) -> &'a mut crate::W<REG> {
        self.variant(MODE_SELECT_A::NEW_MODE)
    }
}
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn hs400_new_sample_en(&self) -> HS400_NEW_SAMPLE_EN_R {
        HS400_NEW_SAMPLE_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn cmd_sample_timing_phase(&self) -> CMD_SAMPLE_TIMING_PHASE_R {
        CMD_SAMPLE_TIMING_PHASE_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn dat_sample_timing_phase(&self) -> DAT_SAMPLE_TIMING_PHASE_R {
        DAT_SAMPLE_TIMING_PHASE_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 16 - Clear command rx phase before sending the command"]
    #[inline(always)]
    pub fn cmd_send_rx_phase_clr(&self) -> CMD_SEND_RX_PHASE_CLR_R {
        CMD_SEND_RX_PHASE_CLR_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 20 - Clear the input phase of data lines before receiving the data"]
    #[inline(always)]
    pub fn dat_recv_rx_phase_clr(&self) -> DAT_RECV_RX_PHASE_CLR_R {
        DAT_RECV_RX_PHASE_CLR_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Clear the input phase of data lines before transferring the data"]
    #[inline(always)]
    pub fn dat_trans_rx_phase_clr(&self) -> DAT_TRANS_RX_PHASE_CLR_R {
        DAT_TRANS_RX_PHASE_CLR_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Clear the input phase of data lines before receiving the CRC status"]
    #[inline(always)]
    pub fn dat_crc_status_rx_phase_clr(&self) -> DAT_CRC_STATUS_RX_PHASE_CLR_R {
        DAT_CRC_STATUS_RX_PHASE_CLR_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 24 - Clear the input phase of command lines and data lines during the update clock operation"]
    #[inline(always)]
    pub fn cmd_dat_rx_phase_clr(&self) -> CMD_DAT_RX_PHASE_CLR_R {
        CMD_DAT_RX_PHASE_CLR_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn mode_select(&self) -> MODE_SELECT_R {
        MODE_SELECT_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn hs400_new_sample_en(&mut self) -> HS400_NEW_SAMPLE_EN_W<SMHC_NTSR_SPEC> {
        HS400_NEW_SAMPLE_EN_W::new(self, 0)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    #[must_use]
    pub fn cmd_sample_timing_phase(&mut self) -> CMD_SAMPLE_TIMING_PHASE_W<SMHC_NTSR_SPEC> {
        CMD_SAMPLE_TIMING_PHASE_W::new(self, 4)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    #[must_use]
    pub fn dat_sample_timing_phase(&mut self) -> DAT_SAMPLE_TIMING_PHASE_W<SMHC_NTSR_SPEC> {
        DAT_SAMPLE_TIMING_PHASE_W::new(self, 8)
    }
    #[doc = "Bit 16 - Clear command rx phase before sending the command"]
    #[inline(always)]
    #[must_use]
    pub fn cmd_send_rx_phase_clr(&mut self) -> CMD_SEND_RX_PHASE_CLR_W<SMHC_NTSR_SPEC> {
        CMD_SEND_RX_PHASE_CLR_W::new(self, 16)
    }
    #[doc = "Bit 20 - Clear the input phase of data lines before receiving the data"]
    #[inline(always)]
    #[must_use]
    pub fn dat_recv_rx_phase_clr(&mut self) -> DAT_RECV_RX_PHASE_CLR_W<SMHC_NTSR_SPEC> {
        DAT_RECV_RX_PHASE_CLR_W::new(self, 20)
    }
    #[doc = "Bit 21 - Clear the input phase of data lines before transferring the data"]
    #[inline(always)]
    #[must_use]
    pub fn dat_trans_rx_phase_clr(&mut self) -> DAT_TRANS_RX_PHASE_CLR_W<SMHC_NTSR_SPEC> {
        DAT_TRANS_RX_PHASE_CLR_W::new(self, 21)
    }
    #[doc = "Bit 22 - Clear the input phase of data lines before receiving the CRC status"]
    #[inline(always)]
    #[must_use]
    pub fn dat_crc_status_rx_phase_clr(&mut self) -> DAT_CRC_STATUS_RX_PHASE_CLR_W<SMHC_NTSR_SPEC> {
        DAT_CRC_STATUS_RX_PHASE_CLR_W::new(self, 22)
    }
    #[doc = "Bit 24 - Clear the input phase of command lines and data lines during the update clock operation"]
    #[inline(always)]
    #[must_use]
    pub fn cmd_dat_rx_phase_clr(&mut self) -> CMD_DAT_RX_PHASE_CLR_W<SMHC_NTSR_SPEC> {
        CMD_DAT_RX_PHASE_CLR_W::new(self, 24)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    #[must_use]
    pub fn mode_select(&mut self) -> MODE_SELECT_W<SMHC_NTSR_SPEC> {
        MODE_SELECT_W::new(self, 31)
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
#[doc = "SD New Timing Set Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`smhc_ntsr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`smhc_ntsr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SMHC_NTSR_SPEC;
impl crate::RegisterSpec for SMHC_NTSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`smhc_ntsr::R`](R) reader structure"]
impl crate::Readable for SMHC_NTSR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`smhc_ntsr::W`](W) writer structure"]
impl crate::Writable for SMHC_NTSR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets smhc_ntsr to value 0"]
impl crate::Resettable for SMHC_NTSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
