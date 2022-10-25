#[doc = "Register `tve_white_level` reader"]
pub struct R(crate::R<TVE_WHITE_LEVEL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TVE_WHITE_LEVEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TVE_WHITE_LEVEL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TVE_WHITE_LEVEL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `tve_white_level` writer"]
pub struct W(crate::W<TVE_WHITE_LEVEL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TVE_WHITE_LEVEL_SPEC>;
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
impl From<crate::W<TVE_WHITE_LEVEL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TVE_WHITE_LEVEL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `white_level` reader - Specify the white level setting. 10-bit unsigned integer. Allowed range is from black_level+1 or vbi_blank_level +1 (whichever is greater) to 1023."]
pub type WHITE_LEVEL_R = crate::FieldReader<u16, u16>;
#[doc = "Field `white_level` writer - Specify the white level setting. 10-bit unsigned integer. Allowed range is from black_level+1 or vbi_blank_level +1 (whichever is greater) to 1023."]
pub type WHITE_LEVEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TVE_WHITE_LEVEL_SPEC, u16, u16, 10, O>;
#[doc = "Field `hd_sync_breezeway_level` reader - Specify the breezeway level setting. 10-bit unsigned integer. Allowed range is from 0 to 1023."]
pub type HD_SYNC_BREEZEWAY_LEVEL_R = crate::FieldReader<u16, u16>;
#[doc = "Field `hd_sync_breezeway_level` writer - Specify the breezeway level setting. 10-bit unsigned integer. Allowed range is from 0 to 1023."]
pub type HD_SYNC_BREEZEWAY_LEVEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TVE_WHITE_LEVEL_SPEC, u16, u16, 10, O>;
impl R {
    #[doc = "Bits 0:9 - Specify the white level setting. 10-bit unsigned integer. Allowed range is from black_level+1 or vbi_blank_level +1 (whichever is greater) to 1023."]
    #[inline(always)]
    pub fn white_level(&self) -> WHITE_LEVEL_R {
        WHITE_LEVEL_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 16:25 - Specify the breezeway level setting. 10-bit unsigned integer. Allowed range is from 0 to 1023."]
    #[inline(always)]
    pub fn hd_sync_breezeway_level(&self) -> HD_SYNC_BREEZEWAY_LEVEL_R {
        HD_SYNC_BREEZEWAY_LEVEL_R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - Specify the white level setting. 10-bit unsigned integer. Allowed range is from black_level+1 or vbi_blank_level +1 (whichever is greater) to 1023."]
    #[inline(always)]
    #[must_use]
    pub fn white_level(&mut self) -> WHITE_LEVEL_W<0> {
        WHITE_LEVEL_W::new(self)
    }
    #[doc = "Bits 16:25 - Specify the breezeway level setting. 10-bit unsigned integer. Allowed range is from 0 to 1023."]
    #[inline(always)]
    #[must_use]
    pub fn hd_sync_breezeway_level(&mut self) -> HD_SYNC_BREEZEWAY_LEVEL_W<16> {
        HD_SYNC_BREEZEWAY_LEVEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TV Encoder White Level Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tve_white_level](index.html) module"]
pub struct TVE_WHITE_LEVEL_SPEC;
impl crate::RegisterSpec for TVE_WHITE_LEVEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tve_white_level::R](R) reader structure"]
impl crate::Readable for TVE_WHITE_LEVEL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tve_white_level::W](W) writer structure"]
impl crate::Writable for TVE_WHITE_LEVEL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets tve_white_level to value 0x01e8_0320"]
impl crate::Resettable for TVE_WHITE_LEVEL_SPEC {
    const RESET_VALUE: Self::Ux = 0x01e8_0320;
}
