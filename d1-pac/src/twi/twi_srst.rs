#[doc = "Register `TWI_SRST` reader"]
pub struct R(crate::R<TWI_SRST_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TWI_SRST_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TWI_SRST_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TWI_SRST_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TWI_SRST` writer"]
pub struct W(crate::W<TWI_SRST_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TWI_SRST_SPEC>;
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
impl From<crate::W<TWI_SRST_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TWI_SRST_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `soft_rst` reader - Soft Reset"]
pub struct SOFT_RST_R(crate::FieldReader<bool>);
impl SOFT_RST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SOFT_RST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SOFT_RST_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `soft_rst` writer - Soft Reset"]
pub struct SOFT_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> SOFT_RST_W<'a> {
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
        self.w.bits = (self.w.bits & !1) | (value as u32 & 1);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Soft Reset"]
    #[inline(always)]
    pub fn soft_rst(&self) -> SOFT_RST_R {
        SOFT_RST_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Soft Reset"]
    #[inline(always)]
    pub fn soft_rst(&mut self) -> SOFT_RST_W {
        SOFT_RST_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TWI Software Reset Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [twi_srst](index.html) module"]
pub struct TWI_SRST_SPEC;
impl crate::RegisterSpec for TWI_SRST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [twi_srst::R](R) reader structure"]
impl crate::Readable for TWI_SRST_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [twi_srst::W](W) writer structure"]
impl crate::Writable for TWI_SRST_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TWI_SRST to value 0"]
impl crate::Resettable for TWI_SRST_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
