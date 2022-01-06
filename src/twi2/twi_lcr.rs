#[doc = "Register `TWI_LCR` reader"]
pub struct R(crate::R<TWI_LCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TWI_LCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TWI_LCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TWI_LCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TWI_LCR` writer"]
pub struct W(crate::W<TWI_LCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TWI_LCR_SPEC>;
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
impl From<crate::W<TWI_LCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TWI_LCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Current State of TWI_SCL\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SCL_STATE_A {
    #[doc = "0: `0`"]
    LOW = 0,
    #[doc = "1: `1`"]
    HIGH = 1,
}
impl From<SCL_STATE_A> for bool {
    #[inline(always)]
    fn from(variant: SCL_STATE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `scl_state` reader - Current State of TWI_SCL"]
pub struct SCL_STATE_R(crate::FieldReader<bool, SCL_STATE_A>);
impl SCL_STATE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SCL_STATE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SCL_STATE_A {
        match self.bits {
            false => SCL_STATE_A::LOW,
            true => SCL_STATE_A::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        **self == SCL_STATE_A::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        **self == SCL_STATE_A::HIGH
    }
}
impl core::ops::Deref for SCL_STATE_R {
    type Target = crate::FieldReader<bool, SCL_STATE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Current State of TWI_SDA\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SDA_STATE_A {
    #[doc = "0: `0`"]
    LOW = 0,
    #[doc = "1: `1`"]
    HIGH = 1,
}
impl From<SDA_STATE_A> for bool {
    #[inline(always)]
    fn from(variant: SDA_STATE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `sda_state` reader - Current State of TWI_SDA"]
pub struct SDA_STATE_R(crate::FieldReader<bool, SDA_STATE_A>);
impl SDA_STATE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SDA_STATE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SDA_STATE_A {
        match self.bits {
            false => SDA_STATE_A::LOW,
            true => SDA_STATE_A::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        **self == SDA_STATE_A::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        **self == SDA_STATE_A::HIGH
    }
}
impl core::ops::Deref for SDA_STATE_R {
    type Target = crate::FieldReader<bool, SDA_STATE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "TWI_SCL Line State Control Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SCL_CTL_A {
    #[doc = "0: `0`"]
    LOW = 0,
    #[doc = "1: `1`"]
    HIGH = 1,
}
impl From<SCL_CTL_A> for bool {
    #[inline(always)]
    fn from(variant: SCL_CTL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `scl_ctl` reader - TWI_SCL Line State Control Bit"]
pub struct SCL_CTL_R(crate::FieldReader<bool, SCL_CTL_A>);
impl SCL_CTL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SCL_CTL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SCL_CTL_A {
        match self.bits {
            false => SCL_CTL_A::LOW,
            true => SCL_CTL_A::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        **self == SCL_CTL_A::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        **self == SCL_CTL_A::HIGH
    }
}
impl core::ops::Deref for SCL_CTL_R {
    type Target = crate::FieldReader<bool, SCL_CTL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `scl_ctl` writer - TWI_SCL Line State Control Bit"]
pub struct SCL_CTL_W<'a> {
    w: &'a mut W,
}
impl<'a> SCL_CTL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SCL_CTL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(SCL_CTL_A::LOW)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(SCL_CTL_A::HIGH)
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
#[doc = "TWI_SCL Line State Control Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SCL_CTL_EN_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<SCL_CTL_EN_A> for bool {
    #[inline(always)]
    fn from(variant: SCL_CTL_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `scl_ctl_en` reader - TWI_SCL Line State Control Enable"]
pub struct SCL_CTL_EN_R(crate::FieldReader<bool, SCL_CTL_EN_A>);
impl SCL_CTL_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SCL_CTL_EN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SCL_CTL_EN_A {
        match self.bits {
            false => SCL_CTL_EN_A::DISABLE,
            true => SCL_CTL_EN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == SCL_CTL_EN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == SCL_CTL_EN_A::ENABLE
    }
}
impl core::ops::Deref for SCL_CTL_EN_R {
    type Target = crate::FieldReader<bool, SCL_CTL_EN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `scl_ctl_en` writer - TWI_SCL Line State Control Enable"]
pub struct SCL_CTL_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SCL_CTL_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SCL_CTL_EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(SCL_CTL_EN_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(SCL_CTL_EN_A::ENABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "TWI_SDA Line State Control Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SDA_CTL_A {
    #[doc = "0: `0`"]
    LOW = 0,
    #[doc = "1: `1`"]
    HIGH = 1,
}
impl From<SDA_CTL_A> for bool {
    #[inline(always)]
    fn from(variant: SDA_CTL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `sda_ctl` reader - TWI_SDA Line State Control Bit"]
pub struct SDA_CTL_R(crate::FieldReader<bool, SDA_CTL_A>);
impl SDA_CTL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SDA_CTL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SDA_CTL_A {
        match self.bits {
            false => SDA_CTL_A::LOW,
            true => SDA_CTL_A::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        **self == SDA_CTL_A::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        **self == SDA_CTL_A::HIGH
    }
}
impl core::ops::Deref for SDA_CTL_R {
    type Target = crate::FieldReader<bool, SDA_CTL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `sda_ctl` writer - TWI_SDA Line State Control Bit"]
pub struct SDA_CTL_W<'a> {
    w: &'a mut W,
}
impl<'a> SDA_CTL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SDA_CTL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(SDA_CTL_A::LOW)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(SDA_CTL_A::HIGH)
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "TWI_SDA Line State Control Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SDA_CTL_EN_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<SDA_CTL_EN_A> for bool {
    #[inline(always)]
    fn from(variant: SDA_CTL_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `sda_ctl_en` reader - TWI_SDA Line State Control Enable"]
pub struct SDA_CTL_EN_R(crate::FieldReader<bool, SDA_CTL_EN_A>);
impl SDA_CTL_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SDA_CTL_EN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SDA_CTL_EN_A {
        match self.bits {
            false => SDA_CTL_EN_A::DISABLE,
            true => SDA_CTL_EN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == SDA_CTL_EN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == SDA_CTL_EN_A::ENABLE
    }
}
impl core::ops::Deref for SDA_CTL_EN_R {
    type Target = crate::FieldReader<bool, SDA_CTL_EN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `sda_ctl_en` writer - TWI_SDA Line State Control Enable"]
pub struct SDA_CTL_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SDA_CTL_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SDA_CTL_EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(SDA_CTL_EN_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(SDA_CTL_EN_A::ENABLE)
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 5 - Current State of TWI_SCL"]
    #[inline(always)]
    pub fn scl_state(&self) -> SCL_STATE_R {
        SCL_STATE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Current State of TWI_SDA"]
    #[inline(always)]
    pub fn sda_state(&self) -> SDA_STATE_R {
        SDA_STATE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - TWI_SCL Line State Control Bit"]
    #[inline(always)]
    pub fn scl_ctl(&self) -> SCL_CTL_R {
        SCL_CTL_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - TWI_SCL Line State Control Enable"]
    #[inline(always)]
    pub fn scl_ctl_en(&self) -> SCL_CTL_EN_R {
        SCL_CTL_EN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - TWI_SDA Line State Control Bit"]
    #[inline(always)]
    pub fn sda_ctl(&self) -> SDA_CTL_R {
        SDA_CTL_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - TWI_SDA Line State Control Enable"]
    #[inline(always)]
    pub fn sda_ctl_en(&self) -> SDA_CTL_EN_R {
        SDA_CTL_EN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - TWI_SCL Line State Control Bit"]
    #[inline(always)]
    pub fn scl_ctl(&mut self) -> SCL_CTL_W {
        SCL_CTL_W { w: self }
    }
    #[doc = "Bit 2 - TWI_SCL Line State Control Enable"]
    #[inline(always)]
    pub fn scl_ctl_en(&mut self) -> SCL_CTL_EN_W {
        SCL_CTL_EN_W { w: self }
    }
    #[doc = "Bit 1 - TWI_SDA Line State Control Bit"]
    #[inline(always)]
    pub fn sda_ctl(&mut self) -> SDA_CTL_W {
        SDA_CTL_W { w: self }
    }
    #[doc = "Bit 0 - TWI_SDA Line State Control Enable"]
    #[inline(always)]
    pub fn sda_ctl_en(&mut self) -> SDA_CTL_EN_W {
        SDA_CTL_EN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TWI Line Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [twi_lcr](index.html) module"]
pub struct TWI_LCR_SPEC;
impl crate::RegisterSpec for TWI_LCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [twi_lcr::R](R) reader structure"]
impl crate::Readable for TWI_LCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [twi_lcr::W](W) writer structure"]
impl crate::Writable for TWI_LCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TWI_LCR to value 0"]
impl crate::Resettable for TWI_LCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
