#[doc = "Register `SMHC_THLD` reader"]
pub struct R(crate::R<SMHC_THLD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SMHC_THLD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SMHC_THLD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SMHC_THLD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SMHC_THLD` writer"]
pub struct W(crate::W<SMHC_THLD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SMHC_THLD_SPEC>;
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
impl From<crate::W<SMHC_THLD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SMHC_THLD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CARD_WR_THLD` reader - Card Read/Write Threshold Size"]
pub struct CARD_WR_THLD_R(crate::FieldReader<u16, u16>);
impl CARD_WR_THLD_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        CARD_WR_THLD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CARD_WR_THLD_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CARD_WR_THLD` writer - Card Read/Write Threshold Size"]
pub struct CARD_WR_THLD_W<'a> {
    w: &'a mut W,
}
impl<'a> CARD_WR_THLD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0fff << 16)) | ((value as u32 & 0x0fff) << 16);
        self.w
    }
}
#[doc = "Card Read/Write Threshold Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CARD_WR_THLD_ENB_A {
    #[doc = "0: Card write threshold disabled"]
    DISABLED = 0,
    #[doc = "1: Card write threshold enabled"]
    ENABLED = 1,
}
impl From<CARD_WR_THLD_ENB_A> for bool {
    #[inline(always)]
    fn from(variant: CARD_WR_THLD_ENB_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CARD_WR_THLD_ENB` reader - Card Read/Write Threshold Enable"]
pub struct CARD_WR_THLD_ENB_R(crate::FieldReader<bool, CARD_WR_THLD_ENB_A>);
impl CARD_WR_THLD_ENB_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CARD_WR_THLD_ENB_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CARD_WR_THLD_ENB_A {
        match self.bits {
            false => CARD_WR_THLD_ENB_A::DISABLED,
            true => CARD_WR_THLD_ENB_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == CARD_WR_THLD_ENB_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == CARD_WR_THLD_ENB_A::ENABLED
    }
}
impl core::ops::Deref for CARD_WR_THLD_ENB_R {
    type Target = crate::FieldReader<bool, CARD_WR_THLD_ENB_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CARD_WR_THLD_ENB` writer - Card Read/Write Threshold Enable"]
pub struct CARD_WR_THLD_ENB_W<'a> {
    w: &'a mut W,
}
impl<'a> CARD_WR_THLD_ENB_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CARD_WR_THLD_ENB_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Card write threshold disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CARD_WR_THLD_ENB_A::DISABLED)
    }
    #[doc = "Card write threshold enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CARD_WR_THLD_ENB_A::ENABLED)
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
#[doc = "Busy Clear Interrupt Generation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BCIG_A {
    #[doc = "0: Busy clear interrupt disabled"]
    DISABLED = 0,
    #[doc = "1: Busy clear interrupt enabled"]
    ENABLED = 1,
}
impl From<BCIG_A> for bool {
    #[inline(always)]
    fn from(variant: BCIG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BCIG` reader - Busy Clear Interrupt Generation"]
pub struct BCIG_R(crate::FieldReader<bool, BCIG_A>);
impl BCIG_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BCIG_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BCIG_A {
        match self.bits {
            false => BCIG_A::DISABLED,
            true => BCIG_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == BCIG_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == BCIG_A::ENABLED
    }
}
impl core::ops::Deref for BCIG_R {
    type Target = crate::FieldReader<bool, BCIG_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BCIG` writer - Busy Clear Interrupt Generation"]
pub struct BCIG_W<'a> {
    w: &'a mut W,
}
impl<'a> BCIG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BCIG_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Busy clear interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(BCIG_A::DISABLED)
    }
    #[doc = "Busy clear interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(BCIG_A::ENABLED)
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
#[doc = "Card Read Threshold Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CARD_RD_THLD_ENB_A {
    #[doc = "0: Card read threshold disabled"]
    DISABLED = 0,
    #[doc = "1: Card read threshold enabled"]
    ENABLED = 1,
}
impl From<CARD_RD_THLD_ENB_A> for bool {
    #[inline(always)]
    fn from(variant: CARD_RD_THLD_ENB_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CARD_RD_THLD_ENB` reader - Card Read Threshold Enable"]
pub struct CARD_RD_THLD_ENB_R(crate::FieldReader<bool, CARD_RD_THLD_ENB_A>);
impl CARD_RD_THLD_ENB_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CARD_RD_THLD_ENB_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CARD_RD_THLD_ENB_A {
        match self.bits {
            false => CARD_RD_THLD_ENB_A::DISABLED,
            true => CARD_RD_THLD_ENB_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == CARD_RD_THLD_ENB_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == CARD_RD_THLD_ENB_A::ENABLED
    }
}
impl core::ops::Deref for CARD_RD_THLD_ENB_R {
    type Target = crate::FieldReader<bool, CARD_RD_THLD_ENB_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CARD_RD_THLD_ENB` writer - Card Read Threshold Enable"]
pub struct CARD_RD_THLD_ENB_W<'a> {
    w: &'a mut W,
}
impl<'a> CARD_RD_THLD_ENB_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CARD_RD_THLD_ENB_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Card read threshold disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CARD_RD_THLD_ENB_A::DISABLED)
    }
    #[doc = "Card read threshold enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CARD_RD_THLD_ENB_A::ENABLED)
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
    #[doc = "Bits 16:27 - Card Read/Write Threshold Size"]
    #[inline(always)]
    pub fn card_wr_thld(&self) -> CARD_WR_THLD_R {
        CARD_WR_THLD_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
    #[doc = "Bit 2 - Card Read/Write Threshold Enable"]
    #[inline(always)]
    pub fn card_wr_thld_enb(&self) -> CARD_WR_THLD_ENB_R {
        CARD_WR_THLD_ENB_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Busy Clear Interrupt Generation"]
    #[inline(always)]
    pub fn bcig(&self) -> BCIG_R {
        BCIG_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Card Read Threshold Enable"]
    #[inline(always)]
    pub fn card_rd_thld_enb(&self) -> CARD_RD_THLD_ENB_R {
        CARD_RD_THLD_ENB_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 16:27 - Card Read/Write Threshold Size"]
    #[inline(always)]
    pub fn card_wr_thld(&mut self) -> CARD_WR_THLD_W {
        CARD_WR_THLD_W { w: self }
    }
    #[doc = "Bit 2 - Card Read/Write Threshold Enable"]
    #[inline(always)]
    pub fn card_wr_thld_enb(&mut self) -> CARD_WR_THLD_ENB_W {
        CARD_WR_THLD_ENB_W { w: self }
    }
    #[doc = "Bit 1 - Busy Clear Interrupt Generation"]
    #[inline(always)]
    pub fn bcig(&mut self) -> BCIG_W {
        BCIG_W { w: self }
    }
    #[doc = "Bit 0 - Card Read Threshold Enable"]
    #[inline(always)]
    pub fn card_rd_thld_enb(&mut self) -> CARD_RD_THLD_ENB_W {
        CARD_RD_THLD_ENB_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Card Threshold Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [smhc_thld](index.html) module"]
pub struct SMHC_THLD_SPEC;
impl crate::RegisterSpec for SMHC_THLD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [smhc_thld::R](R) reader structure"]
impl crate::Readable for SMHC_THLD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [smhc_thld::W](W) writer structure"]
impl crate::Writable for SMHC_THLD_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SMHC_THLD to value 0"]
impl crate::Resettable for SMHC_THLD_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
