#[doc = "Register `tv_basic5` reader"]
pub struct R(crate::R<TV_BASIC5_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TV_BASIC5_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TV_BASIC5_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TV_BASIC5_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `tv_basic5` writer"]
pub struct W(crate::W<TV_BASIC5_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TV_BASIC5_SPEC>;
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
impl From<crate::W<TV_BASIC5_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TV_BASIC5_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `v_spw` reader - Vertical Sync Pulse Width (in lines)\n\nTvspw = (VSPW+1) * Th\n\nNote: VT/2 > (VSPW+1)"]
pub type V_SPW_R = crate::FieldReader<u16, u16>;
#[doc = "Field `v_spw` writer - Vertical Sync Pulse Width (in lines)\n\nTvspw = (VSPW+1) * Th\n\nNote: VT/2 > (VSPW+1)"]
pub type V_SPW_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TV_BASIC5_SPEC, u16, u16, 10, O>;
#[doc = "Field `h_spw` reader - Horizontal Sync Pulse Width (in dclk)\n\nThspw = (HSPW+1) * Tdclk\n\nNote: HT > (HSPW+1)"]
pub type H_SPW_R = crate::FieldReader<u16, u16>;
#[doc = "Field `h_spw` writer - Horizontal Sync Pulse Width (in dclk)\n\nThspw = (HSPW+1) * Tdclk\n\nNote: HT > (HSPW+1)"]
pub type H_SPW_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TV_BASIC5_SPEC, u16, u16, 10, O>;
impl R {
    #[doc = "Bits 0:9 - Vertical Sync Pulse Width (in lines)\n\nTvspw = (VSPW+1) * Th\n\nNote: VT/2 > (VSPW+1)"]
    #[inline(always)]
    pub fn v_spw(&self) -> V_SPW_R {
        V_SPW_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 16:25 - Horizontal Sync Pulse Width (in dclk)\n\nThspw = (HSPW+1) * Tdclk\n\nNote: HT > (HSPW+1)"]
    #[inline(always)]
    pub fn h_spw(&self) -> H_SPW_R {
        H_SPW_R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - Vertical Sync Pulse Width (in lines)\n\nTvspw = (VSPW+1) * Th\n\nNote: VT/2 > (VSPW+1)"]
    #[inline(always)]
    #[must_use]
    pub fn v_spw(&mut self) -> V_SPW_W<0> {
        V_SPW_W::new(self)
    }
    #[doc = "Bits 16:25 - Horizontal Sync Pulse Width (in dclk)\n\nThspw = (HSPW+1) * Tdclk\n\nNote: HT > (HSPW+1)"]
    #[inline(always)]
    #[must_use]
    pub fn h_spw(&mut self) -> H_SPW_W<16> {
        H_SPW_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TV Basic Timing Register5\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tv_basic5](index.html) module"]
pub struct TV_BASIC5_SPEC;
impl crate::RegisterSpec for TV_BASIC5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tv_basic5::R](R) reader structure"]
impl crate::Readable for TV_BASIC5_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tv_basic5::W](W) writer structure"]
impl crate::Writable for TV_BASIC5_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets tv_basic5 to value 0"]
impl crate::Resettable for TV_BASIC5_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
