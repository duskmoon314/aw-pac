#[doc = "Register `twi_srst` reader"]
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
#[doc = "Register `twi_srst` writer"]
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
pub type SOFT_RST_R = crate::BitReader<bool>;
#[doc = "Field `soft_rst` writer - Soft Reset"]
pub type SOFT_RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, TWI_SRST_SPEC, bool, O>;
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
    #[must_use]
    pub fn soft_rst(&mut self) -> SOFT_RST_W<0> {
        SOFT_RST_W::new(self)
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
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets twi_srst to value 0"]
impl crate::Resettable for TWI_SRST_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
