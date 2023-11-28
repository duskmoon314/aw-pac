#[doc = "Register `gp_sr_con` reader"]
pub type R = crate::R<GP_SR_CON_SPEC>;
#[doc = "Register `gp_sr_con` writer"]
pub type W = crate::W<GP_SR_CON_SPEC>;
#[doc = "Field `tacq` reader - ADC acquire time\n\n(n+1)/CLK_IN\n\nDefault value: 2 us"]
pub type TACQ_R = crate::FieldReader<u16>;
#[doc = "Field `tacq` writer - ADC acquire time\n\n(n+1)/CLK_IN\n\nDefault value: 2 us"]
pub type TACQ_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `fs_div` reader - ADC sample frequency divider\n\nCLK_IN/(n+1)\n\nDefault value: 50K"]
pub type FS_DIV_R = crate::FieldReader<u16>;
#[doc = "Field `fs_div` writer - ADC sample frequency divider\n\nCLK_IN/(n+1)\n\nDefault value: 50K"]
pub type FS_DIV_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
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
    pub fn tacq(&mut self) -> TACQ_W<GP_SR_CON_SPEC> {
        TACQ_W::new(self, 0)
    }
    #[doc = "Bits 16:31 - ADC sample frequency divider\n\nCLK_IN/(n+1)\n\nDefault value: 50K"]
    #[inline(always)]
    #[must_use]
    pub fn fs_div(&mut self) -> FS_DIV_W<GP_SR_CON_SPEC> {
        FS_DIV_W::new(self, 16)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "GPADC Sample Rate Configure Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gp_sr_con::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gp_sr_con::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GP_SR_CON_SPEC;
impl crate::RegisterSpec for GP_SR_CON_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gp_sr_con::R`](R) reader structure"]
impl crate::Readable for GP_SR_CON_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gp_sr_con::W`](W) writer structure"]
impl crate::Writable for GP_SR_CON_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets gp_sr_con to value 0x01df_002f"]
impl crate::Resettable for GP_SR_CON_SPEC {
    const RESET_VALUE: Self::Ux = 0x01df_002f;
}
