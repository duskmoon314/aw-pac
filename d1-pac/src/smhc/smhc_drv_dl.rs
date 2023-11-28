#[doc = "Register `smhc_drv_dl` reader"]
pub type R = crate::R<SMHC_DRV_DL_SPEC>;
#[doc = "Register `smhc_drv_dl` writer"]
pub type W = crate::W<SMHC_DRV_DL_SPEC>;
#[doc = "Field `cmd_drv_ph_sel` reader - Command Drive Phase Select"]
pub type CMD_DRV_PH_SEL_R = crate::BitReader;
#[doc = "Field `cmd_drv_ph_sel` writer - Command Drive Phase Select"]
pub type CMD_DRV_PH_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `dat_drv_ph_sel` reader - Data Drive Phase Select"]
pub type DAT_DRV_PH_SEL_R = crate::BitReader;
#[doc = "Field `dat_drv_ph_sel` writer - Data Drive Phase Select"]
pub type DAT_DRV_PH_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 16 - Command Drive Phase Select"]
    #[inline(always)]
    pub fn cmd_drv_ph_sel(&self) -> CMD_DRV_PH_SEL_R {
        CMD_DRV_PH_SEL_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Data Drive Phase Select"]
    #[inline(always)]
    pub fn dat_drv_ph_sel(&self) -> DAT_DRV_PH_SEL_R {
        DAT_DRV_PH_SEL_R::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 16 - Command Drive Phase Select"]
    #[inline(always)]
    #[must_use]
    pub fn cmd_drv_ph_sel(&mut self) -> CMD_DRV_PH_SEL_W<SMHC_DRV_DL_SPEC> {
        CMD_DRV_PH_SEL_W::new(self, 16)
    }
    #[doc = "Bit 17 - Data Drive Phase Select"]
    #[inline(always)]
    #[must_use]
    pub fn dat_drv_ph_sel(&mut self) -> DAT_DRV_PH_SEL_W<SMHC_DRV_DL_SPEC> {
        DAT_DRV_PH_SEL_W::new(self, 17)
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
#[doc = "Drive Delay Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`smhc_drv_dl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`smhc_drv_dl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SMHC_DRV_DL_SPEC;
impl crate::RegisterSpec for SMHC_DRV_DL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`smhc_drv_dl::R`](R) reader structure"]
impl crate::Readable for SMHC_DRV_DL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`smhc_drv_dl::W`](W) writer structure"]
impl crate::Writable for SMHC_DRV_DL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets smhc_drv_dl to value 0"]
impl crate::Resettable for SMHC_DRV_DL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
