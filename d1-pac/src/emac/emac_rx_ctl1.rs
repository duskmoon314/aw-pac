#[doc = "Register `emac_rx_ctl1` reader"]
pub struct R(crate::R<EMAC_RX_CTL1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EMAC_RX_CTL1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EMAC_RX_CTL1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EMAC_RX_CTL1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `emac_rx_ctl1` writer"]
pub struct W(crate::W<EMAC_RX_CTL1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EMAC_RX_CTL1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<EMAC_RX_CTL1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EMAC_RX_CTL1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `flush_rx_frm` reader - Flush Receive Frames"]
pub type FLUSH_RX_FRM_R = crate::BitReader<FLUSH_RX_FRM_A>;
#[doc = "Flush Receive Frames\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FLUSH_RX_FRM_A {
    #[doc = "0: `0`"]
    ENABLE = 0,
    #[doc = "1: `1`"]
    DISABLE = 1,
}
impl From<FLUSH_RX_FRM_A> for bool {
    #[inline(always)]
    fn from(variant: FLUSH_RX_FRM_A) -> Self {
        variant as u8 != 0
    }
}
impl FLUSH_RX_FRM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLUSH_RX_FRM_A {
        match self.bits {
            false => FLUSH_RX_FRM_A::ENABLE,
            true => FLUSH_RX_FRM_A::DISABLE,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == FLUSH_RX_FRM_A::ENABLE
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == FLUSH_RX_FRM_A::DISABLE
    }
}
#[doc = "Field `flush_rx_frm` writer - Flush Receive Frames"]
pub type FLUSH_RX_FRM_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, EMAC_RX_CTL1_SPEC, FLUSH_RX_FRM_A, O>;
impl<'a, const O: u8> FLUSH_RX_FRM_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(FLUSH_RX_FRM_A::ENABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(FLUSH_RX_FRM_A::DISABLE)
    }
}
#[doc = "Field `rx_md` reader - Receive Mode"]
pub type RX_MD_R = crate::BitReader<RX_MD_A>;
#[doc = "Receive Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RX_MD_A {
    #[doc = "0: `0`"]
    GREATER_THAN_TH = 0,
    #[doc = "1: `1`"]
    LOCATE_FULL_FRAME = 1,
}
impl From<RX_MD_A> for bool {
    #[inline(always)]
    fn from(variant: RX_MD_A) -> Self {
        variant as u8 != 0
    }
}
impl RX_MD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RX_MD_A {
        match self.bits {
            false => RX_MD_A::GREATER_THAN_TH,
            true => RX_MD_A::LOCATE_FULL_FRAME,
        }
    }
    #[doc = "Checks if the value of the field is `GREATER_THAN_TH`"]
    #[inline(always)]
    pub fn is_greater_than_th(&self) -> bool {
        *self == RX_MD_A::GREATER_THAN_TH
    }
    #[doc = "Checks if the value of the field is `LOCATE_FULL_FRAME`"]
    #[inline(always)]
    pub fn is_locate_full_frame(&self) -> bool {
        *self == RX_MD_A::LOCATE_FULL_FRAME
    }
}
#[doc = "Field `rx_md` writer - Receive Mode"]
pub type RX_MD_W<'a, const O: u8> = crate::BitWriter<'a, u32, EMAC_RX_CTL1_SPEC, RX_MD_A, O>;
impl<'a, const O: u8> RX_MD_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn greater_than_th(self) -> &'a mut W {
        self.variant(RX_MD_A::GREATER_THAN_TH)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn locate_full_frame(self) -> &'a mut W {
        self.variant(RX_MD_A::LOCATE_FULL_FRAME)
    }
}
#[doc = "Field `rx_runt_frm` reader - "]
pub type RX_RUNT_FRM_R = crate::BitReader<bool>;
#[doc = "Field `rx_runt_frm` writer - "]
pub type RX_RUNT_FRM_W<'a, const O: u8> = crate::BitWriter<'a, u32, EMAC_RX_CTL1_SPEC, bool, O>;
#[doc = "Field `rx_err_frm` reader - "]
pub type RX_ERR_FRM_R = crate::BitReader<RX_ERR_FRM_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RX_ERR_FRM_A {
    #[doc = "0: `0`"]
    DROP = 0,
    #[doc = "1: `1`"]
    FORWARD = 1,
}
impl From<RX_ERR_FRM_A> for bool {
    #[inline(always)]
    fn from(variant: RX_ERR_FRM_A) -> Self {
        variant as u8 != 0
    }
}
impl RX_ERR_FRM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RX_ERR_FRM_A {
        match self.bits {
            false => RX_ERR_FRM_A::DROP,
            true => RX_ERR_FRM_A::FORWARD,
        }
    }
    #[doc = "Checks if the value of the field is `DROP`"]
    #[inline(always)]
    pub fn is_drop(&self) -> bool {
        *self == RX_ERR_FRM_A::DROP
    }
    #[doc = "Checks if the value of the field is `FORWARD`"]
    #[inline(always)]
    pub fn is_forward(&self) -> bool {
        *self == RX_ERR_FRM_A::FORWARD
    }
}
#[doc = "Field `rx_err_frm` writer - "]
pub type RX_ERR_FRM_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, EMAC_RX_CTL1_SPEC, RX_ERR_FRM_A, O>;
impl<'a, const O: u8> RX_ERR_FRM_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn drop(self) -> &'a mut W {
        self.variant(RX_ERR_FRM_A::DROP)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn forward(self) -> &'a mut W {
        self.variant(RX_ERR_FRM_A::FORWARD)
    }
}
#[doc = "Field `rx_th` reader - Threshold for RX DMA FIFO Start"]
pub type RX_TH_R = crate::FieldReader<u8, RX_TH_A>;
#[doc = "Threshold for RX DMA FIFO Start\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RX_TH_A {
    #[doc = "0: `0`"]
    T64 = 0,
    #[doc = "1: `1`"]
    T32 = 1,
    #[doc = "2: `10`"]
    T96 = 2,
    #[doc = "3: `11`"]
    T128 = 3,
}
impl From<RX_TH_A> for u8 {
    #[inline(always)]
    fn from(variant: RX_TH_A) -> Self {
        variant as _
    }
}
impl RX_TH_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RX_TH_A {
        match self.bits {
            0 => RX_TH_A::T64,
            1 => RX_TH_A::T32,
            2 => RX_TH_A::T96,
            3 => RX_TH_A::T128,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `T64`"]
    #[inline(always)]
    pub fn is_t64(&self) -> bool {
        *self == RX_TH_A::T64
    }
    #[doc = "Checks if the value of the field is `T32`"]
    #[inline(always)]
    pub fn is_t32(&self) -> bool {
        *self == RX_TH_A::T32
    }
    #[doc = "Checks if the value of the field is `T96`"]
    #[inline(always)]
    pub fn is_t96(&self) -> bool {
        *self == RX_TH_A::T96
    }
    #[doc = "Checks if the value of the field is `T128`"]
    #[inline(always)]
    pub fn is_t128(&self) -> bool {
        *self == RX_TH_A::T128
    }
}
#[doc = "Field `rx_th` writer - Threshold for RX DMA FIFO Start"]
pub type RX_TH_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, EMAC_RX_CTL1_SPEC, u8, RX_TH_A, 2, O>;
impl<'a, const O: u8> RX_TH_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn t64(self) -> &'a mut W {
        self.variant(RX_TH_A::T64)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn t32(self) -> &'a mut W {
        self.variant(RX_TH_A::T32)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn t96(self) -> &'a mut W {
        self.variant(RX_TH_A::T96)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn t128(self) -> &'a mut W {
        self.variant(RX_TH_A::T128)
    }
}
#[doc = "Field `rx_flow_ctl_th_act` reader - Threshold for Activating Flow Control"]
pub type RX_FLOW_CTL_TH_ACT_R = crate::FieldReader<u8, RX_FLOW_CTL_TH_ACT_A>;
#[doc = "Threshold for Activating Flow Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RX_FLOW_CTL_TH_ACT_A {
    #[doc = "0: `0`"]
    FM1K = 0,
    #[doc = "1: `1`"]
    FM2K = 1,
    #[doc = "2: `10`"]
    FM3K = 2,
    #[doc = "3: `11`"]
    FM4K = 3,
}
impl From<RX_FLOW_CTL_TH_ACT_A> for u8 {
    #[inline(always)]
    fn from(variant: RX_FLOW_CTL_TH_ACT_A) -> Self {
        variant as _
    }
}
impl RX_FLOW_CTL_TH_ACT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RX_FLOW_CTL_TH_ACT_A {
        match self.bits {
            0 => RX_FLOW_CTL_TH_ACT_A::FM1K,
            1 => RX_FLOW_CTL_TH_ACT_A::FM2K,
            2 => RX_FLOW_CTL_TH_ACT_A::FM3K,
            3 => RX_FLOW_CTL_TH_ACT_A::FM4K,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `FM1K`"]
    #[inline(always)]
    pub fn is_fm1k(&self) -> bool {
        *self == RX_FLOW_CTL_TH_ACT_A::FM1K
    }
    #[doc = "Checks if the value of the field is `FM2K`"]
    #[inline(always)]
    pub fn is_fm2k(&self) -> bool {
        *self == RX_FLOW_CTL_TH_ACT_A::FM2K
    }
    #[doc = "Checks if the value of the field is `FM3K`"]
    #[inline(always)]
    pub fn is_fm3k(&self) -> bool {
        *self == RX_FLOW_CTL_TH_ACT_A::FM3K
    }
    #[doc = "Checks if the value of the field is `FM4K`"]
    #[inline(always)]
    pub fn is_fm4k(&self) -> bool {
        *self == RX_FLOW_CTL_TH_ACT_A::FM4K
    }
}
#[doc = "Field `rx_flow_ctl_th_act` writer - Threshold for Activating Flow Control"]
pub type RX_FLOW_CTL_TH_ACT_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, EMAC_RX_CTL1_SPEC, u8, RX_FLOW_CTL_TH_ACT_A, 2, O>;
impl<'a, const O: u8> RX_FLOW_CTL_TH_ACT_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn fm1k(self) -> &'a mut W {
        self.variant(RX_FLOW_CTL_TH_ACT_A::FM1K)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn fm2k(self) -> &'a mut W {
        self.variant(RX_FLOW_CTL_TH_ACT_A::FM2K)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn fm3k(self) -> &'a mut W {
        self.variant(RX_FLOW_CTL_TH_ACT_A::FM3K)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn fm4k(self) -> &'a mut W {
        self.variant(RX_FLOW_CTL_TH_ACT_A::FM4K)
    }
}
#[doc = "Field `rx_flow_ctl_th_deact` reader - Threshold for Deactivating Flow Control"]
pub type RX_FLOW_CTL_TH_DEACT_R = crate::FieldReader<u8, RX_FLOW_CTL_TH_DEACT_A>;
#[doc = "Threshold for Deactivating Flow Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RX_FLOW_CTL_TH_DEACT_A {
    #[doc = "0: `0`"]
    FM1K = 0,
    #[doc = "1: `1`"]
    FM2K = 1,
    #[doc = "2: `10`"]
    FM3K = 2,
    #[doc = "3: `11`"]
    FM4K = 3,
}
impl From<RX_FLOW_CTL_TH_DEACT_A> for u8 {
    #[inline(always)]
    fn from(variant: RX_FLOW_CTL_TH_DEACT_A) -> Self {
        variant as _
    }
}
impl RX_FLOW_CTL_TH_DEACT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RX_FLOW_CTL_TH_DEACT_A {
        match self.bits {
            0 => RX_FLOW_CTL_TH_DEACT_A::FM1K,
            1 => RX_FLOW_CTL_TH_DEACT_A::FM2K,
            2 => RX_FLOW_CTL_TH_DEACT_A::FM3K,
            3 => RX_FLOW_CTL_TH_DEACT_A::FM4K,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `FM1K`"]
    #[inline(always)]
    pub fn is_fm1k(&self) -> bool {
        *self == RX_FLOW_CTL_TH_DEACT_A::FM1K
    }
    #[doc = "Checks if the value of the field is `FM2K`"]
    #[inline(always)]
    pub fn is_fm2k(&self) -> bool {
        *self == RX_FLOW_CTL_TH_DEACT_A::FM2K
    }
    #[doc = "Checks if the value of the field is `FM3K`"]
    #[inline(always)]
    pub fn is_fm3k(&self) -> bool {
        *self == RX_FLOW_CTL_TH_DEACT_A::FM3K
    }
    #[doc = "Checks if the value of the field is `FM4K`"]
    #[inline(always)]
    pub fn is_fm4k(&self) -> bool {
        *self == RX_FLOW_CTL_TH_DEACT_A::FM4K
    }
}
#[doc = "Field `rx_flow_ctl_th_deact` writer - Threshold for Deactivating Flow Control"]
pub type RX_FLOW_CTL_TH_DEACT_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, EMAC_RX_CTL1_SPEC, u8, RX_FLOW_CTL_TH_DEACT_A, 2, O>;
impl<'a, const O: u8> RX_FLOW_CTL_TH_DEACT_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn fm1k(self) -> &'a mut W {
        self.variant(RX_FLOW_CTL_TH_DEACT_A::FM1K)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn fm2k(self) -> &'a mut W {
        self.variant(RX_FLOW_CTL_TH_DEACT_A::FM2K)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn fm3k(self) -> &'a mut W {
        self.variant(RX_FLOW_CTL_TH_DEACT_A::FM3K)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn fm4k(self) -> &'a mut W {
        self.variant(RX_FLOW_CTL_TH_DEACT_A::FM4K)
    }
}
#[doc = "Field `rx_fifo_flow_ctl` reader - Receive FIFO Flow Control Enable"]
pub type RX_FIFO_FLOW_CTL_R = crate::BitReader<RX_FIFO_FLOW_CTL_A>;
#[doc = "Receive FIFO Flow Control Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RX_FIFO_FLOW_CTL_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<RX_FIFO_FLOW_CTL_A> for bool {
    #[inline(always)]
    fn from(variant: RX_FIFO_FLOW_CTL_A) -> Self {
        variant as u8 != 0
    }
}
impl RX_FIFO_FLOW_CTL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RX_FIFO_FLOW_CTL_A {
        match self.bits {
            false => RX_FIFO_FLOW_CTL_A::DISABLE,
            true => RX_FIFO_FLOW_CTL_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == RX_FIFO_FLOW_CTL_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == RX_FIFO_FLOW_CTL_A::ENABLE
    }
}
#[doc = "Field `rx_fifo_flow_ctl` writer - Receive FIFO Flow Control Enable"]
pub type RX_FIFO_FLOW_CTL_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, EMAC_RX_CTL1_SPEC, RX_FIFO_FLOW_CTL_A, O>;
impl<'a, const O: u8> RX_FIFO_FLOW_CTL_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(RX_FIFO_FLOW_CTL_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(RX_FIFO_FLOW_CTL_A::ENABLE)
    }
}
#[doc = "Field `rx_ema_en` reader - Receive DMA Enable"]
pub type RX_EMA_EN_R = crate::BitReader<RX_EMA_EN_A>;
#[doc = "Receive DMA Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RX_EMA_EN_A {
    #[doc = "0: `0`"]
    STOP = 0,
    #[doc = "1: `1`"]
    START = 1,
}
impl From<RX_EMA_EN_A> for bool {
    #[inline(always)]
    fn from(variant: RX_EMA_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl RX_EMA_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RX_EMA_EN_A {
        match self.bits {
            false => RX_EMA_EN_A::STOP,
            true => RX_EMA_EN_A::START,
        }
    }
    #[doc = "Checks if the value of the field is `STOP`"]
    #[inline(always)]
    pub fn is_stop(&self) -> bool {
        *self == RX_EMA_EN_A::STOP
    }
    #[doc = "Checks if the value of the field is `START`"]
    #[inline(always)]
    pub fn is_start(&self) -> bool {
        *self == RX_EMA_EN_A::START
    }
}
#[doc = "Field `rx_ema_en` writer - Receive DMA Enable"]
pub type RX_EMA_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, EMAC_RX_CTL1_SPEC, RX_EMA_EN_A, O>;
impl<'a, const O: u8> RX_EMA_EN_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn stop(self) -> &'a mut W {
        self.variant(RX_EMA_EN_A::STOP)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn start(self) -> &'a mut W {
        self.variant(RX_EMA_EN_A::START)
    }
}
#[doc = "Field `rx_dma_start` reader - "]
pub type RX_DMA_START_R = crate::BitReader<bool>;
#[doc = "Field `rx_dma_start` writer - "]
pub type RX_DMA_START_W<'a, const O: u8> = crate::BitWriter<'a, u32, EMAC_RX_CTL1_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Flush Receive Frames"]
    #[inline(always)]
    pub fn flush_rx_frm(&self) -> FLUSH_RX_FRM_R {
        FLUSH_RX_FRM_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Receive Mode"]
    #[inline(always)]
    pub fn rx_md(&self) -> RX_MD_R {
        RX_MD_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn rx_runt_frm(&self) -> RX_RUNT_FRM_R {
        RX_RUNT_FRM_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn rx_err_frm(&self) -> RX_ERR_FRM_R {
        RX_ERR_FRM_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - Threshold for RX DMA FIFO Start"]
    #[inline(always)]
    pub fn rx_th(&self) -> RX_TH_R {
        RX_TH_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 20:21 - Threshold for Activating Flow Control"]
    #[inline(always)]
    pub fn rx_flow_ctl_th_act(&self) -> RX_FLOW_CTL_TH_ACT_R {
        RX_FLOW_CTL_TH_ACT_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23 - Threshold for Deactivating Flow Control"]
    #[inline(always)]
    pub fn rx_flow_ctl_th_deact(&self) -> RX_FLOW_CTL_TH_DEACT_R {
        RX_FLOW_CTL_TH_DEACT_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bit 24 - Receive FIFO Flow Control Enable"]
    #[inline(always)]
    pub fn rx_fifo_flow_ctl(&self) -> RX_FIFO_FLOW_CTL_R {
        RX_FIFO_FLOW_CTL_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 30 - Receive DMA Enable"]
    #[inline(always)]
    pub fn rx_ema_en(&self) -> RX_EMA_EN_R {
        RX_EMA_EN_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn rx_dma_start(&self) -> RX_DMA_START_R {
        RX_DMA_START_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Flush Receive Frames"]
    #[inline(always)]
    #[must_use]
    pub fn flush_rx_frm(&mut self) -> FLUSH_RX_FRM_W<0> {
        FLUSH_RX_FRM_W::new(self)
    }
    #[doc = "Bit 1 - Receive Mode"]
    #[inline(always)]
    #[must_use]
    pub fn rx_md(&mut self) -> RX_MD_W<1> {
        RX_MD_W::new(self)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn rx_runt_frm(&mut self) -> RX_RUNT_FRM_W<2> {
        RX_RUNT_FRM_W::new(self)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn rx_err_frm(&mut self) -> RX_ERR_FRM_W<3> {
        RX_ERR_FRM_W::new(self)
    }
    #[doc = "Bits 4:5 - Threshold for RX DMA FIFO Start"]
    #[inline(always)]
    #[must_use]
    pub fn rx_th(&mut self) -> RX_TH_W<4> {
        RX_TH_W::new(self)
    }
    #[doc = "Bits 20:21 - Threshold for Activating Flow Control"]
    #[inline(always)]
    #[must_use]
    pub fn rx_flow_ctl_th_act(&mut self) -> RX_FLOW_CTL_TH_ACT_W<20> {
        RX_FLOW_CTL_TH_ACT_W::new(self)
    }
    #[doc = "Bits 22:23 - Threshold for Deactivating Flow Control"]
    #[inline(always)]
    #[must_use]
    pub fn rx_flow_ctl_th_deact(&mut self) -> RX_FLOW_CTL_TH_DEACT_W<22> {
        RX_FLOW_CTL_TH_DEACT_W::new(self)
    }
    #[doc = "Bit 24 - Receive FIFO Flow Control Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rx_fifo_flow_ctl(&mut self) -> RX_FIFO_FLOW_CTL_W<24> {
        RX_FIFO_FLOW_CTL_W::new(self)
    }
    #[doc = "Bit 30 - Receive DMA Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rx_ema_en(&mut self) -> RX_EMA_EN_W<30> {
        RX_EMA_EN_W::new(self)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    #[must_use]
    pub fn rx_dma_start(&mut self) -> RX_DMA_START_W<31> {
        RX_DMA_START_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "EMAC Receive Control Register1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [emac_rx_ctl1](index.html) module"]
pub struct EMAC_RX_CTL1_SPEC;
impl crate::RegisterSpec for EMAC_RX_CTL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [emac_rx_ctl1::R](R) reader structure"]
impl crate::Readable for EMAC_RX_CTL1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [emac_rx_ctl1::W](W) writer structure"]
impl crate::Writable for EMAC_RX_CTL1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets emac_rx_ctl1 to value 0"]
impl crate::Resettable for EMAC_RX_CTL1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
