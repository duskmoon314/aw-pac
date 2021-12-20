#[doc = "Register `TWI_XADDR` reader"]
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
#[doc = "Register `TWI_XADDR` writer"]
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
impl W {
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
}
#[doc = "`reset()` method sets TWI_XADDR to value 0"]
impl crate::Resettable for TWI_XADDR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
