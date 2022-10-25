#[doc = "Register `csic_bist_start_addr` reader"]
pub struct R(crate::R<CSIC_BIST_START_ADDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CSIC_BIST_START_ADDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CSIC_BIST_START_ADDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CSIC_BIST_START_ADDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `csic_bist_start_addr` writer"]
pub struct W(crate::W<CSIC_BIST_START_ADDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CSIC_BIST_START_ADDR_SPEC>;
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
impl From<crate::W<CSIC_BIST_START_ADDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CSIC_BIST_START_ADDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `bist_start_addr` reader - BIST start address, 32-bit aligned"]
pub type BIST_START_ADDR_R = crate::FieldReader<u32, u32>;
#[doc = "Field `bist_start_addr` writer - BIST start address, 32-bit aligned"]
pub type BIST_START_ADDR_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CSIC_BIST_START_ADDR_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - BIST start address, 32-bit aligned"]
    #[inline(always)]
    pub fn bist_start_addr(&self) -> BIST_START_ADDR_R {
        BIST_START_ADDR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - BIST start address, 32-bit aligned"]
    #[inline(always)]
    #[must_use]
    pub fn bist_start_addr(&mut self) -> BIST_START_ADDR_W<0> {
        BIST_START_ADDR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CSIC BIST Start Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csic_bist_start_addr](index.html) module"]
pub struct CSIC_BIST_START_ADDR_SPEC;
impl crate::RegisterSpec for CSIC_BIST_START_ADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [csic_bist_start_addr::R](R) reader structure"]
impl crate::Readable for CSIC_BIST_START_ADDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [csic_bist_start_addr::W](W) writer structure"]
impl crate::Writable for CSIC_BIST_START_ADDR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets csic_bist_start_addr to value 0"]
impl crate::Resettable for CSIC_BIST_START_ADDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
