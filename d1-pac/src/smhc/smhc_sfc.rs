#[doc = "Register `smhc_sfc` reader"]
pub type R = crate::R<SMHC_SFC_SPEC>;
#[doc = "Register `smhc_sfc` writer"]
pub type W = crate::W<SMHC_SFC_SPEC>;
#[doc = "Field `bypass_en` reader - Bypass enable"]
pub type BYPASS_EN_R = crate::BitReader;
#[doc = "Field `bypass_en` writer - Bypass enable"]
pub type BYPASS_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `stop_clk_ctrl` reader - Stop Clock Control"]
pub type STOP_CLK_CTRL_R = crate::FieldReader;
#[doc = "Field `stop_clk_ctrl` writer - Stop Clock Control"]
pub type STOP_CLK_CTRL_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bit 0 - Bypass enable"]
    #[inline(always)]
    pub fn bypass_en(&self) -> BYPASS_EN_R {
        BYPASS_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:4 - Stop Clock Control"]
    #[inline(always)]
    pub fn stop_clk_ctrl(&self) -> STOP_CLK_CTRL_R {
        STOP_CLK_CTRL_R::new(((self.bits >> 1) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Bypass enable"]
    #[inline(always)]
    #[must_use]
    pub fn bypass_en(&mut self) -> BYPASS_EN_W<SMHC_SFC_SPEC> {
        BYPASS_EN_W::new(self, 0)
    }
    #[doc = "Bits 1:4 - Stop Clock Control"]
    #[inline(always)]
    #[must_use]
    pub fn stop_clk_ctrl(&mut self) -> STOP_CLK_CTRL_W<SMHC_SFC_SPEC> {
        STOP_CLK_CTRL_W::new(self, 1)
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
#[doc = "Sample FIFO Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`smhc_sfc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`smhc_sfc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SMHC_SFC_SPEC;
impl crate::RegisterSpec for SMHC_SFC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`smhc_sfc::R`](R) reader structure"]
impl crate::Readable for SMHC_SFC_SPEC {}
#[doc = "`write(|w| ..)` method takes [`smhc_sfc::W`](W) writer structure"]
impl crate::Writable for SMHC_SFC_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets smhc_sfc to value 0"]
impl crate::Resettable for SMHC_SFC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
