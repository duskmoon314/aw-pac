#[doc = "Register `CONFIGFLAG` reader"]
pub struct R(crate::R<CONFIGFLAG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CONFIGFLAG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CONFIGFLAG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CONFIGFLAG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CONFIGFLAG` writer"]
pub struct W(crate::W<CONFIGFLAG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CONFIGFLAG_SPEC>;
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
impl From<crate::W<CONFIGFLAG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CONFIGFLAG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CF` reader - Configure Flag (CF)\n\nHost software sets this bit as the last action in its process of configuring the Host Controller."]
pub type CF_R = crate::BitReader<bool>;
#[doc = "Field `CF` writer - Configure Flag (CF)\n\nHost software sets this bit as the last action in its process of configuring the Host Controller."]
pub type CF_W<'a> = crate::BitWriter<'a, u32, CONFIGFLAG_SPEC, bool, 0>;
impl R {
    #[doc = "Bit 0 - Configure Flag (CF)\n\nHost software sets this bit as the last action in its process of configuring the Host Controller."]
    #[inline(always)]
    pub fn cf(&self) -> CF_R {
        CF_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Configure Flag (CF)\n\nHost software sets this bit as the last action in its process of configuring the Host Controller."]
    #[inline(always)]
    pub fn cf(&mut self) -> CF_W {
        CF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "EHCI Configure Flag Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [configflag](index.html) module"]
pub struct CONFIGFLAG_SPEC;
impl crate::RegisterSpec for CONFIGFLAG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [configflag::R](R) reader structure"]
impl crate::Readable for CONFIGFLAG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [configflag::W](W) writer structure"]
impl crate::Writable for CONFIGFLAG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CONFIGFLAG to value 0"]
impl crate::Resettable for CONFIGFLAG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
