#[doc = "Register `tv_basic3` reader"]
pub struct R(crate::R<TV_BASIC3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TV_BASIC3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TV_BASIC3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TV_BASIC3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `tv_basic3` writer"]
pub struct W(crate::W<TV_BASIC3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TV_BASIC3_SPEC>;
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
impl From<crate::W<TV_BASIC3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TV_BASIC3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `h_bp` reader - Horizontal back porch\n\nThbp = (HBP +1) * Thdclk"]
pub type H_BP_R = crate::FieldReader<u16, u16>;
#[doc = "Field `h_bp` writer - Horizontal back porch\n\nThbp = (HBP +1) * Thdclk"]
pub type H_BP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TV_BASIC3_SPEC, u16, u16, 12, O>;
#[doc = "Field `h_t` reader - Horizontal total time\n\nThcycle = (HT+1) * Thdclk"]
pub type H_T_R = crate::FieldReader<u16, u16>;
#[doc = "Field `h_t` writer - Horizontal total time\n\nThcycle = (HT+1) * Thdclk"]
pub type H_T_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TV_BASIC3_SPEC, u16, u16, 13, O>;
impl R {
    #[doc = "Bits 0:11 - Horizontal back porch\n\nThbp = (HBP +1) * Thdclk"]
    #[inline(always)]
    pub fn h_bp(&self) -> H_BP_R {
        H_BP_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:28 - Horizontal total time\n\nThcycle = (HT+1) * Thdclk"]
    #[inline(always)]
    pub fn h_t(&self) -> H_T_R {
        H_T_R::new(((self.bits >> 16) & 0x1fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Horizontal back porch\n\nThbp = (HBP +1) * Thdclk"]
    #[inline(always)]
    #[must_use]
    pub fn h_bp(&mut self) -> H_BP_W<0> {
        H_BP_W::new(self)
    }
    #[doc = "Bits 16:28 - Horizontal total time\n\nThcycle = (HT+1) * Thdclk"]
    #[inline(always)]
    #[must_use]
    pub fn h_t(&mut self) -> H_T_W<16> {
        H_T_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TV Basic Timing Register3\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tv_basic3](index.html) module"]
pub struct TV_BASIC3_SPEC;
impl crate::RegisterSpec for TV_BASIC3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tv_basic3::R](R) reader structure"]
impl crate::Readable for TV_BASIC3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tv_basic3::W](W) writer structure"]
impl crate::Writable for TV_BASIC3_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets tv_basic3 to value 0"]
impl crate::Resettable for TV_BASIC3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
