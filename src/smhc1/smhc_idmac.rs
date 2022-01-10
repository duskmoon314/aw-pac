#[doc = "Register `SMHC_IDMAC` reader"]
pub struct R(crate::R<SMHC_IDMAC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SMHC_IDMAC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SMHC_IDMAC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SMHC_IDMAC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SMHC_IDMAC` writer"]
pub struct W(crate::W<SMHC_IDMAC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SMHC_IDMAC_SPEC>;
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
impl From<crate::W<SMHC_IDMAC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SMHC_IDMAC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DES_LOAD_CTRL` writer - "]
pub struct DES_LOAD_CTRL_W<'a> {
    w: &'a mut W,
}
impl<'a> DES_LOAD_CTRL_W<'a> {
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
#[doc = "Field `IDMAC_ENB` reader - IDMAC Enable"]
pub struct IDMAC_ENB_R(crate::FieldReader<bool, bool>);
impl IDMAC_ENB_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        IDMAC_ENB_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IDMAC_ENB_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IDMAC_ENB` writer - IDMAC Enable"]
pub struct IDMAC_ENB_W<'a> {
    w: &'a mut W,
}
impl<'a> IDMAC_ENB_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
#[doc = "Field `FIX_BUST_CTRL` reader - Fixed Burst"]
pub struct FIX_BUST_CTRL_R(crate::FieldReader<bool, bool>);
impl FIX_BUST_CTRL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FIX_BUST_CTRL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FIX_BUST_CTRL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FIX_BUST_CTRL` writer - Fixed Burst"]
pub struct FIX_BUST_CTRL_W<'a> {
    w: &'a mut W,
}
impl<'a> FIX_BUST_CTRL_W<'a> {
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
#[doc = "Field `IDMAC_RST` reader - DMA Reset"]
pub struct IDMAC_RST_R(crate::FieldReader<bool, bool>);
impl IDMAC_RST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        IDMAC_RST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IDMAC_RST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IDMAC_RST` writer - DMA Reset"]
pub struct IDMAC_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> IDMAC_RST_W<'a> {
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
    #[doc = "Bit 7 - IDMAC Enable"]
    #[inline(always)]
    pub fn idmac_enb(&self) -> IDMAC_ENB_R {
        IDMAC_ENB_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Fixed Burst"]
    #[inline(always)]
    pub fn fix_bust_ctrl(&self) -> FIX_BUST_CTRL_R {
        FIX_BUST_CTRL_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - DMA Reset"]
    #[inline(always)]
    pub fn idmac_rst(&self) -> IDMAC_RST_R {
        IDMAC_RST_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn des_load_ctrl(&mut self) -> DES_LOAD_CTRL_W {
        DES_LOAD_CTRL_W { w: self }
    }
    #[doc = "Bit 7 - IDMAC Enable"]
    #[inline(always)]
    pub fn idmac_enb(&mut self) -> IDMAC_ENB_W {
        IDMAC_ENB_W { w: self }
    }
    #[doc = "Bit 1 - Fixed Burst"]
    #[inline(always)]
    pub fn fix_bust_ctrl(&mut self) -> FIX_BUST_CTRL_W {
        FIX_BUST_CTRL_W { w: self }
    }
    #[doc = "Bit 0 - DMA Reset"]
    #[inline(always)]
    pub fn idmac_rst(&mut self) -> IDMAC_RST_W {
        IDMAC_RST_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "IDMAC Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [smhc_idmac](index.html) module"]
pub struct SMHC_IDMAC_SPEC;
impl crate::RegisterSpec for SMHC_IDMAC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [smhc_idmac::R](R) reader structure"]
impl crate::Readable for SMHC_IDMAC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [smhc_idmac::W](W) writer structure"]
impl crate::Writable for SMHC_IDMAC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SMHC_IDMAC to value 0"]
impl crate::Resettable for SMHC_IDMAC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
