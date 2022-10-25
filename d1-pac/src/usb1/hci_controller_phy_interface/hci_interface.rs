#[doc = "Register `hci_interface` reader"]
pub struct R(crate::R<HCI_INTERFACE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HCI_INTERFACE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HCI_INTERFACE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HCI_INTERFACE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `hci_interface` writer"]
pub struct W(crate::W<HCI_INTERFACE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HCI_INTERFACE_SPEC>;
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
impl From<crate::W<HCI_INTERFACE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HCI_INTERFACE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ulpi_bypass_enable` reader - ULPI bypass enable"]
pub type ULPI_BYPASS_ENABLE_R = crate::BitReader<ULPI_BYPASS_ENABLE_A>;
#[doc = "ULPI bypass enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ULPI_BYPASS_ENABLE_A {
    #[doc = "1: Enable UTMI interface, disable ULPI interface"]
    UTMI = 1,
    #[doc = "0: Enable ULPI interface, disable UTMI interface"]
    ULPI = 0,
}
impl From<ULPI_BYPASS_ENABLE_A> for bool {
    #[inline(always)]
    fn from(variant: ULPI_BYPASS_ENABLE_A) -> Self {
        variant as u8 != 0
    }
}
impl ULPI_BYPASS_ENABLE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ULPI_BYPASS_ENABLE_A {
        match self.bits {
            true => ULPI_BYPASS_ENABLE_A::UTMI,
            false => ULPI_BYPASS_ENABLE_A::ULPI,
        }
    }
    #[doc = "Checks if the value of the field is `UTMI`"]
    #[inline(always)]
    pub fn is_utmi(&self) -> bool {
        *self == ULPI_BYPASS_ENABLE_A::UTMI
    }
    #[doc = "Checks if the value of the field is `ULPI`"]
    #[inline(always)]
    pub fn is_ulpi(&self) -> bool {
        *self == ULPI_BYPASS_ENABLE_A::ULPI
    }
}
#[doc = "Field `ulpi_bypass_enable` writer - ULPI bypass enable"]
pub type ULPI_BYPASS_ENABLE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, HCI_INTERFACE_SPEC, ULPI_BYPASS_ENABLE_A, O>;
impl<'a, const O: u8> ULPI_BYPASS_ENABLE_W<'a, O> {
    #[doc = "Enable UTMI interface, disable ULPI interface"]
    #[inline(always)]
    pub fn utmi(self) -> &'a mut W {
        self.variant(ULPI_BYPASS_ENABLE_A::UTMI)
    }
    #[doc = "Enable ULPI interface, disable UTMI interface"]
    #[inline(always)]
    pub fn ulpi(self) -> &'a mut W {
        self.variant(ULPI_BYPASS_ENABLE_A::ULPI)
    }
}
#[doc = "Field `ahb_master_interface_incrx_align_enable` reader - Master interface INCRX align enable"]
pub type AHB_MASTER_INTERFACE_INCRX_ALIGN_ENABLE_R =
    crate::BitReader<AHB_MASTER_INTERFACE_INCRX_ALIGN_ENABLE_A>;
#[doc = "Master interface INCRX align enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AHB_MASTER_INTERFACE_INCRX_ALIGN_ENABLE_A {
    #[doc = "1: Start INCRx burst only on burst x-align address"]
    BURST_X_ALIGN_ADDRESS = 1,
    #[doc = "0: Start burst on any double word boundary Note: This bit must enable if any bit of bit\\[11:9\\] is enabled"]
    ANY_DOUBLE_WORD_BOUNDARY = 0,
}
impl From<AHB_MASTER_INTERFACE_INCRX_ALIGN_ENABLE_A> for bool {
    #[inline(always)]
    fn from(variant: AHB_MASTER_INTERFACE_INCRX_ALIGN_ENABLE_A) -> Self {
        variant as u8 != 0
    }
}
impl AHB_MASTER_INTERFACE_INCRX_ALIGN_ENABLE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AHB_MASTER_INTERFACE_INCRX_ALIGN_ENABLE_A {
        match self.bits {
            true => AHB_MASTER_INTERFACE_INCRX_ALIGN_ENABLE_A::BURST_X_ALIGN_ADDRESS,
            false => AHB_MASTER_INTERFACE_INCRX_ALIGN_ENABLE_A::ANY_DOUBLE_WORD_BOUNDARY,
        }
    }
    #[doc = "Checks if the value of the field is `BURST_X_ALIGN_ADDRESS`"]
    #[inline(always)]
    pub fn is_burst_x_align_address(&self) -> bool {
        *self == AHB_MASTER_INTERFACE_INCRX_ALIGN_ENABLE_A::BURST_X_ALIGN_ADDRESS
    }
    #[doc = "Checks if the value of the field is `ANY_DOUBLE_WORD_BOUNDARY`"]
    #[inline(always)]
    pub fn is_any_double_word_boundary(&self) -> bool {
        *self == AHB_MASTER_INTERFACE_INCRX_ALIGN_ENABLE_A::ANY_DOUBLE_WORD_BOUNDARY
    }
}
#[doc = "Field `ahb_master_interface_incrx_align_enable` writer - Master interface INCRX align enable"]
pub type AHB_MASTER_INTERFACE_INCRX_ALIGN_ENABLE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, HCI_INTERFACE_SPEC, AHB_MASTER_INTERFACE_INCRX_ALIGN_ENABLE_A, O>;
impl<'a, const O: u8> AHB_MASTER_INTERFACE_INCRX_ALIGN_ENABLE_W<'a, O> {
    #[doc = "Start INCRx burst only on burst x-align address"]
    #[inline(always)]
    pub fn burst_x_align_address(self) -> &'a mut W {
        self.variant(AHB_MASTER_INTERFACE_INCRX_ALIGN_ENABLE_A::BURST_X_ALIGN_ADDRESS)
    }
    #[doc = "Start burst on any double word boundary Note: This bit must enable if any bit of bit\\[11:9\\] is enabled"]
    #[inline(always)]
    pub fn any_double_word_boundary(self) -> &'a mut W {
        self.variant(AHB_MASTER_INTERFACE_INCRX_ALIGN_ENABLE_A::ANY_DOUBLE_WORD_BOUNDARY)
    }
}
#[doc = "Field `ahb_master_interface_burst_type_incr4_enable` reader - Master interface burst type INCR4 enable"]
pub type AHB_MASTER_INTERFACE_BURST_TYPE_INCR4_ENABLE_R =
    crate::BitReader<AHB_MASTER_INTERFACE_BURST_TYPE_INCR4_ENABLE_A>;
#[doc = "Master interface burst type INCR4 enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AHB_MASTER_INTERFACE_BURST_TYPE_INCR4_ENABLE_A {
    #[doc = "1: Use INCR4 when appropriate"]
    USE_INCR4 = 1,
    #[doc = "0: Do not use INCR4, use other enabled INCRX or unspecified length burst INCR"]
    NOT_USE_INCR4 = 0,
}
impl From<AHB_MASTER_INTERFACE_BURST_TYPE_INCR4_ENABLE_A> for bool {
    #[inline(always)]
    fn from(variant: AHB_MASTER_INTERFACE_BURST_TYPE_INCR4_ENABLE_A) -> Self {
        variant as u8 != 0
    }
}
impl AHB_MASTER_INTERFACE_BURST_TYPE_INCR4_ENABLE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AHB_MASTER_INTERFACE_BURST_TYPE_INCR4_ENABLE_A {
        match self.bits {
            true => AHB_MASTER_INTERFACE_BURST_TYPE_INCR4_ENABLE_A::USE_INCR4,
            false => AHB_MASTER_INTERFACE_BURST_TYPE_INCR4_ENABLE_A::NOT_USE_INCR4,
        }
    }
    #[doc = "Checks if the value of the field is `USE_INCR4`"]
    #[inline(always)]
    pub fn is_use_incr4(&self) -> bool {
        *self == AHB_MASTER_INTERFACE_BURST_TYPE_INCR4_ENABLE_A::USE_INCR4
    }
    #[doc = "Checks if the value of the field is `NOT_USE_INCR4`"]
    #[inline(always)]
    pub fn is_not_use_incr4(&self) -> bool {
        *self == AHB_MASTER_INTERFACE_BURST_TYPE_INCR4_ENABLE_A::NOT_USE_INCR4
    }
}
#[doc = "Field `ahb_master_interface_burst_type_incr4_enable` writer - Master interface burst type INCR4 enable"]
pub type AHB_MASTER_INTERFACE_BURST_TYPE_INCR4_ENABLE_W<'a, const O: u8> = crate::BitWriter<
    'a,
    u32,
    HCI_INTERFACE_SPEC,
    AHB_MASTER_INTERFACE_BURST_TYPE_INCR4_ENABLE_A,
    O,
>;
impl<'a, const O: u8> AHB_MASTER_INTERFACE_BURST_TYPE_INCR4_ENABLE_W<'a, O> {
    #[doc = "Use INCR4 when appropriate"]
    #[inline(always)]
    pub fn use_incr4(self) -> &'a mut W {
        self.variant(AHB_MASTER_INTERFACE_BURST_TYPE_INCR4_ENABLE_A::USE_INCR4)
    }
    #[doc = "Do not use INCR4, use other enabled INCRX or unspecified length burst INCR"]
    #[inline(always)]
    pub fn not_use_incr4(self) -> &'a mut W {
        self.variant(AHB_MASTER_INTERFACE_BURST_TYPE_INCR4_ENABLE_A::NOT_USE_INCR4)
    }
}
#[doc = "Field `ahb_master_interface_incr8_enable` reader - Master interface INCR8 enable"]
pub type AHB_MASTER_INTERFACE_INCR8_ENABLE_R =
    crate::BitReader<AHB_MASTER_INTERFACE_INCR8_ENABLE_A>;
#[doc = "Master interface INCR8 enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AHB_MASTER_INTERFACE_INCR8_ENABLE_A {
    #[doc = "1: Use INCR8 when appropriate"]
    USE_INCR8 = 1,
    #[doc = "0: Do not use INCR8, use other enabled INCRX or unspecified length burst INCR"]
    NOT_USE_INCR8 = 0,
}
impl From<AHB_MASTER_INTERFACE_INCR8_ENABLE_A> for bool {
    #[inline(always)]
    fn from(variant: AHB_MASTER_INTERFACE_INCR8_ENABLE_A) -> Self {
        variant as u8 != 0
    }
}
impl AHB_MASTER_INTERFACE_INCR8_ENABLE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AHB_MASTER_INTERFACE_INCR8_ENABLE_A {
        match self.bits {
            true => AHB_MASTER_INTERFACE_INCR8_ENABLE_A::USE_INCR8,
            false => AHB_MASTER_INTERFACE_INCR8_ENABLE_A::NOT_USE_INCR8,
        }
    }
    #[doc = "Checks if the value of the field is `USE_INCR8`"]
    #[inline(always)]
    pub fn is_use_incr8(&self) -> bool {
        *self == AHB_MASTER_INTERFACE_INCR8_ENABLE_A::USE_INCR8
    }
    #[doc = "Checks if the value of the field is `NOT_USE_INCR8`"]
    #[inline(always)]
    pub fn is_not_use_incr8(&self) -> bool {
        *self == AHB_MASTER_INTERFACE_INCR8_ENABLE_A::NOT_USE_INCR8
    }
}
#[doc = "Field `ahb_master_interface_incr8_enable` writer - Master interface INCR8 enable"]
pub type AHB_MASTER_INTERFACE_INCR8_ENABLE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, HCI_INTERFACE_SPEC, AHB_MASTER_INTERFACE_INCR8_ENABLE_A, O>;
impl<'a, const O: u8> AHB_MASTER_INTERFACE_INCR8_ENABLE_W<'a, O> {
    #[doc = "Use INCR8 when appropriate"]
    #[inline(always)]
    pub fn use_incr8(self) -> &'a mut W {
        self.variant(AHB_MASTER_INTERFACE_INCR8_ENABLE_A::USE_INCR8)
    }
    #[doc = "Do not use INCR8, use other enabled INCRX or unspecified length burst INCR"]
    #[inline(always)]
    pub fn not_use_incr8(self) -> &'a mut W {
        self.variant(AHB_MASTER_INTERFACE_INCR8_ENABLE_A::NOT_USE_INCR8)
    }
}
#[doc = "Field `ahb_master_interface_incr16_enable` reader - Master interface INCR16 enable"]
pub type AHB_MASTER_INTERFACE_INCR16_ENABLE_R =
    crate::BitReader<AHB_MASTER_INTERFACE_INCR16_ENABLE_A>;
#[doc = "Master interface INCR16 enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AHB_MASTER_INTERFACE_INCR16_ENABLE_A {
    #[doc = "1: Use INCR16 when appropriate"]
    USE_INCR16 = 1,
    #[doc = "0: Do not use INCR16, use other enabled INCRX or unspecified length burst INCR"]
    NOT_USE_INCR16 = 0,
}
impl From<AHB_MASTER_INTERFACE_INCR16_ENABLE_A> for bool {
    #[inline(always)]
    fn from(variant: AHB_MASTER_INTERFACE_INCR16_ENABLE_A) -> Self {
        variant as u8 != 0
    }
}
impl AHB_MASTER_INTERFACE_INCR16_ENABLE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AHB_MASTER_INTERFACE_INCR16_ENABLE_A {
        match self.bits {
            true => AHB_MASTER_INTERFACE_INCR16_ENABLE_A::USE_INCR16,
            false => AHB_MASTER_INTERFACE_INCR16_ENABLE_A::NOT_USE_INCR16,
        }
    }
    #[doc = "Checks if the value of the field is `USE_INCR16`"]
    #[inline(always)]
    pub fn is_use_incr16(&self) -> bool {
        *self == AHB_MASTER_INTERFACE_INCR16_ENABLE_A::USE_INCR16
    }
    #[doc = "Checks if the value of the field is `NOT_USE_INCR16`"]
    #[inline(always)]
    pub fn is_not_use_incr16(&self) -> bool {
        *self == AHB_MASTER_INTERFACE_INCR16_ENABLE_A::NOT_USE_INCR16
    }
}
#[doc = "Field `ahb_master_interface_incr16_enable` writer - Master interface INCR16 enable"]
pub type AHB_MASTER_INTERFACE_INCR16_ENABLE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, HCI_INTERFACE_SPEC, AHB_MASTER_INTERFACE_INCR16_ENABLE_A, O>;
impl<'a, const O: u8> AHB_MASTER_INTERFACE_INCR16_ENABLE_W<'a, O> {
    #[doc = "Use INCR16 when appropriate"]
    #[inline(always)]
    pub fn use_incr16(self) -> &'a mut W {
        self.variant(AHB_MASTER_INTERFACE_INCR16_ENABLE_A::USE_INCR16)
    }
    #[doc = "Do not use INCR16, use other enabled INCRX or unspecified length burst INCR"]
    #[inline(always)]
    pub fn not_use_incr16(self) -> &'a mut W {
        self.variant(AHB_MASTER_INTERFACE_INCR16_ENABLE_A::NOT_USE_INCR16)
    }
}
#[doc = "Field `pp2vbus` reader - "]
pub type PP2VBUS_R = crate::BitReader<PP2VBUS_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PP2VBUS_A {
    #[doc = "1: ULPI wrapper interface will automatically set or clear DrvVbus register in ULPI PHY according to the port power status form the root hub"]
    AUTO = 1,
    #[doc = "0: ULPI wrapper will ignore the difference between power status of root hub and ULPI PHY"]
    IGNORE = 0,
}
impl From<PP2VBUS_A> for bool {
    #[inline(always)]
    fn from(variant: PP2VBUS_A) -> Self {
        variant as u8 != 0
    }
}
impl PP2VBUS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PP2VBUS_A {
        match self.bits {
            true => PP2VBUS_A::AUTO,
            false => PP2VBUS_A::IGNORE,
        }
    }
    #[doc = "Checks if the value of the field is `AUTO`"]
    #[inline(always)]
    pub fn is_auto(&self) -> bool {
        *self == PP2VBUS_A::AUTO
    }
    #[doc = "Checks if the value of the field is `IGNORE`"]
    #[inline(always)]
    pub fn is_ignore(&self) -> bool {
        *self == PP2VBUS_A::IGNORE
    }
}
#[doc = "Field `pp2vbus` writer - "]
pub type PP2VBUS_W<'a, const O: u8> = crate::BitWriter<'a, u32, HCI_INTERFACE_SPEC, PP2VBUS_A, O>;
impl<'a, const O: u8> PP2VBUS_W<'a, O> {
    #[doc = "ULPI wrapper interface will automatically set or clear DrvVbus register in ULPI PHY according to the port power status form the root hub"]
    #[inline(always)]
    pub fn auto(self) -> &'a mut W {
        self.variant(PP2VBUS_A::AUTO)
    }
    #[doc = "ULPI wrapper will ignore the difference between power status of root hub and ULPI PHY"]
    #[inline(always)]
    pub fn ignore(self) -> &'a mut W {
        self.variant(PP2VBUS_A::IGNORE)
    }
}
#[doc = "Field `resume_k_to_se0_transition` reader - "]
pub type RESUME_K_TO_SE0_TRANSITION_R = crate::BitReader<RESUME_K_TO_SE0_TRANSITION_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RESUME_K_TO_SE0_TRANSITION_A {
    #[doc = "1: Within 2 us of the resume-K to SE0 transition"]
    WITHIN_2US = 1,
    #[doc = "0: Random time value of the resume-K to SE0 transition"]
    RANDOM = 0,
}
impl From<RESUME_K_TO_SE0_TRANSITION_A> for bool {
    #[inline(always)]
    fn from(variant: RESUME_K_TO_SE0_TRANSITION_A) -> Self {
        variant as u8 != 0
    }
}
impl RESUME_K_TO_SE0_TRANSITION_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RESUME_K_TO_SE0_TRANSITION_A {
        match self.bits {
            true => RESUME_K_TO_SE0_TRANSITION_A::WITHIN_2US,
            false => RESUME_K_TO_SE0_TRANSITION_A::RANDOM,
        }
    }
    #[doc = "Checks if the value of the field is `WITHIN_2US`"]
    #[inline(always)]
    pub fn is_within_2us(&self) -> bool {
        *self == RESUME_K_TO_SE0_TRANSITION_A::WITHIN_2US
    }
    #[doc = "Checks if the value of the field is `RANDOM`"]
    #[inline(always)]
    pub fn is_random(&self) -> bool {
        *self == RESUME_K_TO_SE0_TRANSITION_A::RANDOM
    }
}
#[doc = "Field `resume_k_to_se0_transition` writer - "]
pub type RESUME_K_TO_SE0_TRANSITION_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, HCI_INTERFACE_SPEC, RESUME_K_TO_SE0_TRANSITION_A, O>;
impl<'a, const O: u8> RESUME_K_TO_SE0_TRANSITION_W<'a, O> {
    #[doc = "Within 2 us of the resume-K to SE0 transition"]
    #[inline(always)]
    pub fn within_2us(self) -> &'a mut W {
        self.variant(RESUME_K_TO_SE0_TRANSITION_A::WITHIN_2US)
    }
    #[doc = "Random time value of the resume-K to SE0 transition"]
    #[inline(always)]
    pub fn random(self) -> &'a mut W {
        self.variant(RESUME_K_TO_SE0_TRANSITION_A::RANDOM)
    }
}
#[doc = "Field `ohci_count_select` reader - OHCI count select"]
pub type OHCI_COUNT_SELECT_R = crate::BitReader<OHCI_COUNT_SELECT_A>;
#[doc = "OHCI count select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OHCI_COUNT_SELECT_A {
    #[doc = "1: Simulation mode. The counters will be much shorter then real time"]
    SIMULATION = 1,
    #[doc = "0: Normal mode. The counters will count full time"]
    NORMAL = 0,
}
impl From<OHCI_COUNT_SELECT_A> for bool {
    #[inline(always)]
    fn from(variant: OHCI_COUNT_SELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl OHCI_COUNT_SELECT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OHCI_COUNT_SELECT_A {
        match self.bits {
            true => OHCI_COUNT_SELECT_A::SIMULATION,
            false => OHCI_COUNT_SELECT_A::NORMAL,
        }
    }
    #[doc = "Checks if the value of the field is `SIMULATION`"]
    #[inline(always)]
    pub fn is_simulation(&self) -> bool {
        *self == OHCI_COUNT_SELECT_A::SIMULATION
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == OHCI_COUNT_SELECT_A::NORMAL
    }
}
#[doc = "Field `ohci_count_select` writer - OHCI count select"]
pub type OHCI_COUNT_SELECT_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, HCI_INTERFACE_SPEC, OHCI_COUNT_SELECT_A, O>;
impl<'a, const O: u8> OHCI_COUNT_SELECT_W<'a, O> {
    #[doc = "Simulation mode. The counters will be much shorter then real time"]
    #[inline(always)]
    pub fn simulation(self) -> &'a mut W {
        self.variant(OHCI_COUNT_SELECT_A::SIMULATION)
    }
    #[doc = "Normal mode. The counters will count full time"]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(OHCI_COUNT_SELECT_A::NORMAL)
    }
}
#[doc = "Field `dma_transfer_status_enable` reader - DMA Transfer Status Enable"]
pub type DMA_TRANSFER_STATUS_ENABLE_R = crate::BitReader<DMA_TRANSFER_STATUS_ENABLE_A>;
#[doc = "DMA Transfer Status Enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMA_TRANSFER_STATUS_ENABLE_A {
    #[doc = "0: Disable"]
    DISABLE = 0,
    #[doc = "1: Enable"]
    ENABLE = 1,
}
impl From<DMA_TRANSFER_STATUS_ENABLE_A> for bool {
    #[inline(always)]
    fn from(variant: DMA_TRANSFER_STATUS_ENABLE_A) -> Self {
        variant as u8 != 0
    }
}
impl DMA_TRANSFER_STATUS_ENABLE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMA_TRANSFER_STATUS_ENABLE_A {
        match self.bits {
            false => DMA_TRANSFER_STATUS_ENABLE_A::DISABLE,
            true => DMA_TRANSFER_STATUS_ENABLE_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == DMA_TRANSFER_STATUS_ENABLE_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == DMA_TRANSFER_STATUS_ENABLE_A::ENABLE
    }
}
impl R {
    #[doc = "Bit 0 - ULPI bypass enable"]
    #[inline(always)]
    pub fn ulpi_bypass_enable(&self) -> ULPI_BYPASS_ENABLE_R {
        ULPI_BYPASS_ENABLE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - Master interface INCRX align enable"]
    #[inline(always)]
    pub fn ahb_master_interface_incrx_align_enable(
        &self,
    ) -> AHB_MASTER_INTERFACE_INCRX_ALIGN_ENABLE_R {
        AHB_MASTER_INTERFACE_INCRX_ALIGN_ENABLE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Master interface burst type INCR4 enable"]
    #[inline(always)]
    pub fn ahb_master_interface_burst_type_incr4_enable(
        &self,
    ) -> AHB_MASTER_INTERFACE_BURST_TYPE_INCR4_ENABLE_R {
        AHB_MASTER_INTERFACE_BURST_TYPE_INCR4_ENABLE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Master interface INCR8 enable"]
    #[inline(always)]
    pub fn ahb_master_interface_incr8_enable(&self) -> AHB_MASTER_INTERFACE_INCR8_ENABLE_R {
        AHB_MASTER_INTERFACE_INCR8_ENABLE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Master interface INCR16 enable"]
    #[inline(always)]
    pub fn ahb_master_interface_incr16_enable(&self) -> AHB_MASTER_INTERFACE_INCR16_ENABLE_R {
        AHB_MASTER_INTERFACE_INCR16_ENABLE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn pp2vbus(&self) -> PP2VBUS_R {
        PP2VBUS_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn resume_k_to_se0_transition(&self) -> RESUME_K_TO_SE0_TRANSITION_R {
        RESUME_K_TO_SE0_TRANSITION_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 25 - OHCI count select"]
    #[inline(always)]
    pub fn ohci_count_select(&self) -> OHCI_COUNT_SELECT_R {
        OHCI_COUNT_SELECT_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 28 - DMA Transfer Status Enable"]
    #[inline(always)]
    pub fn dma_transfer_status_enable(&self) -> DMA_TRANSFER_STATUS_ENABLE_R {
        DMA_TRANSFER_STATUS_ENABLE_R::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ULPI bypass enable"]
    #[inline(always)]
    #[must_use]
    pub fn ulpi_bypass_enable(&mut self) -> ULPI_BYPASS_ENABLE_W<0> {
        ULPI_BYPASS_ENABLE_W::new(self)
    }
    #[doc = "Bit 8 - Master interface INCRX align enable"]
    #[inline(always)]
    #[must_use]
    pub fn ahb_master_interface_incrx_align_enable(
        &mut self,
    ) -> AHB_MASTER_INTERFACE_INCRX_ALIGN_ENABLE_W<8> {
        AHB_MASTER_INTERFACE_INCRX_ALIGN_ENABLE_W::new(self)
    }
    #[doc = "Bit 9 - Master interface burst type INCR4 enable"]
    #[inline(always)]
    #[must_use]
    pub fn ahb_master_interface_burst_type_incr4_enable(
        &mut self,
    ) -> AHB_MASTER_INTERFACE_BURST_TYPE_INCR4_ENABLE_W<9> {
        AHB_MASTER_INTERFACE_BURST_TYPE_INCR4_ENABLE_W::new(self)
    }
    #[doc = "Bit 10 - Master interface INCR8 enable"]
    #[inline(always)]
    #[must_use]
    pub fn ahb_master_interface_incr8_enable(&mut self) -> AHB_MASTER_INTERFACE_INCR8_ENABLE_W<10> {
        AHB_MASTER_INTERFACE_INCR8_ENABLE_W::new(self)
    }
    #[doc = "Bit 11 - Master interface INCR16 enable"]
    #[inline(always)]
    #[must_use]
    pub fn ahb_master_interface_incr16_enable(
        &mut self,
    ) -> AHB_MASTER_INTERFACE_INCR16_ENABLE_W<11> {
        AHB_MASTER_INTERFACE_INCR16_ENABLE_W::new(self)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    #[must_use]
    pub fn pp2vbus(&mut self) -> PP2VBUS_W<12> {
        PP2VBUS_W::new(self)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    #[must_use]
    pub fn resume_k_to_se0_transition(&mut self) -> RESUME_K_TO_SE0_TRANSITION_W<18> {
        RESUME_K_TO_SE0_TRANSITION_W::new(self)
    }
    #[doc = "Bit 25 - OHCI count select"]
    #[inline(always)]
    #[must_use]
    pub fn ohci_count_select(&mut self) -> OHCI_COUNT_SELECT_W<25> {
        OHCI_COUNT_SELECT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "HCI Interface Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hci_interface](index.html) module"]
pub struct HCI_INTERFACE_SPEC;
impl crate::RegisterSpec for HCI_INTERFACE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hci_interface::R](R) reader structure"]
impl crate::Readable for HCI_INTERFACE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hci_interface::W](W) writer structure"]
impl crate::Writable for HCI_INTERFACE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets hci_interface to value 0x1000_0000"]
impl crate::Resettable for HCI_INTERFACE_SPEC {
    const RESET_VALUE: Self::Ux = 0x1000_0000;
}
