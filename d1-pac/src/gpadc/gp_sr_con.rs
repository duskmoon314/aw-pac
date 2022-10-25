#[doc = "Register `gp_sr_con` reader"]
pub struct R(crate::R<GP_SR_CON_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GP_SR_CON_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GP_SR_CON_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GP_SR_CON_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `gp_sr_con` writer"]
pub struct W(crate::W<GP_SR_CON_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GP_SR_CON_SPEC>;
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
impl From<crate::W<GP_SR_CON_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GP_SR_CON_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `tacq` reader - ADC acquire time\n\n(n+1)/CLK_IN\n\nDefault value: 2 us"]
pub type TACQ_R = crate::FieldReader<u16, u16>;
#[doc = "Field `tacq` writer - ADC acquire time\n\n(n+1)/CLK_IN\n\nDefault value: 2 us"]
pub type TACQ_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GP_SR_CON_SPEC, u16, u16, 16, O>;
#[doc = "Field `fs_div` reader - ADC sample frequency divider\n\nCLK_IN/(n+1)\n\nDefault value: 50K"]
pub type FS_DIV_R = crate::FieldReader<u16, u16>;
#[doc = "Field `fs_div` writer - ADC sample frequency divider\n\nCLK_IN/(n+1)\n\nDefault value: 50K"]
pub type FS_DIV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GP_SR_CON_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - ADC acquire time\n\n(n+1)/CLK_IN\n\nDefault value: 2 us"]
    #[inline(always)]
    pub fn tacq(&self) -> TACQ_R {
        TACQ_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - ADC sample frequency divider\n\nCLK_IN/(n+1)\n\nDefault value: 50K"]
    #[inline(always)]
    pub fn fs_div(&self) -> FS_DIV_R {
        FS_DIV_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - ADC acquire time\n\n(n+1)/CLK_IN\n\nDefault value: 2 us"]
    #[inline(always)]
    #[must_use]
    pub fn tacq(&mut self) -> TACQ_W<0> {
        TACQ_W::new(self)
    }
    #[doc = "Bits 16:31 - ADC sample frequency divider\n\nCLK_IN/(n+1)\n\nDefault value: 50K"]
    #[inline(always)]
    #[must_use]
    pub fn fs_div(&mut self) -> FS_DIV_W<16> {
        FS_DIV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPADC Sample Rate Configure Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gp_sr_con](index.html) module"]
pub struct GP_SR_CON_SPEC;
impl crate::RegisterSpec for GP_SR_CON_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gp_sr_con::R](R) reader structure"]
impl crate::Readable for GP_SR_CON_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gp_sr_con::W](W) writer structure"]
impl crate::Writable for GP_SR_CON_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets gp_sr_con to value 0x01df_002f"]
impl crate::Resettable for GP_SR_CON_SPEC {
    const RESET_VALUE: Self::Ux = 0x01df_002f;
}
