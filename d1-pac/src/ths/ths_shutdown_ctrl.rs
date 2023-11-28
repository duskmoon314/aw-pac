#[doc = "Register `ths_shutdown_ctrl` reader"]
pub type R = crate::R<THS_SHUTDOWN_CTRL_SPEC>;
#[doc = "Register `ths_shutdown_ctrl` writer"]
pub type W = crate::W<THS_SHUTDOWN_CTRL_SPEC>;
#[doc = "Field `shut_t_hot` reader - Thermal sensor shutdown threshold for hot temperature"]
pub type SHUT_T_HOT_R = crate::FieldReader<u16>;
#[doc = "Field `shut_t_hot` writer - Thermal sensor shutdown threshold for hot temperature"]
pub type SHUT_T_HOT_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - Thermal sensor shutdown threshold for hot temperature"]
    #[inline(always)]
    pub fn shut_t_hot(&self) -> SHUT_T_HOT_R {
        SHUT_T_HOT_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Thermal sensor shutdown threshold for hot temperature"]
    #[inline(always)]
    #[must_use]
    pub fn shut_t_hot(&mut self) -> SHUT_T_HOT_W<THS_SHUTDOWN_CTRL_SPEC> {
        SHUT_T_HOT_W::new(self, 0)
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
#[doc = "THS Shutdown Threshold Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ths_shutdown_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ths_shutdown_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct THS_SHUTDOWN_CTRL_SPEC;
impl crate::RegisterSpec for THS_SHUTDOWN_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ths_shutdown_ctrl::R`](R) reader structure"]
impl crate::Readable for THS_SHUTDOWN_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ths_shutdown_ctrl::W`](W) writer structure"]
impl crate::Writable for THS_SHUTDOWN_CTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ths_shutdown_ctrl to value 0x04e9"]
impl crate::Resettable for THS_SHUTDOWN_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0x04e9;
}
