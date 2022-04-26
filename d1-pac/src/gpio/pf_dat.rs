#[doc = "Register `pf_dat` reader"]
pub struct R(crate::R<PF_DAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PF_DAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PF_DAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PF_DAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `pf_dat` writer"]
pub struct W(crate::W<PF_DAT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PF_DAT_SPEC>;
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
impl From<crate::W<PF_DAT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PF_DAT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PF_DAT` reader - PF Data"]
pub struct PF_DAT_R(crate::FieldReader<u8>);
impl PF_DAT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PF_DAT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PF_DAT_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PF_DAT` writer - PF Data"]
pub struct PF_DAT_W<'a> {
    w: &'a mut W,
}
impl<'a> PF_DAT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | (value as u32 & 0x7f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:6 - PF Data"]
    #[inline(always)]
    pub fn pf_dat(&self) -> PF_DAT_R {
        PF_DAT_R::new((self.bits & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - PF Data"]
    #[inline(always)]
    pub fn pf_dat(&mut self) -> PF_DAT_W {
        PF_DAT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PF Data Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pf_dat](index.html) module"]
pub struct PF_DAT_SPEC;
impl crate::RegisterSpec for PF_DAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pf_dat::R](R) reader structure"]
impl crate::Readable for PF_DAT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pf_dat::W](W) writer structure"]
impl crate::Writable for PF_DAT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets pf_dat to value 0"]
impl crate::Resettable for PF_DAT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
