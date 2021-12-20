#[doc = "Register `pg_eint_deb` reader"]
pub struct R(crate::R<PG_EINT_DEB_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PG_EINT_DEB_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PG_EINT_DEB_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PG_EINT_DEB_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `pg_eint_deb` writer"]
pub struct W(crate::W<PG_EINT_DEB_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PG_EINT_DEB_SPEC>;
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
impl From<crate::W<PG_EINT_DEB_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PG_EINT_DEB_SPEC>) -> Self {
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
#[doc = "PG External Interrupt Debounce Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pg_eint_deb](index.html) module"]
pub struct PG_EINT_DEB_SPEC;
impl crate::RegisterSpec for PG_EINT_DEB_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pg_eint_deb::R](R) reader structure"]
impl crate::Readable for PG_EINT_DEB_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pg_eint_deb::W](W) writer structure"]
impl crate::Writable for PG_EINT_DEB_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets pg_eint_deb to value 0"]
impl crate::Resettable for PG_EINT_DEB_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
