#[doc = "Register `GP_SR_CON` reader"]
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
#[doc = "Register `GP_SR_CON` writer"]
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
#[doc = "Field `FS_DIV` reader - ADC sample frequency divider\n\nCLK_IN/(n+1)\n\nDefault value: 50K"]
pub type FS_DIV_R = crate::FieldReader<u16, u16>;
#[doc = "Field `FS_DIV` writer - ADC sample frequency divider\n\nCLK_IN/(n+1)\n\nDefault value: 50K"]
pub type FS_DIV_W<'a> = crate::FieldWriter<'a, u32, GP_SR_CON_SPEC, u16, u16, 16, 16>;
#[doc = "Field `TACQ` reader - ADC acquire time\n\n(n+1)/CLK_IN\n\nDefault value: 2 us"]
pub type TACQ_R = crate::FieldReader<u16, u16>;
#[doc = "Field `TACQ` writer - ADC acquire time\n\n(n+1)/CLK_IN\n\nDefault value: 2 us"]
pub type TACQ_W<'a> = crate::FieldWriter<'a, u32, GP_SR_CON_SPEC, u16, u16, 16, 0>;
impl R {
    #[doc = "Bits 16:31 - ADC sample frequency divider\n\nCLK_IN/(n+1)\n\nDefault value: 50K"]
    #[inline(always)]
    pub fn fs_div(&self) -> FS_DIV_R {
        FS_DIV_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
    #[doc = "Bits 0:15 - ADC acquire time\n\n(n+1)/CLK_IN\n\nDefault value: 2 us"]
    #[inline(always)]
    pub fn tacq(&self) -> TACQ_R {
        TACQ_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:31 - ADC sample frequency divider\n\nCLK_IN/(n+1)\n\nDefault value: 50K"]
    #[inline(always)]
    pub fn fs_div(&mut self) -> FS_DIV_W {
        FS_DIV_W::new(self)
    }
    #[doc = "Bits 0:15 - ADC acquire time\n\n(n+1)/CLK_IN\n\nDefault value: 2 us"]
    #[inline(always)]
    pub fn tacq(&mut self) -> TACQ_W {
        TACQ_W::new(self)
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
}
#[doc = "`reset()` method sets GP_SR_CON to value 0x01df_002f"]
impl crate::Resettable for GP_SR_CON_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x01df_002f
    }
}
