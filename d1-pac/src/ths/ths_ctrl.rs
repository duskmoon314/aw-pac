#[doc = "Register `ths_ctrl` reader"]
pub type R = crate::R<THS_CTRL_SPEC>;
#[doc = "Register `ths_ctrl` writer"]
pub type W = crate::W<THS_CTRL_SPEC>;
#[doc = "Field `tacq` reader - ADC acquire time\n\nCLK_IN/(n + 1)\n\nThe default value is 2 us."]
pub type TACQ_R = crate::FieldReader<u16>;
#[doc = "Field `tacq` writer - ADC acquire time\n\nCLK_IN/(n + 1)\n\nThe default value is 2 us."]
pub type TACQ_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
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
    pub fn tacq(&mut self) -> TACQ_W<THS_CTRL_SPEC> {
        TACQ_W::new(self, 16)
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
#[doc = "THS Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ths_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ths_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct THS_CTRL_SPEC;
impl crate::RegisterSpec for THS_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ths_ctrl::R`](R) reader structure"]
impl crate::Readable for THS_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ths_ctrl::W`](W) writer structure"]
impl crate::Writable for THS_CTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ths_ctrl to value 0x01df_002f"]
impl crate::Resettable for THS_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0x01df_002f;
}
