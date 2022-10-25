#[doc = "Register `twi_xaddr` reader"]
pub struct R(crate::R<TWI_XADDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TWI_XADDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TWI_XADDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TWI_XADDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `twi_xaddr` writer"]
pub struct W(crate::W<TWI_XADDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TWI_XADDR_SPEC>;
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
impl From<crate::W<TWI_XADDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TWI_XADDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `slax` reader - Extend Slave Address\n\nSLAX\\[7:0\\]"]
pub type SLAX_R = crate::FieldReader<u8, u8>;
#[doc = "Field `slax` writer - Extend Slave Address\n\nSLAX\\[7:0\\]"]
pub type SLAX_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TWI_XADDR_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Extend Slave Address\n\nSLAX\\[7:0\\]"]
    #[inline(always)]
    pub fn slax(&self) -> SLAX_R {
        SLAX_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Extend Slave Address\n\nSLAX\\[7:0\\]"]
    #[inline(always)]
    #[must_use]
    pub fn slax(&mut self) -> SLAX_W<0> {
        SLAX_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TWI Extended Slave Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [twi_xaddr](index.html) module"]
pub struct TWI_XADDR_SPEC;
impl crate::RegisterSpec for TWI_XADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [twi_xaddr::R](R) reader structure"]
impl crate::Readable for TWI_XADDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [twi_xaddr::W](W) writer structure"]
impl crate::Writable for TWI_XADDR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets twi_xaddr to value 0"]
impl crate::Resettable for TWI_XADDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
