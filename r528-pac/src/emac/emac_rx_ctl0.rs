#[doc = "Register `EMAC_RX_CTL0` reader"]
pub struct R(crate::R<EMAC_RX_CTL0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EMAC_RX_CTL0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EMAC_RX_CTL0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EMAC_RX_CTL0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EMAC_RX_CTL0` writer"]
pub struct W(crate::W<EMAC_RX_CTL0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EMAC_RX_CTL0_SPEC>;
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
impl From<crate::W<EMAC_RX_CTL0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EMAC_RX_CTL0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Enable Receiver\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RX_EN_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<RX_EN_A> for bool {
    #[inline(always)]
    fn from(variant: RX_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RX_EN` reader - Enable Receiver"]
pub struct RX_EN_R(crate::FieldReader<bool, RX_EN_A>);
impl RX_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RX_EN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RX_EN_A {
        match self.bits {
            false => RX_EN_A::DISABLE,
            true => RX_EN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == RX_EN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == RX_EN_A::ENABLE
    }
}
impl core::ops::Deref for RX_EN_R {
    type Target = crate::FieldReader<bool, RX_EN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RX_EN` writer - Enable Receiver"]
pub struct RX_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RX_EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(RX_EN_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(RX_EN_A::ENABLE)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 31)) | ((value as u32 & 0x01) << 31);
        self.w
    }
}
#[doc = "Frame Receive Length Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RX_FRM_LEN_CTL_A {
    #[doc = "0: `0`"]
    B2048 = 0,
    #[doc = "1: `1`"]
    B16384 = 1,
}
impl From<RX_FRM_LEN_CTL_A> for bool {
    #[inline(always)]
    fn from(variant: RX_FRM_LEN_CTL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RX_FRM_LEN_CTL` reader - Frame Receive Length Control"]
pub struct RX_FRM_LEN_CTL_R(crate::FieldReader<bool, RX_FRM_LEN_CTL_A>);
impl RX_FRM_LEN_CTL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RX_FRM_LEN_CTL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RX_FRM_LEN_CTL_A {
        match self.bits {
            false => RX_FRM_LEN_CTL_A::B2048,
            true => RX_FRM_LEN_CTL_A::B16384,
        }
    }
    #[doc = "Checks if the value of the field is `B2048`"]
    #[inline(always)]
    pub fn is_b2048(&self) -> bool {
        **self == RX_FRM_LEN_CTL_A::B2048
    }
    #[doc = "Checks if the value of the field is `B16384`"]
    #[inline(always)]
    pub fn is_b16384(&self) -> bool {
        **self == RX_FRM_LEN_CTL_A::B16384
    }
}
impl core::ops::Deref for RX_FRM_LEN_CTL_R {
    type Target = crate::FieldReader<bool, RX_FRM_LEN_CTL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RX_FRM_LEN_CTL` writer - Frame Receive Length Control"]
pub struct RX_FRM_LEN_CTL_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_FRM_LEN_CTL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RX_FRM_LEN_CTL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn b2048(self) -> &'a mut W {
        self.variant(RX_FRM_LEN_CTL_A::B2048)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn b16384(self) -> &'a mut W {
        self.variant(RX_FRM_LEN_CTL_A::B16384)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 30)) | ((value as u32 & 0x01) << 30);
        self.w
    }
}
#[doc = "Jumbo Frame Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum JUMBO_FRM_EN_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<JUMBO_FRM_EN_A> for bool {
    #[inline(always)]
    fn from(variant: JUMBO_FRM_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `JUMBO_FRM_EN` reader - Jumbo Frame Enable"]
pub struct JUMBO_FRM_EN_R(crate::FieldReader<bool, JUMBO_FRM_EN_A>);
impl JUMBO_FRM_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        JUMBO_FRM_EN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> JUMBO_FRM_EN_A {
        match self.bits {
            false => JUMBO_FRM_EN_A::DISABLE,
            true => JUMBO_FRM_EN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == JUMBO_FRM_EN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == JUMBO_FRM_EN_A::ENABLE
    }
}
impl core::ops::Deref for JUMBO_FRM_EN_R {
    type Target = crate::FieldReader<bool, JUMBO_FRM_EN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `JUMBO_FRM_EN` writer - Jumbo Frame Enable"]
pub struct JUMBO_FRM_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> JUMBO_FRM_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: JUMBO_FRM_EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(JUMBO_FRM_EN_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(JUMBO_FRM_EN_A::ENABLE)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 29)) | ((value as u32 & 0x01) << 29);
        self.w
    }
}
#[doc = "Field `STRIP_FCS` reader - "]
pub struct STRIP_FCS_R(crate::FieldReader<bool, bool>);
impl STRIP_FCS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        STRIP_FCS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STRIP_FCS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STRIP_FCS` writer - "]
pub struct STRIP_FCS_W<'a> {
    w: &'a mut W,
}
impl<'a> STRIP_FCS_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 28)) | ((value as u32 & 0x01) << 28);
        self.w
    }
}
#[doc = "Check CRC Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHECK_CRC_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    CHECK = 1,
}
impl From<CHECK_CRC_A> for bool {
    #[inline(always)]
    fn from(variant: CHECK_CRC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHECK_CRC` reader - Check CRC Enable"]
pub struct CHECK_CRC_R(crate::FieldReader<bool, CHECK_CRC_A>);
impl CHECK_CRC_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CHECK_CRC_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHECK_CRC_A {
        match self.bits {
            false => CHECK_CRC_A::DISABLE,
            true => CHECK_CRC_A::CHECK,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == CHECK_CRC_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `CHECK`"]
    #[inline(always)]
    pub fn is_check(&self) -> bool {
        **self == CHECK_CRC_A::CHECK
    }
}
impl core::ops::Deref for CHECK_CRC_R {
    type Target = crate::FieldReader<bool, CHECK_CRC_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHECK_CRC` writer - Check CRC Enable"]
pub struct CHECK_CRC_W<'a> {
    w: &'a mut W,
}
impl<'a> CHECK_CRC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CHECK_CRC_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CHECK_CRC_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn check(self) -> &'a mut W {
        self.variant(CHECK_CRC_A::CHECK)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 27)) | ((value as u32 & 0x01) << 27);
        self.w
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RX_PAUSE_FRM_MD_A {
    #[doc = "0: `0`"]
    ONLY_MULTICAST = 0,
    #[doc = "1: `1`"]
    ALSO_UNICAST_MAC0 = 1,
}
impl From<RX_PAUSE_FRM_MD_A> for bool {
    #[inline(always)]
    fn from(variant: RX_PAUSE_FRM_MD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RX_PAUSE_FRM_MD` reader - "]
pub struct RX_PAUSE_FRM_MD_R(crate::FieldReader<bool, RX_PAUSE_FRM_MD_A>);
impl RX_PAUSE_FRM_MD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RX_PAUSE_FRM_MD_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RX_PAUSE_FRM_MD_A {
        match self.bits {
            false => RX_PAUSE_FRM_MD_A::ONLY_MULTICAST,
            true => RX_PAUSE_FRM_MD_A::ALSO_UNICAST_MAC0,
        }
    }
    #[doc = "Checks if the value of the field is `ONLY_MULTICAST`"]
    #[inline(always)]
    pub fn is_only_multicast(&self) -> bool {
        **self == RX_PAUSE_FRM_MD_A::ONLY_MULTICAST
    }
    #[doc = "Checks if the value of the field is `ALSO_UNICAST_MAC0`"]
    #[inline(always)]
    pub fn is_also_unicast_mac0(&self) -> bool {
        **self == RX_PAUSE_FRM_MD_A::ALSO_UNICAST_MAC0
    }
}
impl core::ops::Deref for RX_PAUSE_FRM_MD_R {
    type Target = crate::FieldReader<bool, RX_PAUSE_FRM_MD_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RX_PAUSE_FRM_MD` writer - "]
pub struct RX_PAUSE_FRM_MD_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_PAUSE_FRM_MD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RX_PAUSE_FRM_MD_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn only_multicast(self) -> &'a mut W {
        self.variant(RX_PAUSE_FRM_MD_A::ONLY_MULTICAST)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn also_unicast_mac0(self) -> &'a mut W {
        self.variant(RX_PAUSE_FRM_MD_A::ALSO_UNICAST_MAC0)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 17)) | ((value as u32 & 0x01) << 17);
        self.w
    }
}
#[doc = "Field `RX_FLOW_CTL_EN` reader - "]
pub struct RX_FLOW_CTL_EN_R(crate::FieldReader<bool, bool>);
impl RX_FLOW_CTL_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RX_FLOW_CTL_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RX_FLOW_CTL_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RX_FLOW_CTL_EN` writer - "]
pub struct RX_FLOW_CTL_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_FLOW_CTL_EN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 16)) | ((value as u32 & 0x01) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bit 31 - Enable Receiver"]
    #[inline(always)]
    pub fn rx_en(&self) -> RX_EN_R {
        RX_EN_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Frame Receive Length Control"]
    #[inline(always)]
    pub fn rx_frm_len_ctl(&self) -> RX_FRM_LEN_CTL_R {
        RX_FRM_LEN_CTL_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Jumbo Frame Enable"]
    #[inline(always)]
    pub fn jumbo_frm_en(&self) -> JUMBO_FRM_EN_R {
        JUMBO_FRM_EN_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn strip_fcs(&self) -> STRIP_FCS_R {
        STRIP_FCS_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Check CRC Enable"]
    #[inline(always)]
    pub fn check_crc(&self) -> CHECK_CRC_R {
        CHECK_CRC_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn rx_pause_frm_md(&self) -> RX_PAUSE_FRM_MD_R {
        RX_PAUSE_FRM_MD_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn rx_flow_ctl_en(&self) -> RX_FLOW_CTL_EN_R {
        RX_FLOW_CTL_EN_R::new(((self.bits >> 16) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 31 - Enable Receiver"]
    #[inline(always)]
    pub fn rx_en(&mut self) -> RX_EN_W {
        RX_EN_W { w: self }
    }
    #[doc = "Bit 30 - Frame Receive Length Control"]
    #[inline(always)]
    pub fn rx_frm_len_ctl(&mut self) -> RX_FRM_LEN_CTL_W {
        RX_FRM_LEN_CTL_W { w: self }
    }
    #[doc = "Bit 29 - Jumbo Frame Enable"]
    #[inline(always)]
    pub fn jumbo_frm_en(&mut self) -> JUMBO_FRM_EN_W {
        JUMBO_FRM_EN_W { w: self }
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn strip_fcs(&mut self) -> STRIP_FCS_W {
        STRIP_FCS_W { w: self }
    }
    #[doc = "Bit 27 - Check CRC Enable"]
    #[inline(always)]
    pub fn check_crc(&mut self) -> CHECK_CRC_W {
        CHECK_CRC_W { w: self }
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn rx_pause_frm_md(&mut self) -> RX_PAUSE_FRM_MD_W {
        RX_PAUSE_FRM_MD_W { w: self }
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn rx_flow_ctl_en(&mut self) -> RX_FLOW_CTL_EN_W {
        RX_FLOW_CTL_EN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "EMAC Receive Control Register0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [emac_rx_ctl0](index.html) module"]
pub struct EMAC_RX_CTL0_SPEC;
impl crate::RegisterSpec for EMAC_RX_CTL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [emac_rx_ctl0::R](R) reader structure"]
impl crate::Readable for EMAC_RX_CTL0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [emac_rx_ctl0::W](W) writer structure"]
impl crate::Writable for EMAC_RX_CTL0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EMAC_RX_CTL0 to value 0"]
impl crate::Resettable for EMAC_RX_CTL0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
