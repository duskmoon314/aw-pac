#[doc = "Register `tve_level` reader"]
pub struct R(crate::R<TVE_LEVEL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TVE_LEVEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TVE_LEVEL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TVE_LEVEL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `tve_level` writer"]
pub struct W(crate::W<TVE_LEVEL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TVE_LEVEL_SPEC>;
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
impl From<crate::W<TVE_LEVEL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TVE_LEVEL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `black_level` reader - Specify the black level setting. This is 10 bits unsigned integer. Allowed range is from 240 to 1023."]
pub type BLACK_LEVEL_R = crate::FieldReader<u16, u16>;
#[doc = "Field `black_level` writer - Specify the black level setting. This is 10 bits unsigned integer. Allowed range is from 240 to 1023."]
pub type BLACK_LEVEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TVE_LEVEL_SPEC, u16, u16, 10, O>;
#[doc = "Field `blank_level` reader - Specify the blank level setting for active lines. This is 10 bits unsigned integer. Allowed range is from 0 to 1023."]
pub type BLANK_LEVEL_R = crate::FieldReader<u16, u16>;
#[doc = "Field `blank_level` writer - Specify the blank level setting for active lines. This is 10 bits unsigned integer. Allowed range is from 0 to 1023."]
pub type BLANK_LEVEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TVE_LEVEL_SPEC, u16, u16, 10, O>;
impl R {
    #[doc = "Bits 0:9 - Specify the black level setting. This is 10 bits unsigned integer. Allowed range is from 240 to 1023."]
    #[inline(always)]
    pub fn black_level(&self) -> BLACK_LEVEL_R {
        BLACK_LEVEL_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 16:25 - Specify the blank level setting for active lines. This is 10 bits unsigned integer. Allowed range is from 0 to 1023."]
    #[inline(always)]
    pub fn blank_level(&self) -> BLANK_LEVEL_R {
        BLANK_LEVEL_R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - Specify the black level setting. This is 10 bits unsigned integer. Allowed range is from 240 to 1023."]
    #[inline(always)]
    #[must_use]
    pub fn black_level(&mut self) -> BLACK_LEVEL_W<0> {
        BLACK_LEVEL_W::new(self)
    }
    #[doc = "Bits 16:25 - Specify the blank level setting for active lines. This is 10 bits unsigned integer. Allowed range is from 0 to 1023."]
    #[inline(always)]
    #[must_use]
    pub fn blank_level(&mut self) -> BLANK_LEVEL_W<16> {
        BLANK_LEVEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TV Encoder Level Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tve_level](index.html) module"]
pub struct TVE_LEVEL_SPEC;
impl crate::RegisterSpec for TVE_LEVEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tve_level::R](R) reader structure"]
impl crate::Readable for TVE_LEVEL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tve_level::W](W) writer structure"]
impl crate::Writable for TVE_LEVEL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets tve_level to value 0x00f0_011a"]
impl crate::Resettable for TVE_LEVEL_SPEC {
    const RESET_VALUE: Self::Ux = 0x00f0_011a;
}
