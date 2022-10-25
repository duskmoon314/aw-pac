#[doc = "Register `ths_ctrl` reader"]
pub struct R(crate::R<THS_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<THS_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<THS_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<THS_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ths_ctrl` writer"]
pub struct W(crate::W<THS_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<THS_CTRL_SPEC>;
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
impl From<crate::W<THS_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<THS_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `tacq` reader - ADC acquire time\n\nCLK_IN/(n + 1)\n\nThe default value is 2 us."]
pub type TACQ_R = crate::FieldReader<u16, u16>;
#[doc = "Field `tacq` writer - ADC acquire time\n\nCLK_IN/(n + 1)\n\nThe default value is 2 us."]
pub type TACQ_W<'a, const O: u8> = crate::FieldWriter<'a, u32, THS_CTRL_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 16:31 - ADC acquire time\n\nCLK_IN/(n + 1)\n\nThe default value is 2 us."]
    #[inline(always)]
    pub fn tacq(&self) -> TACQ_R {
        TACQ_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:31 - ADC acquire time\n\nCLK_IN/(n + 1)\n\nThe default value is 2 us."]
    #[inline(always)]
    #[must_use]
    pub fn tacq(&mut self) -> TACQ_W<16> {
        TACQ_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "THS Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ths_ctrl](index.html) module"]
pub struct THS_CTRL_SPEC;
impl crate::RegisterSpec for THS_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ths_ctrl::R](R) reader structure"]
impl crate::Readable for THS_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ths_ctrl::W](W) writer structure"]
impl crate::Writable for THS_CTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ths_ctrl to value 0x01df_002f"]
impl crate::Resettable for THS_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0x01df_002f;
}
