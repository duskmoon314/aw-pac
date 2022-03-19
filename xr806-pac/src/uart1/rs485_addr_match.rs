#[doc = "Register `RS485_ADDR_MATCH` reader"]
pub struct R(crate::R<RS485_ADDR_MATCH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RS485_ADDR_MATCH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RS485_ADDR_MATCH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RS485_ADDR_MATCH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RS485_ADDR_MATCH` writer"]
pub struct W(crate::W<RS485_ADDR_MATCH_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RS485_ADDR_MATCH_SPEC>;
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
impl From<crate::W<RS485_ADDR_MATCH_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RS485_ADDR_MATCH_SPEC>) -> Self {
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
#[doc = "UART RS485 Address Match Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rs485_addr_match](index.html) module"]
pub struct RS485_ADDR_MATCH_SPEC;
impl crate::RegisterSpec for RS485_ADDR_MATCH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rs485_addr_match::R](R) reader structure"]
impl crate::Readable for RS485_ADDR_MATCH_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rs485_addr_match::W](W) writer structure"]
impl crate::Writable for RS485_ADDR_MATCH_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RS485_ADDR_MATCH to value 0"]
impl crate::Resettable for RS485_ADDR_MATCH_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
