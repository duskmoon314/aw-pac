#[doc = "Register `csic_prs_ncsic_rx_signal0_dly_adj` reader"]
pub type R = crate::R<CSIC_PRS_NCSIC_RX_SIGNAL0_DLY_ADJ_SPEC>;
#[doc = "Register `csic_prs_ncsic_rx_signal0_dly_adj` writer"]
pub type W = crate::W<CSIC_PRS_NCSIC_RX_SIGNAL0_DLY_ADJ_SPEC>;
#[doc = "Field `pclk_dly` reader - Pclk_dly 32 Step for adjust, 1 step = 0.2 ns"]
pub type PCLK_DLY_R = crate::FieldReader;
#[doc = "Field `pclk_dly` writer - Pclk_dly 32 Step for adjust, 1 step = 0.2 ns"]
pub type PCLK_DLY_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `hsync_dly` reader - Hsync_dly 32 Step for adjust, 1 step = 0.2 ns"]
pub type HSYNC_DLY_R = crate::FieldReader;
#[doc = "Field `hsync_dly` writer - Hsync_dly 32 Step for adjust, 1 step = 0.2 ns"]
pub type HSYNC_DLY_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `vsync_dly` reader - Vsync_dly 32 Step for adjust, 1 step = 0.2 ns"]
pub type VSYNC_DLY_R = crate::FieldReader;
#[doc = "Field `vsync_dly` writer - Vsync_dly 32 Step for adjust, 1 step = 0.2 ns"]
pub type VSYNC_DLY_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `filed_dly` reader - Filed_dly 32 Step for adjust, 1 step = 0.2 ns"]
pub type FILED_DLY_R = crate::FieldReader;
#[doc = "Field `filed_dly` writer - Filed_dly 32 Step for adjust, 1 step = 0.2 ns"]
pub type FILED_DLY_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - Pclk_dly 32 Step for adjust, 1 step = 0.2 ns"]
    #[inline(always)]
    pub fn pclk_dly(&self) -> PCLK_DLY_R {
        PCLK_DLY_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - Hsync_dly 32 Step for adjust, 1 step = 0.2 ns"]
    #[inline(always)]
    pub fn hsync_dly(&self) -> HSYNC_DLY_R {
        HSYNC_DLY_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - Vsync_dly 32 Step for adjust, 1 step = 0.2 ns"]
    #[inline(always)]
    pub fn vsync_dly(&self) -> VSYNC_DLY_R {
        VSYNC_DLY_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:28 - Filed_dly 32 Step for adjust, 1 step = 0.2 ns"]
    #[inline(always)]
    pub fn filed_dly(&self) -> FILED_DLY_R {
        FILED_DLY_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Pclk_dly 32 Step for adjust, 1 step = 0.2 ns"]
    #[inline(always)]
    #[must_use]
    pub fn pclk_dly(&mut self) -> PCLK_DLY_W<CSIC_PRS_NCSIC_RX_SIGNAL0_DLY_ADJ_SPEC> {
        PCLK_DLY_W::new(self, 0)
    }
    #[doc = "Bits 8:12 - Hsync_dly 32 Step for adjust, 1 step = 0.2 ns"]
    #[inline(always)]
    #[must_use]
    pub fn hsync_dly(&mut self) -> HSYNC_DLY_W<CSIC_PRS_NCSIC_RX_SIGNAL0_DLY_ADJ_SPEC> {
        HSYNC_DLY_W::new(self, 8)
    }
    #[doc = "Bits 16:20 - Vsync_dly 32 Step for adjust, 1 step = 0.2 ns"]
    #[inline(always)]
    #[must_use]
    pub fn vsync_dly(&mut self) -> VSYNC_DLY_W<CSIC_PRS_NCSIC_RX_SIGNAL0_DLY_ADJ_SPEC> {
        VSYNC_DLY_W::new(self, 16)
    }
    #[doc = "Bits 24:28 - Filed_dly 32 Step for adjust, 1 step = 0.2 ns"]
    #[inline(always)]
    #[must_use]
    pub fn filed_dly(&mut self) -> FILED_DLY_W<CSIC_PRS_NCSIC_RX_SIGNAL0_DLY_ADJ_SPEC> {
        FILED_DLY_W::new(self, 24)
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
#[doc = "CSIC Parser NCSIC RX Signal0 Delay Adjust Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csic_prs_ncsic_rx_signal0_dly_adj::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csic_prs_ncsic_rx_signal0_dly_adj::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CSIC_PRS_NCSIC_RX_SIGNAL0_DLY_ADJ_SPEC;
impl crate::RegisterSpec for CSIC_PRS_NCSIC_RX_SIGNAL0_DLY_ADJ_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csic_prs_ncsic_rx_signal0_dly_adj::R`](R) reader structure"]
impl crate::Readable for CSIC_PRS_NCSIC_RX_SIGNAL0_DLY_ADJ_SPEC {}
#[doc = "`write(|w| ..)` method takes [`csic_prs_ncsic_rx_signal0_dly_adj::W`](W) writer structure"]
impl crate::Writable for CSIC_PRS_NCSIC_RX_SIGNAL0_DLY_ADJ_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets csic_prs_ncsic_rx_signal0_dly_adj to value 0"]
impl crate::Resettable for CSIC_PRS_NCSIC_RX_SIGNAL0_DLY_ADJ_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
